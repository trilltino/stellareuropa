use crate::database::connection::DbPool;
use crate::database::repositories::UserRepository;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use tower_cookies::Cookies;
use tracing::{info, error, warn, debug};
use shared::dto::{
    SignUpRequest, SignUpResponse, UserPublic, UserType,
    SignupRequest, LoginRequest, AuthResponse, UserInfo, ErrorResponse,
    Guest,
};
use crate::{
    database::models::User,
    auth::{hash_password, verify_password, encode_jwt},
    config::Config,
    extractors::CurrentUser,
};

// ===== LEGACY HANDLERS (Wallet-based auth) =====

fn create_user_public(user: &User) -> UserPublic {
    let user_type = match user.user_type.as_str() {
        "Ambassador" => UserType::Ambassador,
        "ChapterLead" => UserType::ChapterLead,
        _ => UserType::Ambassador,
    };

    UserPublic {
        id: user.id.to_string(),
        username: user.username.clone(),
        email: user.email.clone(),
        wallet_address: user.wallet_address.clone().unwrap_or_default(),
        user_type,
        organization: user.organization.clone(),
        bio: user.bio.clone(),
        created_at: user.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
    }
}

fn create_error_user_public() -> UserPublic {
    UserPublic {
        id: "error".to_string(),
        username: "Error".to_string(),
        email: "".to_string(),
        wallet_address: "".to_string(),
        user_type: UserType::Ambassador,
        organization: None,
        bio: None,
        created_at: "".to_string(),
    }
}

