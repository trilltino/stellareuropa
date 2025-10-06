use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SCFProject {
    pub id: i32,

    // Basic Information
    pub project_title: String,
    pub description: String,
    pub video_url: Option<String>,
    pub project_category: String,
    pub project_type: String,
    pub regions_of_operation: Vec<String>,
    pub country: String,
    pub other_chains: Option<Vec<String>>,

    // Traction & Integration
    pub current_traction: String,
    pub integration_status: String,
    pub integration_description: String,

    // Links & Resources
    pub website: String,
    pub open_source: bool,
    pub analytics_url: Option<String>,
    pub analytics_explanation: Option<String>,
    pub x_url: Option<String>,
    pub pitch_deck_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub discord_url: Option<String>,
    pub project_thumbnail: Option<String>,

    // Team Information
    pub submitter_type: String,
    pub team_description: String,
    pub team_member_count: i32,
    pub team_members: Option<sqlx::types::JsonValue>, // JSONB

    // Support & Additional Info
    pub support_needed: Option<String>,

    // Metadata
    pub submitter_id: i32,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl SCFProject {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_title: String,
        description: String,
        video_url: Option<String>,
        project_category: String,
        project_type: String,
        regions_of_operation: Vec<String>,
        country: String,
        other_chains: Option<Vec<String>>,
        current_traction: String,
        integration_status: String,
        integration_description: String,
        website: String,
        open_source: bool,
        analytics_url: Option<String>,
        analytics_explanation: Option<String>,
        x_url: Option<String>,
        pitch_deck_url: Option<String>,
        linkedin_url: Option<String>,
        discord_url: Option<String>,
        project_thumbnail: Option<String>,
        submitter_type: String,
        team_description: String,
        team_member_count: i32,
        team_members: Option<sqlx::types::JsonValue>,
        support_needed: Option<String>,
        submitter_id: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            project_title,
            description,
            video_url,
            project_category,
            project_type,
            regions_of_operation,
            country,
            other_chains,
            current_traction,
            integration_status,
            integration_description,
            website,
            open_source,
            analytics_url,
            analytics_explanation,
            x_url,
            pitch_deck_url,
            linkedin_url,
            discord_url,
            project_thumbnail,
            submitter_type,
            team_description,
            team_member_count,
            team_members,
            support_needed,
            submitter_id,
            status: "Draft".to_string(),
            created_at: Some(now),
            updated_at: None,
        }
    }
}
