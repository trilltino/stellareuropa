use std::env;
use tracing::{info, warn, debug};
use sqlx::{Pool, Postgres};
use axum::extract::FromRef;

/// Application configuration loaded from environment variables
/// Includes JWT, cookie, and security settings
#[derive(Clone, Debug)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub cookie_name: String,
    pub cookie_domain: String,
    pub cookie_secure: bool,
    pub cookie_http_only: bool,
    pub cookie_same_site: String,
}

/// Combined application state with database pool and config
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub config: Config,
}

// Implement FromRef to allow extracting Config from AppState
impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

// Implement FromRef to allow extracting Pool from AppState
impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl Config {
    /// Load configuration from environment variables
    /// Call dotenvy::dotenv() before this to load .env file
    pub fn from_env() -> Result<Self, String> {
        debug!("[CONFIG] Loading configuration from environment variables");

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| {
                warn!("[CONFIG] ❌ JWT_SECRET not found in environment");
                "JWT_SECRET must be set in .env file or environment".to_string()
            })?;

        let jwt_expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| {
                info!("[CONFIG] JWT_EXPIRATION_HOURS not set, using default: 24");
                "24".to_string()
            })
            .parse()
            .map_err(|_| {
                warn!("[CONFIG] ❌ JWT_EXPIRATION_HOURS must be a valid number");
                "JWT_EXPIRATION_HOURS must be a number".to_string()
            })?;

        let cookie_name = env::var("COOKIE_NAME")
            .unwrap_or_else(|_| {
                info!("[CONFIG] COOKIE_NAME not set, using default: stellar_auth");
                "stellar_auth".to_string()
            });

        let cookie_domain = env::var("COOKIE_DOMAIN")
            .unwrap_or_else(|_| {
                info!("[CONFIG] COOKIE_DOMAIN not set, using default: localhost");
                "localhost".to_string()
            });

        let cookie_secure = env::var("COOKIE_SECURE")
            .unwrap_or_else(|_| {
                warn!("[CONFIG] ⚠️  COOKIE_SECURE not set, using default: false (INSECURE - set to true in production!)");
                "false".to_string()
            })
            .parse()
            .unwrap_or(false);

        let cookie_http_only = env::var("COOKIE_HTTP_ONLY")
            .unwrap_or_else(|_| {
                info!("[CONFIG] COOKIE_HTTP_ONLY not set, using default: true");
                "true".to_string()
            })
            .parse()
            .unwrap_or(true);

        let cookie_same_site = env::var("COOKIE_SAME_SITE")
            .unwrap_or_else(|_| {
                info!("[CONFIG] COOKIE_SAME_SITE not set, using default: Lax");
                "Lax".to_string()
            });

        let config = Config {
            jwt_secret,
            jwt_expiration_hours,
            cookie_name,
            cookie_domain,
            cookie_secure,
            cookie_http_only,
            cookie_same_site,
        };

        info!("[CONFIG] ✅ Configuration loaded successfully");
        debug!("[CONFIG] JWT expiration: {} hours", config.jwt_expiration_hours);
        debug!("[CONFIG] Cookie name: {}", config.cookie_name);
        debug!("[CONFIG] Cookie domain: {}", config.cookie_domain);
        debug!("[CONFIG] Cookie secure: {}", config.cookie_secure);
        debug!("[CONFIG] Cookie HTTP-only: {}", config.cookie_http_only);
        debug!("[CONFIG] Cookie SameSite: {}", config.cookie_same_site);

        Ok(config)
    }

    /// Validate configuration for security issues
    pub fn validate(&self) -> Result<(), String> {
        debug!("[CONFIG] Validating configuration for security issues");

        // JWT secret must be at least 32 characters for security
        if self.jwt_secret.len() < 32 {
            warn!("[CONFIG] ❌ JWT_SECRET is too short ({}  characters), must be at least 32 characters", self.jwt_secret.len());
            return Err("JWT_SECRET must be at least 32 characters for security".to_string());
        }

        // Warn if using default secret
        if self.jwt_secret.contains("change-this") || self.jwt_secret.contains("secret-key") {
            warn!("[CONFIG] ⚠️  SECURITY WARNING: JWT_SECRET appears to be a default value. Change it in production!");
        }

        // Warn if cookie is not secure in production-like settings
        if !self.cookie_secure && self.cookie_domain != "localhost" && !self.cookie_domain.starts_with("127.") {
            warn!("[CONFIG] ⚠️  SECURITY WARNING: COOKIE_SECURE=false on non-localhost domain. Set to true in production!");
        }

        // Validate SameSite value
        if !["Strict", "Lax", "None"].contains(&self.cookie_same_site.as_str()) {
            warn!("[CONFIG] ❌ Invalid COOKIE_SAME_SITE value: {}", self.cookie_same_site);
            return Err(format!("COOKIE_SAME_SITE must be 'Strict', 'Lax', or 'None', got: {}", self.cookie_same_site));
        }

        // Validate expiration hours
        if self.jwt_expiration_hours < 1 || self.jwt_expiration_hours > 720 {
            warn!("[CONFIG] ⚠️  JWT_EXPIRATION_HOURS should be between 1 and 720 (30 days), got: {}", self.jwt_expiration_hours);
        }

        info!("[CONFIG] ✅ Configuration validation passed");
        Ok(())
    }

    /// Get JWT expiration in seconds (for convenience)
    pub fn jwt_expiration_seconds(&self) -> i64 {
        self.jwt_expiration_hours * 3600
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_short_secret() {
        let config = Config {
            jwt_secret: "short".to_string(),
            jwt_expiration_hours: 24,
            cookie_name: "test".to_string(),
            cookie_domain: "localhost".to_string(),
            cookie_secure: false,
            cookie_http_only: true,
            cookie_same_site: "Strict".to_string(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_valid() {
        let config = Config {
            jwt_secret: "this-is-a-valid-secret-key-with-32-plus-characters".to_string(),
            jwt_expiration_hours: 24,
            cookie_name: "test".to_string(),
            cookie_domain: "localhost".to_string(),
            cookie_secure: false,
            cookie_http_only: true,
            cookie_same_site: "Strict".to_string(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_jwt_expiration_seconds() {
        let config = Config {
            jwt_secret: "this-is-a-valid-secret-key-with-32-plus-characters".to_string(),
            jwt_expiration_hours: 2,
            cookie_name: "test".to_string(),
            cookie_domain: "localhost".to_string(),
            cookie_secure: false,
            cookie_http_only: true,
            cookie_same_site: "Strict".to_string(),
        };

        assert_eq!(config.jwt_expiration_seconds(), 7200);
    }
}
