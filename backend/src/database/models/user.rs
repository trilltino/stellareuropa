use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// User model with authentication fields
/// Supports both legacy wallet-based auth and new password-based auth
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,

    // Legacy field (now optional - can be null for password-only users)
    pub wallet_address: Option<String>,

    // Legacy role field (kept for backward compatibility)
    pub user_type: String,

    // New authentication fields
    pub password_hash: Option<String>, // Bcrypt hash (nullable for wallet-only users)
    pub role: String,                  // visitor, chapter_lead, admin
    pub email_verified: bool,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,

    // User profile
    pub organization: Option<String>,
    pub bio: Option<String>,

    // Timestamps
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    /// Create new user with password authentication (recommended)
    pub fn new_with_password(
        username: String,
        email: String,
        password_hash: String,
        role: String,
        wallet_address: Option<String>,
        organization: Option<String>,
        bio: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // Will be set by database
            username,
            email,
            wallet_address,
            user_type: match role.as_str() {
                "chapter_lead" => "ChapterLead".to_string(),
                _ => "Ambassador".to_string(),
            },
            password_hash: Some(password_hash),
            role,
            email_verified: false,
            is_active: true,
            last_login: None,
            organization,
            bio,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    /// Legacy: Create user with wallet authentication only
    /// Kept for backward compatibility
    pub fn new_with_wallet(
        username: String,
        email: String,
        wallet_address: String,
        user_type: String,
        organization: Option<String>,
        bio: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let role = match user_type.as_str() {
            "ChapterLead" => "chapter_lead".to_string(),
            _ => "visitor".to_string(),
        };

        Self {
            id: 0,
            username,
            email,
            wallet_address: Some(wallet_address),
            user_type,
            password_hash: None, // No password for wallet-only users
            role,
            email_verified: false,
            is_active: true,
            last_login: None,
            organization,
            bio,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    /// Check if user has password authentication enabled
    pub fn has_password_auth(&self) -> bool {
        self.password_hash.is_some()
    }

    /// Check if user has wallet authentication enabled
    pub fn has_wallet_auth(&self) -> bool {
        self.wallet_address.is_some()
    }

    /// Check if user account is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Convert User model to UserInfo DTO (safe for client responses)
impl From<User> for shared::dto::UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            role: user.role,
            wallet_address: user.wallet_address,
            created_at: user.created_at
                .map(|dt| dt.to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
        }
    }
}

/// Convert &User reference to UserInfo DTO
impl From<&User> for shared::dto::UserInfo {
    fn from(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            wallet_address: user.wallet_address.clone(),
            created_at: user.created_at
                .map(|dt| dt.to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
        }
    }
}