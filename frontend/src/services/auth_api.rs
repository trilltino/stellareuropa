use reqwest::Client;
use shared::dto::{SignupRequest, LoginRequest, AuthResponse, UserInfo, Guest};
use crate::services::api_error::ApiError;
use web_sys::console;

const API_BASE_URL: &str = "http://127.0.0.1:8080/api";


pub async fn signup(request: SignupRequest) -> Result<AuthResponse, ApiError> {
    let client = Client::new();

    console::log_1(&"[AUTH API] 🔐 Sending signup request".into());
    console::log_2(&"[AUTH API]    Username:".into(), &request.username.clone().into());
    console::log_2(&"[AUTH API]    Email:".into(), &request.email.clone().into());
    console::log_2(&"[AUTH API]    Role:".into(), &request.role.clone().into());

    match client
        .post(format!("{API_BASE_URL}/auth/signup"))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            console::log_2(&"[AUTH API]    Response status:".into(), &status.as_u16().into());

            if status.is_success() {
                match response.json::<AuthResponse>().await {
                    Ok(auth_response) => {
                        console::log_1(&"[AUTH API] ✅ Signup successful".into());
                        console::log_2(&"[AUTH API]    User ID:".into(), &auth_response.user.id.clone().into());
                        console::log_2(&"[AUTH API]    Username:".into(), &auth_response.user.username.clone().into());
                        Ok(auth_response)
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH API] ❌ Failed to parse response: {e}").into());
                        Err(ApiError::ParseError(e.to_string()))
                    }
                }
            } else {
                let status_code = status.as_u16();
                match response.text().await {
                    Ok(error_text) => {
                        console::error_1(&format!("[AUTH API] ❌ Signup failed: {error_text}").into());
                        Err(ApiError::HttpError {
                            status: status_code,
                            message: error_text,
                        })
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH API] ❌ Failed to read error: {e}").into());
                        Err(ApiError::HttpError {
                            status: status_code,
                            message: "Unknown error".to_string(),
                        })
                    }
                }
            }
        }
        Err(e) => {
            console::error_1(&format!("[AUTH API] ❌ Network error: {e}").into());
            Err(ApiError::NetworkError(e.to_string()))
        }
    }
}



pub async fn login(request: LoginRequest) -> Result<AuthResponse, ApiError> {
    let client = Client::new();

    console::log_1(&"[AUTH API] 🔐 Sending login request".into());
    console::log_2(&"[AUTH API]    Email/Username:".into(), &request.email_or_username.clone().into());

    match client
        .post(format!("{API_BASE_URL}/auth/login"))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            console::log_2(&"[AUTH API]    Response status:".into(), &status.as_u16().into());

            if status.is_success() {
                match response.json::<AuthResponse>().await {
                    Ok(auth_response) => {
                        console::log_1(&"[AUTH API] ✅ Login successful".into());
                        console::log_2(&"[AUTH API]    User ID:".into(), &auth_response.user.id.clone().into());
                        console::log_2(&"[AUTH API]    Username:".into(), &auth_response.user.username.clone().into());
                        console::log_2(&"[AUTH API]    Role:".into(), &auth_response.user.role.clone().into());
                        Ok(auth_response)
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH API] ❌ Failed to parse response: {e}").into());
                        Err(ApiError::ParseError(e.to_string()))
                    }
                }
            } else {
                let status_code = status.as_u16();
                match response.text().await {
                    Ok(error_text) => {
                        console::error_1(&format!("[AUTH API] ❌ Login failed: {error_text}").into());
                        Err(ApiError::HttpError {
                            status: status_code,
                            message: error_text,
                        })
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH API] ❌ Failed to read error: {e}").into());
                        Err(ApiError::HttpError {
                            status: status_code,
                            message: "Unknown error".to_string(),
                        })
                    }
                }
            }
        }
        Err(e) => {
            console::error_1(&format!("[AUTH API] ❌ Network error: {e}").into());
            Err(ApiError::NetworkError(e.to_string()))
        }
    }
}