/// Legacy signup handler (wallet-based authentication)
/// Kept for backward compatibility
pub async fn signup(
    State(pool): State<DbPool>,
    Json(req): Json<SignUpRequest>,
) -> (StatusCode, Json<SignUpResponse>) {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("[SIGNUP] 📝 LEGACY WALLET SIGNUP REQUEST");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    debug!("   Username: {}", req.username);
    debug!("   Email: {}", req.email);
    debug!("   Wallet Address: {}", req.wallet_address);
    debug!("   User Type: {:?}", req.user_type);
    debug!("   Organization: {:?}", req.organization);
    debug!("   Bio: {:?}", req.bio);

    // Check if user already exists by email or wallet address
    match UserRepository::find_by_email(&pool, &req.email).await {
        Ok(Some(_)) => {
            warn!("[SIGNUP] ❌ Email already exists: {}", req.email);
            let resp = SignUpResponse {
                user: create_error_user_public(),
                message: "User with this email already exists".to_string(),
            };
            return (StatusCode::CONFLICT, Json(resp));
        }
        Ok(None) => {}
        Err(e) => {
            error!("[SIGNUP] ❌ Database error checking email: {}", e);
            let resp = SignUpResponse {
                user: create_error_user_public(),
                message: format!("Database error: {e}"),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
        }
    }

    match UserRepository::find_by_wallet_address(&pool, &req.wallet_address).await {
        Ok(Some(_)) => {
            warn!("[SIGNUP] ❌ Wallet address already exists: {}", req.wallet_address);
            let resp = SignUpResponse {
                user: create_error_user_public(),
                message: "User with this wallet address already exists".to_string(),
            };
            (StatusCode::CONFLICT, Json(resp))
        }
        Ok(None) => {
            let user_type_str = req.user_type.to_string();
            match UserRepository::create_user(
                &pool,
                &req.username,
                &req.email,
                &req.wallet_address,
                &user_type_str,
                req.organization.as_deref(),
                req.bio.as_deref(),
            ).await {
                Ok(db_user) => {
                    info!("[SIGNUP] ✅ User created successfully!");
                    info!("   User ID: {}", db_user.id);
                    info!("   Username: {}", db_user.username);
                    info!("   Email: {}", db_user.email);
                    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

                    let user_public = create_user_public(&db_user);
                    let resp = SignUpResponse {
                        user: user_public,
                        message: "User created successfully!".to_string(),
                    };
                    (StatusCode::CREATED, Json(resp))
                }
                Err(e) => {
                    error!("[SIGNUP] ❌ Database error creating user: {}", e);
                    let resp = SignUpResponse {
                        user: create_error_user_public(),
                        message: format!("Failed to create user: {e}"),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
                }
            }
        }
        Err(e) => {
            error!("[SIGNUP] ❌ Database error: {}", e);
            let resp = SignUpResponse {
                user: create_error_user_public(),
                message: format!("Database error: {e}"),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(resp))
        }
    }
}

// ===== NEW AUTH HANDLERS (Password-based JWT authentication) =====

/// New signup with password authentication
pub async fn signup_with_password(
    State(pool): State<DbPool>,
    State(config): State<Config>,
    cookies: Cookies,
    Json(req): Json<SignupRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<ErrorResponse>)> {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("[SIGNUP] 🔐 NEW PASSWORD SIGNUP REQUEST");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    debug!("   Username: {}", req.username);
    debug!("   Email: {}", req.email);
    debug!("   Role: {}", req.role);
    debug!("   Wallet: {:?}", req.wallet_address);

    // Validate role
    if req.role != "visitor" && req.role != "chapter_lead" {
        warn!("[SIGNUP] ❌ Invalid role: {}", req.role);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: "Invalid role. Must be 'visitor' or 'chapter_lead'".to_string() })
        ));
    }

    // Check if email already exists
    match UserRepository::find_by_email(&pool, &req.email).await {
        Ok(Some(_)) => {
            warn!("[SIGNUP] ❌ Email already registered: {}", req.email);
            return Err((
                StatusCode::CONFLICT,
                Json(ErrorResponse { error: "Email already registered".to_string() })
            ));
        }
        Ok(None) => {}
        Err(e) => {
            error!("[SIGNUP] ❌ Database error checking email: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Database error".to_string() })
            ));
        }
    }

    // Check if username already exists
    match UserRepository::find_by_username(&pool, &req.username).await {
        Ok(Some(_)) => {
            warn!("[SIGNUP] ❌ Username already taken: {}", req.username);
            return Err((
                StatusCode::CONFLICT,
                Json(ErrorResponse { error: "Username already taken".to_string() })
            ));
        }
        Ok(None) => {}
        Err(e) => {
            error!("[SIGNUP] ❌ Database error checking username: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Database error".to_string() })
            ));
        }
    }

    // Hash password
    debug!("[SIGNUP] Hashing password...");
    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(e) => {
            error!("[SIGNUP] ❌ Password hashing failed: {}", e);
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Password must be at least 8 characters with uppercase, lowercase, and numbers".to_string()
                })
            ));
        }
    };

    // Create user
    debug!("[SIGNUP] Creating user in database...");
    let user = match UserRepository::create_user_with_auth(
        &pool,
        &req.username,
        &req.email,
        &password_hash,
        &req.role,
        req.wallet_address.as_deref(),
    ).await {
        Ok(user) => user,
        Err(e) => {
            error!("[SIGNUP] ❌ Failed to create user: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Failed to create user".to_string() })
            ));
        }
    };

    // Generate JWT
    debug!("[SIGNUP] Generating JWT token...");
    let token = match encode_jwt(
        user.id,
        user.username.clone(),
        user.role.clone(),
        &config.jwt_secret,
        config.jwt_expiration_hours,
    ) {
        Ok(token) => token,
        Err(e) => {
            error!("[SIGNUP] ❌ JWT encoding failed: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Failed to generate token".to_string() })
            ));
        }
    };

    // Set cookie
    debug!("[SIGNUP] Setting authentication cookie...");
    let cookie = crate::auth::create_auth_cookie(token, &config);
    cookies.add(cookie);

    info!("[SIGNUP] ✅ User created and authenticated!");
    info!("   User ID: {}", user.id);
    info!("   Username: {}", user.username);
    info!("   Role: {}", user.role);
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            user: UserInfo {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
                role: user.role,
                wallet_address: user.wallet_address,
                created_at: user.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
            },
            message: "Signup successful".to_string(),
        })
    ))
}

