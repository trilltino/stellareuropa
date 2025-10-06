use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;

/// Parameters for creating a new event
///
/// This struct groups the 36+ parameters previously passed individually,
/// making the function signature readable and maintainable.
#[derive(Debug, Clone)]
pub struct CreateEventParams<'a> {
    // Basic event information
    pub title: &'a str,
    pub description: &'a str,
    pub event_type: &'a str,
    pub date: DateTime<Utc>,
    pub location: &'a str,
    pub max_participants: Option<i32>,
    pub registration_required: bool,
    pub contact_email: &'a str,
    pub external_link: Option<&'a str>,
    pub organizer_id: i32,

    // Strategic planning
    pub strategic_focus_areas: Option<&'a Vec<String>>,
    pub target_audience: &'a str,
    pub quarterly_goals: &'a str,
    pub strategic_purpose: &'a str,
    pub success_metrics: Option<&'a str>,

    // KPI estimates
    pub monthly_active_ambassadors: Option<i32>,
    pub monthly_active_accounts: Option<i32>,
    pub scf_referrals: Option<i32>,
    pub content_produced: Option<i32>,
    pub expected_attendance: Option<i32>,
    pub social_growth_target: Option<i32>,

    // Post-event report (optional)
    pub actual_attendance: Option<i32>,
    pub social_traction: Option<&'a str>,
    pub content_created: Option<&'a str>,
    pub active_developers_summary: Option<&'a str>,
    pub qualitative_feedback: Option<&'a str>,

    // Event costs (optional)
    pub sponsorship_cost: Option<BigDecimal>,
    pub travel_cost: Option<BigDecimal>,
    pub awards_cost: Option<BigDecimal>,
    pub other_costs: Option<BigDecimal>,

    // Event evaluation (optional)
    pub project_submissions: Option<i32>,
    pub promotion_reach: Option<&'a str>,
    pub developer_integration: Option<&'a str>,
    pub host_summary: Option<&'a str>,

    // Media
    pub event_images: Option<&'a Vec<String>>,
}

/// Parameters for updating event KPI data
#[derive(Debug, Clone)]
pub struct UpdateKpiParams<'a> {
    pub strategic_focus_areas: Option<&'a Vec<String>>,
    pub monthly_active_ambassadors: Option<i32>,
    pub monthly_active_accounts: Option<i32>,
    pub scf_referrals: Option<i32>,
    pub content_produced: Option<i32>,
    pub expected_attendance: Option<i32>,
    pub social_growth_target: Option<i32>,
    pub target_audience: &'a str,
    pub quarterly_goals: &'a str,
    pub strategic_purpose: &'a str,
    pub success_metrics: Option<&'a str>,
}

/// Parameters for updating post-event data
#[derive(Debug, Clone)]
pub struct UpdatePostEventParams<'a> {
    // Post-event report
    pub actual_attendance: Option<i32>,
    pub social_traction: Option<&'a str>,
    pub content_created: Option<&'a str>,
    pub active_developers_summary: Option<&'a str>,
    pub qualitative_feedback: Option<&'a str>,

    // Event costs
    pub sponsorship_cost: Option<BigDecimal>,
    pub travel_cost: Option<BigDecimal>,
    pub awards_cost: Option<BigDecimal>,
    pub other_costs: Option<BigDecimal>,

    // Event evaluation
    pub project_submissions: Option<i32>,
    pub promotion_reach: Option<&'a str>,
    pub developer_integration: Option<&'a str>,
    pub host_summary: Option<&'a str>,

    // Media
    pub event_images: Option<&'a Vec<String>>,
}
