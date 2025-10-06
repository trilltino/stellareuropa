use crate::database::connection::DbPool;
use crate::database::repositories::{EventRepository, UserRepository, CreateEventParams, UpdateKpiParams, UpdatePostEventParams};
use axum::{
    extract::{Json, State, Query, Path},
    http::StatusCode,
};
use tracing::{info, error, debug};
use shared::dto::{EventRequest, EventResponse, EventListResponse, EventType, StrategicFocusArea, KPIEstimates, PostEventReport, EventCosts, EventEvaluation};
use crate::database::models::Event;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct ListEventsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn create_event_response(event: &Event, organizer_username: &str) -> EventResponse {
    let event_type = match event.event_type.as_str() {
        "Workshop" => EventType::Workshop,
        "Meetup" => EventType::Meetup,
        "Conference" => EventType::Conference,
        "Hackathon" => EventType::Hackathon,
        "Community" => EventType::Community,
        _ => EventType::Community, // default fallback
    };

    let strategic_focus_areas = event.strategic_focus_areas.as_ref()
        .map(|areas| areas.iter().filter_map(|area| {
            match area.as_str() {
                "Community Participation" => Some(StrategicFocusArea::CommunityParticipation),
                "On-Chain Activity" => Some(StrategicFocusArea::OnChainActivity),
                "SCF Referrals" => Some(StrategicFocusArea::SCFReferrals),
                "Ecosystem Collaboration" => Some(StrategicFocusArea::EcosystemCollaboration),
                "Developer Growth" => Some(StrategicFocusArea::DeveloperGrowth),
                _ => None,
            }
        }).collect())
        .unwrap_or_default();

    let kpi_estimates = KPIEstimates {
        monthly_active_ambassadors: event.monthly_active_ambassadors.map(|v| v as u32),
        monthly_active_accounts: event.monthly_active_accounts.map(|v| v as u32),
        scf_referrals: event.scf_referrals.map(|v| v as u32),
        content_produced: event.content_produced.map(|v| v as u32),
        expected_attendance: event.expected_attendance.map(|v| v as u32),
        social_growth_target: event.social_growth_target.map(|v| v as u32),
    };

    let post_event_report = if event.actual_attendance.is_some() || event.social_traction.is_some()
                              || event.content_created.is_some() || event.active_developers_summary.is_some()
                              || event.qualitative_feedback.is_some() {
        Some(PostEventReport {
            actual_attendance: event.actual_attendance.map(|v| v as u32),
            social_traction: event.social_traction.clone(),
            content_created: event.content_created.clone(),
            active_developers_summary: event.active_developers_summary.clone(),
            qualitative_feedback: event.qualitative_feedback.clone(),
        })
    } else {
        None
    };

    let event_costs = if event.sponsorship_cost.is_some() || event.travel_cost.is_some()
                       || event.awards_cost.is_some() || event.other_costs.is_some() {
        Some(EventCosts {
            sponsorship_cost: event.sponsorship_cost.as_ref().and_then(|bd| bd.to_string().parse::<f64>().ok()),
            travel_cost: event.travel_cost.as_ref().and_then(|bd| bd.to_string().parse::<f64>().ok()),
            awards_cost: event.awards_cost.as_ref().and_then(|bd| bd.to_string().parse::<f64>().ok()),
            other_costs: event.other_costs.as_ref().and_then(|bd| bd.to_string().parse::<f64>().ok()),
        })
    } else {
        None
    };

    let event_evaluation = if event.project_submissions.is_some() || event.promotion_reach.is_some()
                            || event.developer_integration.is_some() || event.host_summary.is_some() {
        Some(EventEvaluation {
            project_submissions: event.project_submissions.map(|v| v as u32),
            promotion_reach: event.promotion_reach.clone(),
            developer_integration: event.developer_integration.clone(),
            host_summary: event.host_summary.clone(),
        })
    } else {
        None
    };

    EventResponse {
        id: event.id.to_string(),
        title: event.title.clone(),
        description: event.description.clone(),
        event_type,
        date: event.date.to_string(),
        location: event.location.clone(),
        max_participants: event.max_participants.map(|p| p as u32),
        registration_required: event.registration_required,
        contact_email: event.contact_email.clone(),
        external_link: event.external_link.clone(),
        organizer: organizer_username.to_string(),
        created_at: event.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
        strategic_focus_areas,
        kpi_estimates,
        target_audience: event.target_audience.clone(),
        quarterly_goals: event.quarterly_goals.clone(),
        strategic_purpose: event.strategic_purpose.clone(),
        success_metrics: event.success_metrics.clone(),
        post_event_report,
        event_costs,
        event_evaluation,
        event_images: event.event_images.clone(),
    }
}