/// Login handler
pub async fn login(
    State(pool): State<DbPool>,
    State(config): State<Config>,
    cookies: Cookies,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<ErrorResponse>)> {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("[LOGIN] 🔓 LOGIN ATTEMPT");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    debug!("   Identifier: {}", req.email_or_username);

    // Find user by email or username
    let user = if req.email_or_username.contains('@') {
        debug!("[LOGIN] Looking up by email...");
        UserRepository::find_by_email(&pool, &req.email_or_username).await
    } else {
        debug!("[LOGIN] Looking up by username...");
        UserRepository::find_by_username(&pool, &req.email_or_username).await
    };

    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!("[LOGIN] ❌ User not found: {}", req.email_or_username);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse { error: "Invalid credentials".to_string() })
            ));
        }
        Err(e) => {
            error!("[LOGIN] ❌ Database error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Database error".to_string() })
            ));
        }
    };

    // Check if user is active
    if !user.is_active {
        warn!("[LOGIN] ❌ Account deactivated: {}", user.username);
        return Err((
            StatusCode::FORBIDDEN,
            Json(ErrorResponse { error: "Account is deactivated".to_string() })
        ));
    }

    // Verify password
    debug!("[LOGIN] Verifying password...");
    let password_hash = match &user.password_hash {
        Some(hash) => hash,
        None => {
            warn!("[LOGIN] ❌ No password set for user: {}", user.username);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse { error: "Invalid credentials".to_string() })
            ));
        }
    };

    let is_valid = match verify_password(&req.password, password_hash) {
        Ok(valid) => valid,
        Err(e) => {
            error!("[LOGIN] ❌ Password verification error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Authentication error".to_string() })
            ));
        }
    };

    if !is_valid {
        warn!("[LOGIN] ❌ Invalid password for user: {}", user.username);
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse { error: "Invalid credentials".to_string() })
        ));
    }

    // Update last login
    debug!("[LOGIN] Updating last login timestamp...");
    let _ = UserRepository::update_last_login(&pool, user.id).await;

    // Generate JWT
    debug!("[LOGIN] Generating JWT token...");
    let token = match encode_jwt(
        user.id,
        user.username.clone(),
        user.role.clone(),
        &config.jwt_secret,
        config.jwt_expiration_hours,
    ) {
        Ok(token) => token,
        Err(e) => {
            error!("[LOGIN] ❌ JWT encoding failed: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Failed to generate token".to_string() })
            ));
        }
    };

    // Set cookie
    debug!("[LOGIN] Setting authentication cookie...");
    let cookie = crate::auth::create_auth_cookie(token, &config);
    cookies.add(cookie);

    info!("[LOGIN] ✅ User authenticated successfully!");
    info!("   User ID: {}", user.id);
    info!("   Username: {}", user.username);
    info!("   Role: {}", user.role);
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok((
        StatusCode::OK,
        Json(AuthResponse {
            user: UserInfo {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
                role: user.role,
                wallet_address: user.wallet_address,
                created_at: user.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
            },
            message: "Login successful".to_string(),
        })
    ))
}

/// Logout handler
pub async fn logout(
    State(config): State<Config>,
    cookies: Cookies,
) -> impl IntoResponse {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("[LOGOUT] 👋 LOGOUT REQUEST");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    debug!("[LOGOUT] Creating logout cookie...");
    let cookie = crate::auth::create_logout_cookie(&config);
    cookies.add(cookie);

    info!("[LOGOUT] ✅ User logged out successfully");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    (StatusCode::OK, Json(serde_json::json!({ "message": "Logout successful" })))
}

/// Get current authenticated user info
pub async fn me(
    CurrentUser(user): CurrentUser,
) -> impl IntoResponse {
    debug!("[ME] Current user request for: {}", user.username);

    Json(UserInfo {
        id: user.user_id.to_string(),
        username: user.username,
        email: String::new(), // Don't expose email in this endpoint for security
        role: user.role,
        wallet_address: None, // Don't expose wallet in this endpoint for security
        created_at: String::new(),
    })
}

