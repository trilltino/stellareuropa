use reqwest::Client;
use shared::dto::{SignUpRequest, SignUpResponse, EventRequest, EventListResponse};

const API_BASE_URL: &str = "http://127.0.0.1:8080/api";

pub async fn signup(request: SignUpRequest) -> Result<SignUpResponse, String> {
    let client = Client::new();

    match client
        .post(&format!("{}/signup", API_BASE_URL))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<SignUpResponse>().await {
                    Ok(signup_response) => Ok(signup_response),
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            } else {
                let status = response.status();
                match response.text().await {
                    Ok(error_text) => Err(error_text),
                    Err(_) => Err(format!("HTTP error: {}", status))
                }
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn create_event(request: EventRequest) -> Result<String, String> {
    let client = Client::new();

    match client
        .post(&format!("{}/events", API_BASE_URL))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(message) => Ok(message),
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            } else {
                let status = response.status();
                match response.text().await {
                    Ok(error_text) => Err(error_text),
                    Err(_) => Err(format!("HTTP error: {}", status))
                }
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn list_events(limit: Option<u32>, offset: Option<u32>) -> Result<EventListResponse, String> {
    let client = Client::new();

    let mut url = format!("{}/events", API_BASE_URL);
    let mut params = Vec::new();

    if let Some(limit) = limit {
        params.push(format!("limit={}", limit));
    }
    if let Some(offset) = offset {
        params.push(format!("offset={}", offset));
    }

    if !params.is_empty() {
        url.push_str("?");
        url.push_str(&params.join("&"));
    }

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<EventListResponse>().await {
                    Ok(events_response) => Ok(events_response),
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            } else {
                let status = response.status();
                match response.text().await {
                    Ok(error_text) => Err(error_text),
                    Err(_) => Err(format!("HTTP error: {}", status))
                }
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn health_check() -> Result<String, String> {
    let client = Client::new();

    match client.get("http://127.0.0.1:8080/health").send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(text) => Ok(text),
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            } else {
                let status = response.status();
                Err(format!("HTTP error: {}", status))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}