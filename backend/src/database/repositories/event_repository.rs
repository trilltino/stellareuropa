use crate::database::models::Event;
use crate::database::connection::DbPool;
use crate::database::repositories::event_params::{CreateEventParams, UpdateKpiParams, UpdatePostEventParams};
use sqlx::{Error as SqlxError};

pub struct EventRepository;

impl EventRepository {
    /// Create a new event with all associated data
    ///
    /// This uses a parameter struct to avoid the 36-parameter anti-pattern.
    pub async fn create_event(
        pool: &DbPool,
        params: CreateEventParams<'_>,
    ) -> Result<Event, SqlxError> {
        sqlx::query_as!(
            Event,
            r#"
            INSERT INTO events (title, description, event_type, date, location, max_participants, registration_required, contact_email, external_link, organizer_id,
                              strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals, content_produced, expected_attendance,
                              social_growth_target, target_audience, quarterly_goals, strategic_purpose, success_metrics,
                              actual_attendance, social_traction, content_created, active_developers_summary, qualitative_feedback,
                              sponsorship_cost, travel_cost, awards_cost, other_costs,
                              project_submissions, promotion_reach, developer_integration, host_summary, event_images, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21,
                   $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, NOW())
            RETURNING id, title, description, event_type, date, location, max_participants, registration_required, contact_email, external_link, organizer_id,
                      strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals, content_produced, expected_attendance,
                      social_growth_target, target_audience, quarterly_goals, strategic_purpose, success_metrics,
                      actual_attendance, social_traction, content_created, active_developers_summary, qualitative_feedback,
                      sponsorship_cost, travel_cost, awards_cost, other_costs,
                      project_submissions, promotion_reach, developer_integration, host_summary, event_images, created_at
            "#,
            params.title,
            params.description,
            params.event_type,
            params.date,
            params.location,
            params.max_participants,
            params.registration_required,
            params.contact_email,
            params.external_link,
            params.organizer_id,
            params.strategic_focus_areas.map(|v| v.as_slice()),
            params.monthly_active_ambassadors,
            params.monthly_active_accounts,
            params.scf_referrals,
            params.content_produced,
            params.expected_attendance,
            params.social_growth_target,
            params.target_audience,
            params.quarterly_goals,
            params.strategic_purpose,
            params.success_metrics,
            params.actual_attendance,
            params.social_traction,
            params.content_created,
            params.active_developers_summary,
            params.qualitative_feedback,
            params.sponsorship_cost,
            params.travel_cost,
            params.awards_cost,
            params.other_costs,
            params.project_submissions,
            params.promotion_reach,
            params.developer_integration,
            params.host_summary,
            params.event_images.map(|v| v.as_slice())
        )
        .fetch_one(pool)
        .await
    }

    pub async fn list_events(
        pool: &DbPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Event>, SqlxError> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        sqlx::query_as!(
            Event,
            r#"
            SELECT id, title, description, event_type, date, location, max_participants,
                   registration_required, contact_email, external_link, organizer_id, created_at,
                   strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals,
                   content_produced, expected_attendance, social_growth_target, target_audience,
                   quarterly_goals, strategic_purpose, success_metrics,
                   actual_attendance, social_traction, content_created, active_developers_summary, qualitative_feedback,
                   sponsorship_cost, travel_cost, awards_cost, other_costs,
                   project_submissions, promotion_reach, developer_integration, host_summary, event_images
            FROM events
            ORDER BY date ASC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(
        pool: &DbPool,
        event_id: i32,
    ) -> Result<Option<Event>, SqlxError> {
        sqlx::query_as!(
            Event,
            r#"
            SELECT id, title, description, event_type, date, location, max_participants,
                   registration_required, contact_email, external_link, organizer_id, created_at,
                   strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals,
                   content_produced, expected_attendance, social_growth_target, target_audience,
                   quarterly_goals, strategic_purpose, success_metrics,
                   actual_attendance, social_traction, content_created, active_developers_summary, qualitative_feedback,
                   sponsorship_cost, travel_cost, awards_cost, other_costs,
                   project_submissions, promotion_reach, developer_integration, host_summary, event_images
            FROM events WHERE id = $1
            "#,
            event_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn update_event_kpi(
        pool: &DbPool,
        event_id: i32,
        params: UpdateKpiParams<'_>,
    ) -> Result<Event, SqlxError> {
        sqlx::query_as!(
            Event,
            r#"
            UPDATE events
            SET strategic_focus_areas = $2,
                monthly_active_ambassadors = $3,
                monthly_active_accounts = $4,
                scf_referrals = $5,
                content_produced = $6,
                expected_attendance = $7,
                social_growth_target = $8,
                target_audience = $9,
                quarterly_goals = $10,
                strategic_purpose = $11,
                success_metrics = $12
            WHERE id = $1
            RETURNING id, title, description, event_type, date, location, max_participants,
                      registration_required, contact_email, external_link, organizer_id, created_at,
                      strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals,
                      content_produced, expected_attendance, social_growth_target, target_audience,
                      quarterly_goals, strategic_purpose, success_metrics,
                      actual_attendance, social_traction, content_created, active_developers_summary, qualitative_feedback,
                      sponsorship_cost, travel_cost, awards_cost, other_costs,
                      project_submissions, promotion_reach, developer_integration, host_summary, event_images
            "#,
            event_id,
            params.strategic_focus_areas.map(|v| v.as_slice()),
            params.monthly_active_ambassadors,
            params.monthly_active_accounts,
            params.scf_referrals,
            params.content_produced,
            params.expected_attendance,
            params.social_growth_target,
            params.target_audience,
            params.quarterly_goals,
            params.strategic_purpose,
            params.success_metrics
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update_post_event_data(
        pool: &DbPool,
        event_id: i32,
        params: UpdatePostEventParams<'_>,
    ) -> Result<Event, SqlxError> {
        sqlx::query_as!(
            Event,
            r#"
            UPDATE events
            SET actual_attendance = $2,
                social_traction = $3,
                content_created = $4,
                active_developers_summary = $5,
                qualitative_feedback = $6,
                sponsorship_cost = $7,
                travel_cost = $8,
                awards_cost = $9,
                other_costs = $10,
                project_submissions = $11,
                promotion_reach = $12,
                developer_integration = $13,
                host_summary = $14,
                event_images = $15
            WHERE id = $1
            RETURNING id, title, description, event_type, date, location, max_participants,
                      registration_required, contact_email, external_link, organizer_id, created_at,
                      strategic_focus_areas, monthly_active_ambassadors, monthly_active_accounts, scf_referrals,
                      content_produced, expected_attendance, social_growth_target, target_audience,
                      quarterly_goals, strategic_purpose, success_metrics,
                      actual_attendance, social_traction, content_created, active_developers_summary, qualitative_feedback,
                      sponsorship_cost, travel_cost, awards_cost, other_costs,
                      project_submissions, promotion_reach, developer_integration, host_summary, event_images
            "#,
            event_id,
            params.actual_attendance,
            params.social_traction,
            params.content_created,
            params.active_developers_summary,
            params.qualitative_feedback,
            params.sponsorship_cost,
            params.travel_cost,
            params.awards_cost,
            params.other_costs,
            params.project_submissions,
            params.promotion_reach,
            params.developer_integration,
            params.host_summary,
            params.event_images.map(|v| v.as_slice())
        )
        .fetch_one(pool)
        .await
    }
}