/// Freighter wallet login/register handler
/// Similar to yew-scaffold pattern - creates account if doesn't exist, logs in if it does
pub async fn wallet_login(
    State(pool): State<DbPool>,
    State(config): State<Config>,
    cookies: Cookies,
    Json(guest): Json<Guest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<ErrorResponse>)> {
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("[WALLET LOGIN] 👛 FREIGHTER WALLET AUTH");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    debug!("   Username: {}", guest.username);
    debug!("   Wallet: {}...{}",
           &guest.wallet_address[..6],
           &guest.wallet_address[guest.wallet_address.len()-6..]);

    // Check if user already exists by wallet address
    match UserRepository::find_by_wallet_address(&pool, &guest.wallet_address).await {
        Ok(Some(existing_user)) => {
            info!("[WALLET LOGIN] Found existing user - id={}, username={}",
                  existing_user.id, existing_user.username);

            // Check if user is active
            if !existing_user.is_active {
                warn!("[WALLET LOGIN] ❌ Account deactivated: {}", existing_user.username);
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ErrorResponse { error: "Account is deactivated".to_string() })
                ));
            }

            // Update username if different
            let user = if existing_user.username != guest.username {
                info!("[WALLET LOGIN] Username changed from '{}' to '{}', updating...",
                      existing_user.username, guest.username);
                match UserRepository::update_username_by_wallet(&pool, &guest.wallet_address, &guest.username).await {
                    Ok(updated_user) => {
                        info!("[WALLET LOGIN] Username updated successfully");
                        updated_user
                    },
                    Err(e) => {
                        error!("[WALLET LOGIN] Failed to update username: {:?}", e);
                        existing_user // Fall back to existing user
                    }
                }
            } else {
                existing_user
            };

            // Update last login
            let _ = UserRepository::update_last_login(&pool, user.id).await;

            // Generate JWT
            debug!("[WALLET LOGIN] Generating JWT token...");
            let token = match encode_jwt(
                user.id,
                user.username.clone(),
                user.role.clone(),
                &config.jwt_secret,
                config.jwt_expiration_hours,
            ) {
                Ok(token) => token,
                Err(e) => {
                    error!("[WALLET LOGIN] ❌ JWT encoding failed: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse { error: "Failed to generate token".to_string() })
                    ));
                }
            };

            // Set cookie
            let cookie = crate::auth::create_auth_cookie(token, &config);
            cookies.add(cookie);

            info!("[WALLET LOGIN] ✅ Existing user logged in!");
            info!("   User ID: {}", user.id);
            info!("   Username: {}", user.username);
            info!("   Role: {}", user.role);
            info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            Ok((
                StatusCode::OK,
                Json(AuthResponse {
                    user: UserInfo {
                        id: user.id.to_string(),
                        username: user.username,
                        email: user.email,
                        role: user.role,
                        wallet_address: Some(guest.wallet_address),
                        created_at: user.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
                    },
                    message: "Welcome back! Login successful.".to_string(),
                })
            ))
        },
        Ok(None) => {
            info!("[WALLET LOGIN] No existing user found, creating new account...");

            // Create new user with wallet authentication
            // Use a default email based on wallet address
            let default_email = format!("{}@wallet.stellar-europe.org", &guest.wallet_address[..8]);

            let user = match UserRepository::create_user_with_wallet(
                &pool,
                &guest.username,
                &default_email,
                &guest.wallet_address,
                "visitor", // Default role for wallet users
            ).await {
                Ok(user) => user,
                Err(e) => {
                    error!("[WALLET LOGIN] ❌ Failed to create user: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse { error: "Failed to create account".to_string() })
                    ));
                }
            };

            // Generate JWT
            debug!("[WALLET LOGIN] Generating JWT token...");
            let token = match encode_jwt(
                user.id,
                user.username.clone(),
                user.role.clone(),
                &config.jwt_secret,
                config.jwt_expiration_hours,
            ) {
                Ok(token) => token,
                Err(e) => {
                    error!("[WALLET LOGIN] ❌ JWT encoding failed: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse { error: "Failed to generate token".to_string() })
                    ));
                }
            };

            // Set cookie
            let cookie = crate::auth::create_auth_cookie(token, &config);
            cookies.add(cookie);

            info!("[WALLET LOGIN] ✅ New user created and logged in!");
            info!("   User ID: {}", user.id);
            info!("   Username: {}", user.username);
            info!("   Role: {}", user.role);
            info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            Ok((
                StatusCode::CREATED,
                Json(AuthResponse {
                    user: UserInfo {
                        id: user.id.to_string(),
                        username: user.username,
                        email: user.email,
                        role: user.role,
                        wallet_address: Some(guest.wallet_address),
                        created_at: user.created_at.map_or("Unknown".to_string(), |dt| dt.to_string()),
                    },
                    message: "Account created successfully! Welcome.".to_string(),
                })
            ))
        },
        Err(e) => {
            error!("[WALLET LOGIN] ❌ Database error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: "Database error".to_string() })
            ))
        }
    }
}