pub async fn create_event(
    current_user: crate::extractors::current_user::CurrentUser,
    State(pool): State<DbPool>,
    Json(req): Json<EventRequest>,
) -> (StatusCode, Json<String>) {
    let organizer_id = current_user.0.user_id;

    info!(
        "Received event creation request from user {}: title={}",
        organizer_id, req.title
    );

    debug!("[EVENT] NEW EVENT CREATION REQUEST");
    debug!("   User ID: {}", organizer_id);
    debug!("   Username: {}", current_user.0.username);
    debug!("   Title: {}", req.title);
    debug!("   Type: {:?}", req.event_type);
    debug!("   Date: {}", req.date);
    debug!("   Location: {}", req.location);
    debug!("   Contact: {}", req.contact_email);
    debug!("   Strategic Focus Areas: {:?}", req.strategic_focus_areas);
    debug!("   Target Audience: {}", req.target_audience);
    debug!("   Strategic Purpose: {}", req.strategic_purpose);
    debug!("   KPI Estimates:");
    debug!("     - Monthly Active Ambassadors: {:?}", req.kpi_estimates.monthly_active_ambassadors);
    debug!("     - Monthly Active Accounts: {:?}", req.kpi_estimates.monthly_active_accounts);
    debug!("     - SCF Referrals: {:?}", req.kpi_estimates.scf_referrals);
    debug!("   ────────────────────────────────────");

    // Parse the date string - handle different formats
    let date = if req.date.contains('T') && !req.date.ends_with('Z') && !req.date.contains('+') {
        // HTML datetime-local format: "2025-09-05T08:33"
        match chrono::NaiveDateTime::parse_from_str(&req.date, "%Y-%m-%dT%H:%M") {
            Ok(naive_dt) => naive_dt.and_utc(), // Convert to UTC
            Err(_) => {
                // Try with seconds: "2025-09-05T08:33:00"
                match chrono::NaiveDateTime::parse_from_str(&req.date, "%Y-%m-%dT%H:%M:%S") {
                    Ok(naive_dt) => naive_dt.and_utc(),
                    Err(e) => {
                        error!("Invalid date format: {:?}", e);
                        return (StatusCode::BAD_REQUEST, Json("Invalid date format".to_string()));
                    }
                }
            }
        }
    } else {
        // Try parsing as full UTC datetime
        match req.date.parse::<DateTime<Utc>>() {
            Ok(d) => d,
            Err(e) => {
                error!("Invalid date format: {:?}", e);
                return (StatusCode::BAD_REQUEST, Json("Invalid date format".to_string()));
            }
        }
    };

    let event_type_str = req.event_type.to_string();

    let strategic_focus_areas_strings: Vec<String> = req.strategic_focus_areas.iter()
        .map(|area| area.to_string())
        .collect();

    // Build parameter struct instead of passing 36 individual parameters
    let params = CreateEventParams {
        title: &req.title,
        description: &req.description,
        event_type: &event_type_str,
        date,
        location: &req.location,
        max_participants: req.max_participants.map(|p| p as i32),
        registration_required: req.registration_required,
        contact_email: &req.contact_email,
        external_link: req.external_link.as_deref(),
        organizer_id,
        strategic_focus_areas: Some(&strategic_focus_areas_strings),
        target_audience: &req.target_audience,
        quarterly_goals: &req.quarterly_goals,
        strategic_purpose: &req.strategic_purpose,
        success_metrics: req.success_metrics.as_deref(),
        monthly_active_ambassadors: req.kpi_estimates.monthly_active_ambassadors.map(|v| v as i32),
        monthly_active_accounts: req.kpi_estimates.monthly_active_accounts.map(|v| v as i32),
        scf_referrals: req.kpi_estimates.scf_referrals.map(|v| v as i32),
        content_produced: req.kpi_estimates.content_produced.map(|v| v as i32),
        expected_attendance: req.kpi_estimates.expected_attendance.map(|v| v as i32),
        social_growth_target: req.kpi_estimates.social_growth_target.map(|v| v as i32),
        actual_attendance: req.post_event_report.as_ref().and_then(|r| r.actual_attendance).map(|v| v as i32),
        social_traction: req.post_event_report.as_ref().and_then(|r| r.social_traction.as_deref()),
        content_created: req.post_event_report.as_ref().and_then(|r| r.content_created.as_deref()),
        active_developers_summary: req.post_event_report.as_ref().and_then(|r| r.active_developers_summary.as_deref()),
        qualitative_feedback: req.post_event_report.as_ref().and_then(|r| r.qualitative_feedback.as_deref()),
        sponsorship_cost: req.event_costs.as_ref().and_then(|c| c.sponsorship_cost).and_then(|v| BigDecimal::from_str(&v.to_string()).ok()),
        travel_cost: req.event_costs.as_ref().and_then(|c| c.travel_cost).and_then(|v| BigDecimal::from_str(&v.to_string()).ok()),
        awards_cost: req.event_costs.as_ref().and_then(|c| c.awards_cost).and_then(|v| BigDecimal::from_str(&v.to_string()).ok()),
        other_costs: req.event_costs.as_ref().and_then(|c| c.other_costs).and_then(|v| BigDecimal::from_str(&v.to_string()).ok()),
        project_submissions: req.event_evaluation.as_ref().and_then(|e| e.project_submissions).map(|v| v as i32),
        promotion_reach: req.event_evaluation.as_ref().and_then(|e| e.promotion_reach.as_deref()),
        developer_integration: req.event_evaluation.as_ref().and_then(|e| e.developer_integration.as_deref()),
        host_summary: req.event_evaluation.as_ref().and_then(|e| e.host_summary.as_deref()),
        event_images: req.event_images.as_ref(),
    };

    match EventRepository::create_event(&pool, params).await {
        Ok(event) => {
            info!("Event created successfully: {} (ID: {})", event.title, event.id);
            debug!("Event date: {}, location: {}", event.date, event.location);
            (StatusCode::CREATED, Json("Event created successfully!".to_string()))
        }
        Err(e) => {
            error!("Failed to create event: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to create event: {e}")))
        }
    }
}

