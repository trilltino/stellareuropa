// Database layer
pub mod database;

// HTTP handlers
pub mod handlers;

// Authentication modules
pub mod auth;
pub mod config;
pub mod middleware;
pub mod extractors;

// Re-exports
pub use database::*;
pub use handlers::*;
pub use config::{Config, AppState};
pub use auth::{encode_jwt, decode_jwt, validate_token, hash_password, verify_password};
pub use middleware::{auth_middleware, require_admin, require_chapter_lead};
pub use extractors::CurrentUser;