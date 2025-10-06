use serde::{Deserialize, Serialize};

// ===== LEGACY DTOs (kept for backward compatibility) =====

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum UserType {
    Ambassador,
    ChapterLead,
}

impl std::fmt::Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Ambassador => write!(f, "Ambassador"),
            UserType::ChapterLead => write!(f, "ChapterLead"),
        }
    }
}

/// Legacy signup request (wallet-based)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignUpRequest {
    pub username: String,
    pub email: String,
    pub wallet_address: String,
    pub user_type: UserType,
    pub organization: Option<String>,
    pub bio: Option<String>,
}

// ===== NEW AUTH DTOs =====

/// New signup request with password authentication
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String, // "visitor" or "chapter_lead"
    pub wallet_address: Option<String>,
}

/// Login request (password-based)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email_or_username: String, // Can be either email or username
    pub password: String,
}

/// Freighter wallet login/signup request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Guest {
    pub username: String,
    pub wallet_address: String,
}

/// Authentication response (signup/login)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub user: UserInfo,
    pub message: String,
}

/// User information (public, safe to send to client)
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub wallet_address: Option<String>,
    pub created_at: String,
}

// Note: From<User> implementation is in backend (requires User model)

/// Error response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub error: String,
}