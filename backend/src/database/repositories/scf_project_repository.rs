use crate::database::models::SCFProject;
use crate::database::connection::DbPool;
use sqlx::{Error as SqlxError};

pub struct SCFProjectRepository;

impl SCFProjectRepository {
    #[allow(clippy::too_many_arguments)]
    pub async fn create_project(
        pool: &DbPool,
        project_title: &str,
        description: &str,
        video_url: Option<&str>,
        project_category: &str,
        project_type: &str,
        regions_of_operation: &Vec<String>,
        country: &str,
        other_chains: Option<&Vec<String>>,
        current_traction: &str,
        integration_status: &str,
        integration_description: &str,
        website: &str,
        open_source: bool,
        analytics_url: Option<&str>,
        analytics_explanation: Option<&str>,
        x_url: Option<&str>,
        pitch_deck_url: Option<&str>,
        linkedin_url: Option<&str>,
        discord_url: Option<&str>,
        project_thumbnail: Option<&str>,
        submitter_type: &str,
        team_description: &str,
        team_member_count: i32,
        team_members: Option<&sqlx::types::JsonValue>,
        support_needed: Option<&str>,
        submitter_id: i32,
    ) -> Result<SCFProject, SqlxError> {
        sqlx::query_as!(
            SCFProject,
            r#"
            INSERT INTO scf_projects (
                project_title, description, video_url, project_category, project_type,
                regions_of_operation, country, other_chains, current_traction,
                integration_status, integration_description, website, open_source,
                analytics_url, analytics_explanation, x_url, pitch_deck_url,
                linkedin_url, discord_url, project_thumbnail, submitter_type,
                team_description, team_member_count, team_members, support_needed,
                submitter_id, status, created_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25, $26, 'Draft', NOW()
            )
            RETURNING
                id, project_title, description, video_url, project_category, project_type,
                regions_of_operation, country, other_chains, current_traction,
                integration_status, integration_description, website, open_source,
                analytics_url, analytics_explanation, x_url, pitch_deck_url,
                linkedin_url, discord_url, project_thumbnail, submitter_type,
                team_description, team_member_count, team_members, support_needed,
                submitter_id, status, created_at, updated_at
            "#,
            project_title,
            description,
            video_url,
            project_category,
            project_type,
            regions_of_operation.as_slice(),
            country,
            other_chains.map(|v| v.as_slice()),
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
            submitter_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_project_by_id(pool: &DbPool, id: i32) -> Result<Option<SCFProject>, SqlxError> {
        let project = sqlx::query_as!(
            SCFProject,
            r#"
            SELECT
                id, project_title, description, video_url, project_category, project_type,
                regions_of_operation, country, other_chains, current_traction,
                integration_status, integration_description, website, open_source,
                analytics_url, analytics_explanation, x_url, pitch_deck_url,
                linkedin_url, discord_url, project_thumbnail, submitter_type,
                team_description, team_member_count, team_members, support_needed,
                submitter_id, status, created_at, updated_at
            FROM scf_projects
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(project)
    }

    pub async fn list_projects(
        pool: &DbPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<SCFProject>, SqlxError> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let projects = sqlx::query_as!(
            SCFProject,
            r#"
            SELECT
                id, project_title, description, video_url, project_category, project_type,
                regions_of_operation, country, other_chains, current_traction,
                integration_status, integration_description, website, open_source,
                analytics_url, analytics_explanation, x_url, pitch_deck_url,
                linkedin_url, discord_url, project_thumbnail, submitter_type,
                team_description, team_member_count, team_members, support_needed,
                submitter_id, status, created_at, updated_at
            FROM scf_projects
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(projects)
    }

    pub async fn update_project_status(
        pool: &DbPool,
        id: i32,
        status: &str,
    ) -> Result<SCFProject, SqlxError> {
        sqlx::query_as!(
            SCFProject,
            r#"
            UPDATE scf_projects
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING
                id, project_title, description, video_url, project_category, project_type,
                regions_of_operation, country, other_chains, current_traction,
                integration_status, integration_description, website, open_source,
                analytics_url, analytics_explanation, x_url, pitch_deck_url,
                linkedin_url, discord_url, project_thumbnail, submitter_type,
                team_description, team_member_count, team_members, support_needed,
                submitter_id, status, created_at, updated_at
            "#,
            status,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn count_projects(pool: &DbPool) -> Result<i64, SqlxError> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)::bigint as "count!"
            FROM scf_projects
            "#
        )
        .fetch_one(pool)
        .await?;

        Ok(count)
    }
}
