use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    Workshop,
    Meetup,
    Conference,
    Hackathon,
    Community,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StrategicFocusArea {
    CommunityParticipation,
    OnChainActivity,
    SCFReferrals,
    EcosystemCollaboration,
    DeveloperGrowth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct KPIEstimates {
    pub monthly_active_ambassadors: Option<u32>,
    pub monthly_active_accounts: Option<u32>,
    pub scf_referrals: Option<u32>,
    pub content_produced: Option<u32>,
    pub expected_attendance: Option<u32>,
    pub social_growth_target: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostEventReport {
    pub actual_attendance: Option<u32>,
    pub social_traction: Option<String>,
    pub content_created: Option<String>,
    pub active_developers_summary: Option<String>,
    pub qualitative_feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventCosts {
    pub sponsorship_cost: Option<f64>,
    pub travel_cost: Option<f64>,
    pub awards_cost: Option<f64>,
    pub other_costs: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventEvaluation {
    pub project_submissions: Option<u32>,
    pub promotion_reach: Option<String>,
    pub developer_integration: Option<String>,
    pub host_summary: Option<String>,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Workshop => write!(f, "Workshop"),
            EventType::Meetup => write!(f, "Meetup"),
            EventType::Conference => write!(f, "Conference"),
            EventType::Hackathon => write!(f, "Hackathon"),
            EventType::Community => write!(f, "Community Event"),
        }
    }
}

impl std::fmt::Display for StrategicFocusArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrategicFocusArea::CommunityParticipation => write!(f, "Community Participation"),
            StrategicFocusArea::OnChainActivity => write!(f, "On-Chain Activity"),
            StrategicFocusArea::SCFReferrals => write!(f, "SCF Referrals"),
            StrategicFocusArea::EcosystemCollaboration => write!(f, "Ecosystem Collaboration"),
            StrategicFocusArea::DeveloperGrowth => write!(f, "Developer Growth"),
        }
    }
}

impl From<&str> for EventType {
    fn from(s: &str) -> Self {
        match s {
            "Workshop" => EventType::Workshop,
            "Meetup" => EventType::Meetup,
            "Conference" => EventType::Conference,
            "Hackathon" => EventType::Hackathon,
            _ => EventType::Community,
        }
    }
}

impl From<&str> for StrategicFocusArea {
    fn from(s: &str) -> Self {
        match s {
            "Community Participation" => StrategicFocusArea::CommunityParticipation,
            "On-Chain Activity" => StrategicFocusArea::OnChainActivity,
            "SCF Referrals" => StrategicFocusArea::SCFReferrals,
            "Ecosystem Collaboration" => StrategicFocusArea::EcosystemCollaboration,
            "Developer Growth" => StrategicFocusArea::DeveloperGrowth,
            _ => StrategicFocusArea::CommunityParticipation, // Default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventRequest {
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub date: String, // ISO date string
    pub location: String,
    pub max_participants: Option<u32>,
    pub registration_required: bool,
    pub contact_email: String,
    pub external_link: Option<String>,
    // KPI Planning fields
    pub strategic_focus_areas: Vec<StrategicFocusArea>,
    pub kpi_estimates: KPIEstimates,
    pub target_audience: String,
    pub quarterly_goals: String,
    pub strategic_purpose: String,
    pub success_metrics: Option<String>,
    // Post-Event Reporting (optional - to be filled after event)
    pub post_event_report: Option<PostEventReport>,
    pub event_costs: Option<EventCosts>,
    pub event_evaluation: Option<EventEvaluation>,
    pub event_images: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub date: String,
    pub location: String,
    pub max_participants: Option<u32>,
    pub registration_required: bool,
    pub contact_email: String,
    pub external_link: Option<String>,
    pub organizer: String, // username of organizer
    pub created_at: String,
    // KPI Planning fields
    pub strategic_focus_areas: Vec<StrategicFocusArea>,
    pub kpi_estimates: KPIEstimates,
    pub target_audience: String,
    pub quarterly_goals: String,
    pub strategic_purpose: String,
    pub success_metrics: Option<String>,
    // Post-Event Reporting
    pub post_event_report: Option<PostEventReport>,
    pub event_costs: Option<EventCosts>,
    pub event_evaluation: Option<EventEvaluation>,
    pub event_images: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventListResponse {
    pub events: Vec<EventResponse>,
    pub total: usize,
}