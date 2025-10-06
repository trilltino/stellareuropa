use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use bigdecimal::BigDecimal;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub event_type: String,
    pub date: DateTime<Utc>,
    pub location: String,
    pub max_participants: Option<i32>,
    pub registration_required: bool,
    pub contact_email: String,
    pub external_link: Option<String>,
    pub organizer_id: i32,
    pub created_at: Option<DateTime<Utc>>,
    // KPI Planning fields
    pub strategic_focus_areas: Option<Vec<String>>,
    pub monthly_active_ambassadors: Option<i32>,
    pub monthly_active_accounts: Option<i32>,
    pub scf_referrals: Option<i32>,
    pub content_produced: Option<i32>,
    pub expected_attendance: Option<i32>,
    pub social_growth_target: Option<i32>,
    pub target_audience: String,
    pub quarterly_goals: String,
    pub strategic_purpose: String,
    pub success_metrics: Option<String>,
    // Post-Event Reporting
    pub actual_attendance: Option<i32>,
    pub social_traction: Option<String>,
    pub content_created: Option<String>,
    pub active_developers_summary: Option<String>,
    pub qualitative_feedback: Option<String>,
    // Event Costs
    pub sponsorship_cost: Option<BigDecimal>,
    pub travel_cost: Option<BigDecimal>,
    pub awards_cost: Option<BigDecimal>,
    pub other_costs: Option<BigDecimal>,
    // Event Evaluation
    pub project_submissions: Option<i32>,
    pub promotion_reach: Option<String>,
    pub developer_integration: Option<String>,
    pub host_summary: Option<String>,
    pub event_images: Option<Vec<String>>,
}

// Removed unused Event::new() constructor - entities are created through repository methods