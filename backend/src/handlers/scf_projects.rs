use crate::database::connection::DbPool;
use crate::database::repositories::SCFProjectRepository;
use crate::handlers::error::{ApiError, ApiResult};
use axum::{
    extract::{Json, State, Query, Path},
    http::StatusCode,
};
use tracing::{info, error, debug};
use shared::dto::{
    SCFProjectRequest, SCFProjectResponse, SCFProjectListResponse,
    ProjectCategory, ProjectType, IntegrationStatus, SubmitterType,
    ProjectStatus, TeamMember
};
use crate::database::models::SCFProject;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListProjectsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn create_project_response(project: &SCFProject) -> SCFProjectResponse {
    // Use From trait implementations for clean enum conversion
    let project_category = ProjectCategory::from(project.project_category.as_str());
    let project_type = ProjectType::from(project.project_type.as_str());
    let integration_status = IntegrationStatus::from(project.integration_status.as_str());
    let submitter_type = SubmitterType::from(project.submitter_type.as_str());
    let status = ProjectStatus::from(project.status.as_str());

    let team_members = project.team_members.as_ref()
        .and_then(|json| serde_json::from_value::<Vec<TeamMember>>(json.clone()).ok());

    SCFProjectResponse {
        id: project.id.to_string(),
        project_title: project.project_title.clone(),
        description: project.description.clone(),
        video_url: project.video_url.clone(),
        project_category,
        project_type,
        regions_of_operation: project.regions_of_operation.clone(),
        country: project.country.clone(),
        other_chains: project.other_chains.clone(),
        current_traction: project.current_traction.clone(),
        integration_status,
        integration_description: project.integration_description.clone(),
        website: project.website.clone(),
        open_source: project.open_source,
        analytics_url: project.analytics_url.clone(),
        analytics_explanation: project.analytics_explanation.clone(),
        x_url: project.x_url.clone(),
        pitch_deck_url: project.pitch_deck_url.clone(),
        linkedin_url: project.linkedin_url.clone(),
        discord_url: project.discord_url.clone(),
        project_thumbnail: project.project_thumbnail.clone(),
        submitter_type,
        team_description: project.team_description.clone(),
        team_member_count: project.team_member_count as u32,
        team_members,
        support_needed: project.support_needed.clone(),
        created_at: project.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
        updated_at: project.updated_at.map(|dt| dt.to_string()),
        status,
    }
}

pub async fn create_project(
    current_user: crate::extractors::current_user::CurrentUser,
    State(pool): State<DbPool>,
    Json(req): Json<SCFProjectRequest>,
) -> ApiResult<(StatusCode, Json<SCFProjectResponse>)> {
    let submitter_id = current_user.0.user_id;

    info!(
        "Received SCF project creation request from user {}: title={}",
        submitter_id, req.project_title
    );

    debug!("[PROJECT] NEW SCF PROJECT SUBMISSION");
    debug!("   User ID: {}", submitter_id);
    debug!("   Username: {}", current_user.0.username);
    debug!("   Title: {}", req.project_title);
    debug!("   Category: {:?}", req.project_category);
    debug!("   Type: {:?}", req.project_type);
    debug!("   Country: {}", req.country);
    debug!("   Team Members: {}", req.team_member_count);
    debug!("   ────────────────────────────────────");

    // Convert enum to string for database
    let project_category_str = req.project_category.to_string();
    let project_type_str = req.project_type.to_string();
    let integration_status_str = req.integration_status.to_string();
    let submitter_type_str = req.submitter_type.to_string();

    // Convert team_members to JSON
    let team_members_json = req.team_members.as_ref()
        .and_then(|members| serde_json::to_value(members).ok());

    let project = SCFProjectRepository::create_project(
        &pool,
        &req.project_title,
        &req.description,
        req.video_url.as_deref(),
        &project_category_str,
        &project_type_str,
        &req.regions_of_operation,
        &req.country,
        req.other_chains.as_ref(),
        &req.current_traction,
        &integration_status_str,
        &req.integration_description,
        &req.website,
        req.open_source,
        req.analytics_url.as_deref(),
        req.analytics_explanation.as_deref(),
        req.x_url.as_deref(),
        req.pitch_deck_url.as_deref(),
        req.linkedin_url.as_deref(),
        req.discord_url.as_deref(),
        req.project_thumbnail.as_deref(),
        &submitter_type_str,
        &req.team_description,
        req.team_member_count as i32,
        team_members_json.as_ref(),
        req.support_needed.as_deref(),
        submitter_id,
    ).await.map_err(|e| {
        error!("Failed to create SCF project: {:?}", e);
        ApiError::database(format!("Failed to create project: {e}"))
    })?;

    info!("SCF project created successfully with ID: {}", project.id);
    debug!("[SUCCESS] PROJECT CREATED!");
    debug!("   ID: {}", project.id);
    debug!("   Status: {}", project.status);

    let response = create_project_response(&project);
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_project(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> ApiResult<Json<SCFProjectResponse>> {
    info!("Fetching SCF project with ID: {}", id);

    let project = SCFProjectRepository::get_project_by_id(&pool, id)
        .await
        .map_err(|e| {
            error!("Failed to fetch SCF project: {:?}", e);
            ApiError::database(format!("Failed to fetch project: {e}"))
        })?
        .ok_or_else(|| ApiError::not_found(format!("Project with ID {id} not found")))?;

    let response = create_project_response(&project);
    Ok(Json(response))
}

pub async fn list_projects(
    State(pool): State<DbPool>,
    Query(params): Query<ListProjectsQuery>,
) -> ApiResult<Json<SCFProjectListResponse>> {
    info!("Fetching SCF projects list with limit={:?}, offset={:?}", params.limit, params.offset);

    let projects = SCFProjectRepository::list_projects(&pool, params.limit, params.offset)
        .await
        .map_err(|e| {
            error!("Failed to fetch SCF projects: {:?}", e);
            ApiError::database(format!("Failed to fetch projects: {e}"))
        })?;

    let total = SCFProjectRepository::count_projects(&pool)
        .await
        .unwrap_or(projects.len() as i64) as usize;

    let responses: Vec<SCFProjectResponse> = projects.iter()
        .map(create_project_response)
        .collect();

    let list_response = SCFProjectListResponse {
        projects: responses,
        total,
    };

    Ok(Json(list_response))
}

pub async fn update_project_status(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(status): Json<String>,
) -> ApiResult<Json<SCFProjectResponse>> {
    info!("Updating SCF project {} status to: {}", id, status);

    let project = SCFProjectRepository::update_project_status(&pool, id, &status)
        .await
        .map_err(|e| {
            error!("Failed to update SCF project status: {:?}", e);
            ApiError::database(format!("Failed to update project status: {e}"))
        })?;

    let response = create_project_response(&project);
    Ok(Json(response))
}