pub async fn list_events(
    State(pool): State<DbPool>,
    Query(params): Query<ListEventsQuery>,
) -> (StatusCode, Json<EventListResponse>) {
    debug!("Events list request (limit: {:?}, offset: {:?})", params.limit, params.offset);

    match EventRepository::list_events(&pool, params.limit, params.offset).await {
        Ok(events) => {
            // Performance optimization: Batch load all organizers to prevent N+1 query problem
            // Instead of 1 query for events + N queries for each organizer (N+1 total),
            // we do 1 query for events + 1 batch query for all organizers (2 total)

            // Step 1: Collect unique organizer IDs
            let organizer_ids: Vec<i32> = events
                .iter()
                .map(|event| event.organizer_id)
                .collect::<std::collections::HashSet<_>>()  // Remove duplicates
                .into_iter()
                .collect();

            // Step 2: Batch fetch all organizers in one query
            let organizers = match UserRepository::find_by_ids(&pool, &organizer_ids).await {
                Ok(users) => users,
                Err(e) => {
                    error!("Failed to batch load organizers: {:?}", e);
                    Vec::new()  // Continue with empty organizers if batch load fails
                }
            };

            // Step 3: Create HashMap for O(1) username lookups
            let organizer_map: std::collections::HashMap<i32, String> = organizers
                .into_iter()
                .map(|user| (user.id, user.username))
                .collect();

            debug!("Loaded {} unique organizers for {} events", organizer_map.len(), events.len());

            // Step 4: Map events to responses using the HashMap (no additional queries)
            let event_responses: Vec<EventResponse> = events
                .iter()
                .map(|event| {
                    let organizer_username = organizer_map
                        .get(&event.organizer_id)
                        .cloned()
                        .unwrap_or_else(|| "Unknown".to_string());

                    create_event_response(event, &organizer_username)
                })
                .collect();

            let response = EventListResponse {
                total: event_responses.len(),
                events: event_responses,
            };

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            error!("Database error listing events: {:?}", e);
            let response = EventListResponse {
                total: 0,
                events: vec![],
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn update_event_kpi(
    State(pool): State<DbPool>,
    Path(event_id): Path<i32>,
    Json(req): Json<EventRequest>,
) -> (StatusCode, Json<String>) {
    debug!("Event KPI update request: event_id={}, focus_areas={:?}", event_id, req.strategic_focus_areas);

    let strategic_focus_areas_strings: Vec<String> = req.strategic_focus_areas.iter()
        .map(|area| area.to_string())
        .collect();

    let params = UpdateKpiParams {
        strategic_focus_areas: Some(&strategic_focus_areas_strings),
        monthly_active_ambassadors: req.kpi_estimates.monthly_active_ambassadors.map(|v| v as i32),
        monthly_active_accounts: req.kpi_estimates.monthly_active_accounts.map(|v| v as i32),
        scf_referrals: req.kpi_estimates.scf_referrals.map(|v| v as i32),
        content_produced: req.kpi_estimates.content_produced.map(|v| v as i32),
        expected_attendance: req.kpi_estimates.expected_attendance.map(|v| v as i32),
        social_growth_target: req.kpi_estimates.social_growth_target.map(|v| v as i32),
        target_audience: &req.target_audience,
        quarterly_goals: &req.quarterly_goals,
        strategic_purpose: &req.strategic_purpose,
        success_metrics: req.success_metrics.as_deref(),
    };

    match EventRepository::update_event_kpi(&pool, event_id, params).await {
        Ok(event) => {
            info!("Event KPI updated successfully: {} (ID: {})", event.title, event.id);
            (StatusCode::OK, Json("Event KPI updated successfully!".to_string()))
        }
        Err(e) => {
            error!("Failed to update event KPI: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to update event KPI: {e}")))
        }
    }
}

pub async fn update_post_event_data(
    State(pool): State<DbPool>,
    Path(event_id): Path<i32>,
    Json(req): Json<EventRequest>,
) -> (StatusCode, Json<String>) {
    debug!("Post-event data update: event_id={}, has_report={}, has_costs={}, has_eval={}, images={}",
        event_id, req.post_event_report.is_some(), req.event_costs.is_some(),
        req.event_evaluation.is_some(), req.event_images.as_ref().map(|v| v.len()).unwrap_or(0));

    let params = UpdatePostEventParams {
        actual_attendance: req.post_event_report.as_ref().and_then(|r| r.actual_attendance).map(|v| v as i32),
        social_traction: req.post_event_report.as_ref().and_then(|r| r.social_traction.as_deref()),
        content_created: req.post_event_report.as_ref().and_then(|r| r.content_created.as_deref()),
        active_developers_summary: req.post_event_report.as_ref().and_then(|r| r.active_developers_summary.as_deref()),
        qualitative_feedback: req.post_event_report.as_ref().and_then(|r| r.qualitative_feedback.as_deref()),
        sponsorship_cost: req.event_costs.as_ref().and_then(|c| c.sponsorship_cost).and_then(|v| BigDecimal::from_str(&v.to_string()).ok()),
        travel_cost: req.event_costs.as_ref().and_then(|c| c.travel_cost).and_then(|v| BigDecimal::from_str(&v.to_string()).ok()),
        awards_cost: req.event_costs.as_ref().and_then(|c| c.awards_cost).and_then(|v| BigDecimal::from_str(&v.to_string()).ok()),
        other_costs: req.event_costs.as_ref().and_then(|c| c.other_costs).and_then(|v| BigDecimal::from_str(&v.to_string()).ok()),
        project_submissions: req.event_evaluation.as_ref().and_then(|e| e.project_submissions).map(|v| v as i32),
        promotion_reach: req.event_evaluation.as_ref().and_then(|e| e.promotion_reach.as_deref()),
        developer_integration: req.event_evaluation.as_ref().and_then(|e| e.developer_integration.as_deref()),
        host_summary: req.event_evaluation.as_ref().and_then(|e| e.host_summary.as_deref()),
        event_images: req.event_images.as_ref(),
    };

    match EventRepository::update_post_event_data(&pool, event_id, params).await {
        Ok(event) => {
            info!("Post-event data updated successfully: {} (ID: {})", event.title, event.id);
            (StatusCode::OK, Json("Post-event data updated successfully!".to_string()))
        }
        Err(e) => {
            error!("Failed to update post-event data: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to update post-event data: {e}")))
        }
    }
}