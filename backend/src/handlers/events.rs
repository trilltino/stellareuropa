use crate::database::connection::DbPool;
use crate::database::repositories::{EventRepository, UserRepository};
use axum::{
    extract::{Json, State, Query},
    http::StatusCode,
};
use tracing::{info, error};
use shared::dto::{EventRequest, EventResponse, EventListResponse, EventType, StrategicFocusArea, KPIEstimates};
use crate::database::models::Event;
use serde::Deserialize;
use chrono::{DateTime, Utc};

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
    }
}

pub async fn create_event(
    State(pool): State<DbPool>,
    Json(req): Json<EventRequest>,
) -> (StatusCode, Json<String>) {
    println!("🎪 NEW EVENT CREATION REQUEST");
    println!("   Title: {}", req.title);
    println!("   Type: {:?}", req.event_type);
    println!("   Date: {}", req.date);
    println!("   Location: {}", req.location);
    println!("   Contact: {}", req.contact_email);
    println!("   Strategic Focus Areas: {:?}", req.strategic_focus_areas);
    println!("   Target Audience: {}", req.target_audience);
    println!("   Strategic Purpose: {}", req.strategic_purpose);
    println!("   KPI Estimates:");
    println!("     - Monthly Active Ambassadors: {:?}", req.kpi_estimates.monthly_active_ambassadors);
    println!("     - Monthly Active Accounts: {:?}", req.kpi_estimates.monthly_active_accounts);
    println!("     - SCF Referrals: {:?}", req.kpi_estimates.scf_referrals);
    println!("   ────────────────────────────────────");

    info!("Received event creation request: title={}", req.title);

    // For now, we'll use a placeholder organizer_id of 1
    // In a real app, this would come from authentication
    let organizer_id = 1;

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

    match EventRepository::create_event(
        &pool,
        &req.title,
        &req.description,
        &event_type_str,
        date,
        &req.location,
        req.max_participants.map(|p| p as i32),
        req.registration_required,
        &req.contact_email,
        req.external_link.as_deref(),
        organizer_id,
        Some(&strategic_focus_areas_strings),
        req.kpi_estimates.monthly_active_ambassadors.map(|v| v as i32),
        req.kpi_estimates.monthly_active_accounts.map(|v| v as i32),
        req.kpi_estimates.scf_referrals.map(|v| v as i32),
        req.kpi_estimates.content_produced.map(|v| v as i32),
        req.kpi_estimates.expected_attendance.map(|v| v as i32),
        req.kpi_estimates.social_growth_target.map(|v| v as i32),
        &req.target_audience,
        &req.quarterly_goals,
        &req.strategic_purpose,
        req.success_metrics.as_deref(),
    ).await {
        Ok(event) => {
            println!("✅ EVENT CREATED SUCCESSFULLY!");
            println!("   Event ID: {}", event.id);
            println!("   Title: {}", event.title);
            println!("   Date: {}", event.date);
            println!("   🎉 Event is ready for the community!");
            println!("   ════════════════════════════════════");
            (StatusCode::CREATED, Json("Event created successfully!".to_string()))
        }
        Err(e) => {
            println!("❌ EVENT CREATION FAILED: {}", e);
            error!("Database error creating event: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to create event: {}", e)))
        }
    }
}

pub async fn list_events(
    State(pool): State<DbPool>,
    Query(params): Query<ListEventsQuery>,
) -> (StatusCode, Json<EventListResponse>) {
    println!("📋 EVENTS LIST REQUEST (limit: {:?}, offset: {:?})", params.limit, params.offset);
    info!("Received events list request");

    match EventRepository::list_events(&pool, params.limit, params.offset).await {
        Ok(events) => {
            let mut event_responses = Vec::new();

            for event in events {
                // Get organizer username
                let organizer_username = match UserRepository::find_by_id(&pool, event.organizer_id).await {
                    Ok(Some(user)) => user.username,
                    Ok(None) => "Unknown".to_string(),
                    Err(_) => "Unknown".to_string(),
                };

                event_responses.push(create_event_response(&event, &organizer_username));
            }

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