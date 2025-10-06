use reqwest::Client;
use shared::dto::{SignUpRequest, SignUpResponse, EventRequest, EventListResponse, SCFProjectRequest, SCFProjectResponse};
use crate::services::api_error::ApiError;
use web_sys::window;

fn get_api_base_url() -> String {
    if let Some(win) = window() {
        if let Ok(location) = win.location().origin() {
            return format!("{location}/api");
        }
    }
    "http://127.0.0.1:8080/api".to_string()
}

pub async fn signup(request: SignUpRequest) -> Result<SignUpResponse, ApiError> {
    let client = Client::new();
    let api_url = get_api_base_url();

    match client
        .post(format!("{api_url}/signup"))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<SignUpResponse>().await {
                    Ok(signup_response) => Ok(signup_response),
                    Err(e) => Err(ApiError::ParseError(e.to_string())),
                }
            } else {
                let status = response.status().as_u16();
                match response.text().await {
                    Ok(error_text) => Err(ApiError::HttpError { status, message: error_text }),
                    Err(_) => Err(ApiError::HttpError { status, message: "Unknown error".to_string() })
                }
            }
        }
        Err(e) => Err(ApiError::NetworkError(e.to_string())),
    }
}

pub async fn create_event(request: EventRequest) -> Result<String, ApiError> {
    let client = Client::new();
    let api_url = get_api_base_url();

    match client
        .post(format!("{api_url}/events"))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(message) => Ok(message),
                    Err(e) => Err(ApiError::ParseError(e.to_string())),
                }
            } else {
                let status = response.status().as_u16();
                match response.text().await {
                    Ok(error_text) => Err(ApiError::HttpError { status, message: error_text }),
                    Err(_) => Err(ApiError::HttpError { status, message: "Unknown error".to_string() })
                }
            }
        }
        Err(e) => Err(ApiError::NetworkError(e.to_string())),
    }
}

pub async fn list_events(limit: Option<u32>, offset: Option<u32>) -> Result<EventListResponse, ApiError> {
    let client = Client::new();
    let api_url = get_api_base_url();

    let mut url = format!("{api_url}/events");
    let mut params = Vec::new();

    if let Some(limit) = limit {
        params.push(format!("limit={limit}"));
    }
    if let Some(offset) = offset {
        params.push(format!("offset={offset}"));
    }

    if !params.is_empty() {
        url.push('?');
        url.push_str(&params.join("&"));
    }

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<EventListResponse>().await {
                    Ok(events_response) => Ok(events_response),
                    Err(e) => Err(ApiError::ParseError(e.to_string())),
                }
            } else {
                let status = response.status().as_u16();
                match response.text().await {
                    Ok(error_text) => Err(ApiError::HttpError { status, message: error_text }),
                    Err(_) => Err(ApiError::HttpError { status, message: "Unknown error".to_string() })
                }
            }
        }
        Err(e) => Err(ApiError::NetworkError(e.to_string())),
    }
}

pub async fn health_check() -> Result<String, ApiError> {
    let client = Client::new();

    // Get base URL and strip /api suffix for health endpoint
    let base = get_api_base_url().trim_end_matches("/api").to_string();

    match client.get(format!("{base}/health")).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => Ok(text),
                    Err(e) => Err(ApiError::ParseError(e.to_string())),
                }
            } else {
                let status = response.status().as_u16();
                Err(ApiError::HttpError { status, message: "Health check failed".to_string() })
            }
        }
        Err(e) => Err(ApiError::NetworkError(e.to_string())),
    }
}

pub async fn create_scf_project(request: SCFProjectRequest) -> Result<SCFProjectResponse, ApiError> {
    let client = Client::new();
    let api_url = get_api_base_url();

    match client
        .post(format!("{api_url}/scf-projects"))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<SCFProjectResponse>().await {
                    Ok(project_response) => Ok(project_response),
                    Err(e) => Err(ApiError::ParseError(e.to_string())),
                }
            } else {
                let status = response.status().as_u16();
                match response.text().await {
                    Ok(error_text) => Err(ApiError::HttpError { status, message: error_text }),
                    Err(_) => Err(ApiError::HttpError { status, message: "Unknown error".to_string() })
                }
            }
        }
        Err(e) => Err(ApiError::NetworkError(e.to_string())),
    }
}

pub async fn update_event_kpi(event_id: u32, request: EventRequest) -> Result<String, ApiError> {
    let client = Client::new();
    let api_url = get_api_base_url();

    match client
        .patch(format!("{api_url}/events/{event_id}/kpi"))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(message) => Ok(message),
                    Err(e) => Err(ApiError::ParseError(e.to_string())),
                }
            } else {
                let status = response.status().as_u16();
                match response.text().await {
                    Ok(error_text) => Err(ApiError::HttpError { status, message: error_text }),
                    Err(_) => Err(ApiError::HttpError { status, message: "Unknown error".to_string() })
                }
            }
        }
        Err(e) => Err(ApiError::NetworkError(e.to_string())),
    }
}

pub async fn update_post_event_data(event_id: u32, request: EventRequest) -> Result<String, ApiError> {
    let client = Client::new();
    let api_url = get_api_base_url();

    match client
        .patch(format!("{api_url}/events/{event_id}/post-event"))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(message) => Ok(message),
                    Err(e) => Err(ApiError::ParseError(e.to_string())),
                }
            } else {
                let status = response.status().as_u16();
                match response.text().await {
                    Ok(error_text) => Err(ApiError::HttpError { status, message: error_text }),
                    Err(_) => Err(ApiError::HttpError { status, message: "Unknown error".to_string() })
                }
            }
        }
        Err(e) => Err(ApiError::NetworkError(e.to_string())),
    }
}