pub async fn wallet_login(request: Guest) -> Result<AuthResponse, ApiError> {
    let client = Client::new();

    console::log_1(&"[AUTH API] 👛 Sending wallet login request".into());
    console::log_2(&"[AUTH API]    Username:".into(), &request.username.clone().into());
    console::log_2(&"[AUTH API]    Wallet:".into(), &format!("{}...{}", &request.wallet_address[..6], &request.wallet_address[request.wallet_address.len()-6..]).into());

    match client
        .post(format!("{API_BASE_URL}/auth/wallet"))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            console::log_2(&"[AUTH API]    Response status:".into(), &status.as_u16().into());

            if status.is_success() {
                match response.json::<AuthResponse>().await {
                    Ok(auth_response) => {
                        console::log_1(&"[AUTH API] ✅ Wallet login successful".into());
                        console::log_2(&"[AUTH API]    User ID:".into(), &auth_response.user.id.clone().into());
                        console::log_2(&"[AUTH API]    Username:".into(), &auth_response.user.username.clone().into());
                        console::log_2(&"[AUTH API]    Role:".into(), &auth_response.user.role.clone().into());
                        Ok(auth_response)
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH API] ❌ Failed to parse response: {e}").into());
                        Err(ApiError::ParseError(e.to_string()))
                    }
                }
            } else {
                let status_code = status.as_u16();
                match response.text().await {
                    Ok(error_text) => {
                        console::error_1(&format!("[AUTH API] ❌ Wallet login failed: {error_text}").into());
                        Err(ApiError::HttpError {
                            status: status_code,
                            message: error_text,
                        })
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH API] ❌ Failed to read error: {e}").into());
                        Err(ApiError::HttpError {
                            status: status_code,
                            message: "Unknown error".to_string(),
                        })
                    }
                }
            }
        }
        Err(e) => {
            console::error_1(&format!("[AUTH API] ❌ Network error: {e}").into());
            Err(ApiError::NetworkError(e.to_string()))
        }
    }
}


pub async fn logout() -> Result<(), ApiError> {
    let client = Client::new();

    console::log_1(&"[AUTH API] 🔓 Sending logout request".into());

    match client
        .post(format!("{API_BASE_URL}/auth/logout"))
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            console::log_2(&"[AUTH API]    Response status:".into(), &status.as_u16().into());

            if status.is_success() {
                console::log_1(&"[AUTH API] ✅ Logout successful".into());
                Ok(())
            } else {
                let status_code = status.as_u16();
                match response.text().await {
                    Ok(error_text) => {
                        console::error_1(&format!("[AUTH API] ❌ Logout failed: {error_text}").into());
                        Err(ApiError::HttpError {
                            status: status_code,
                            message: error_text,
                        })
                    }
                    Err(_) => {
                        console::error_1(&"[AUTH API] ❌ Logout failed".into());
                        Err(ApiError::HttpError {
                            status: status_code,
                            message: "Logout failed".to_string(),
                        })
                    }
                }
            }
        }
        Err(e) => {
            console::error_1(&format!("[AUTH API] ❌ Network error: {e}").into());
            Err(ApiError::NetworkError(e.to_string()))
        }
    }
}


pub async fn get_current_user() -> Result<UserInfo, ApiError> {
    let client = Client::new();

    console::log_1(&"[AUTH API] 🔍 Fetching current user".into());

    match client
        .get(format!("{API_BASE_URL}/auth/me"))
        .send()
        .await
    {
        Ok(response) => {
            let status = response.status();
            console::log_2(&"[AUTH API]    Response status:".into(), &status.as_u16().into());

            if status.is_success() {
                match response.json::<UserInfo>().await {
                    Ok(user) => {
                        console::log_1(&"[AUTH API] ✅ Got current user".into());
                        console::log_2(&"[AUTH API]    User ID:".into(), &user.id.clone().into());
                        console::log_2(&"[AUTH API]    Username:".into(), &user.username.clone().into());
                        Ok(user)
                    }
                    Err(e) => {
                        console::error_1(&format!("[AUTH API] ❌ Failed to parse user: {e}").into());
                        Err(ApiError::ParseError(e.to_string()))
                    }
                }
            } else {
                let status_code = status.as_u16();
                console::error_1(&format!("[AUTH API] ❌ Get current user failed with status {status_code}").into());

                // 401 means not authenticated
                if status_code == 401 {
                    Err(ApiError::HttpError {
                        status: status_code,
                        message: "Not authenticated".to_string(),
                    })
                } else {
                    match response.text().await {
                        Ok(error_text) => Err(ApiError::HttpError {
                            status: status_code,
                            message: error_text,
                        }),
                        Err(_) => Err(ApiError::HttpError {
                            status: status_code,
                            message: "Unknown error".to_string(),
                        }),
                    }
                }
            }
        }
        Err(e) => {
            console::error_1(&format!("[AUTH API] ❌ Network error: {e}").into());
            Err(ApiError::NetworkError(e.to_string()))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_base_url_is_correct() {
        assert_eq!(API_BASE_URL, "http://127.0.0.1:8080/api");
    }
}
