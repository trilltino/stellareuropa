use crate::database::models::User;
use crate::database::connection::DbPool;
use sqlx::{Error as SqlxError};
use tracing::{debug, info};

pub struct UserRepository;

impl UserRepository {
    /// Legacy: Create user with wallet (for backward compatibility)
    pub async fn create_user(
        pool: &DbPool,
        username: &str,
        email: &str,
        wallet_address: &str,
        user_type: &str,
        organization: Option<&str>,
        bio: Option<&str>,
    ) -> Result<User, SqlxError> {
        let role = match user_type {
            "ChapterLead" => "chapter_lead",
            _ => "visitor",
        };

        debug!("[REPO] Creating legacy user with wallet: {}", username);

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, wallet_address, user_type, role, organization, bio, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            RETURNING
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            "#,
            username,
            email,
            Some(wallet_address),
            user_type,
            role,
            organization,
            bio
        )
        .fetch_one(pool)
        .await?;

        info!("[REPO] ✅ User created with wallet: {} (ID: {})", username, user.id);
        Ok(user)
    }

    /// Create user with password authentication
    pub async fn create_user_with_auth(
        pool: &DbPool,
        username: &str,
        email: &str,
        password_hash: &str,
        role: &str,
        wallet_address: Option<&str>,
    ) -> Result<User, SqlxError> {
        debug!("[REPO] Creating user with auth: {} (role: {})", username, role);

        let user_type = match role {
            "chapter_lead" => "ChapterLead",
            _ => "Ambassador",
        };

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, role, user_type, wallet_address, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            "#,
            username,
            email,
            password_hash,
            role,
            user_type,
            wallet_address
        )
        .fetch_one(pool)
        .await?;

        info!("[REPO] ✅ User created with password auth: {} (ID: {}, role: {})", username, user.id, role);
        Ok(user)
    }

    pub async fn find_by_email(pool: &DbPool, email: &str) -> Result<Option<User>, SqlxError> {
        debug!("[REPO] Finding user by email: {}", email);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        if let Some(ref u) = user {
            debug!("[REPO] ✅ User found by email: {} (ID: {})", email, u.id);
        } else {
            debug!("[REPO] User not found by email: {}", email);
        }

        Ok(user)
    }

    pub async fn find_by_username(pool: &DbPool, username: &str) -> Result<Option<User>, SqlxError> {
        debug!("[REPO] Finding user by username: {}", username);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            FROM users WHERE username = $1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        if let Some(ref u) = user {
            debug!("[REPO] ✅ User found by username: {} (ID: {})", username, u.id);
        } else {
            debug!("[REPO] User not found by username: {}", username);
        }

        Ok(user)
    }

    pub async fn find_by_wallet_address(pool: &DbPool, wallet_address: &str) -> Result<Option<User>, SqlxError> {
        debug!("[REPO] Finding user by wallet address: {}", wallet_address);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            FROM users WHERE wallet_address = $1
            "#,
            Some(wallet_address)
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            debug!("[REPO] ✅ User found by wallet: {}", wallet_address);
        } else {
            debug!("[REPO] User not found by wallet: {}", wallet_address);
        }

        Ok(user)
    }

    pub async fn find_by_id(pool: &DbPool, user_id: i32) -> Result<Option<User>, SqlxError> {
        debug!("[REPO] Finding user by ID: {}", user_id);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        if user.is_some() {
            debug!("[REPO] ✅ User found by ID: {}", user_id);
        } else {
            debug!("[REPO] User not found by ID: {}", user_id);
        }

        Ok(user)
    }

    /// Batch find users by IDs (prevents N+1 query problem)
    ///
    /// Example: Instead of 100 individual queries for 100 events' organizers,
    /// this makes 1 batch query for all unique organizer IDs.
    pub async fn find_by_ids(pool: &DbPool, user_ids: &[i32]) -> Result<Vec<User>, SqlxError> {
        if user_ids.is_empty() {
            debug!("[REPO] find_by_ids called with empty list, returning empty vec");
            return Ok(Vec::new());
        }

        debug!("[REPO] Finding {} users by IDs (batch)", user_ids.len());

        let users = sqlx::query_as!(
            User,
            r#"
            SELECT
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            FROM users
            WHERE id = ANY($1)
            "#,
            user_ids
        )
        .fetch_all(pool)
        .await?;

        info!("[REPO] ✅ Found {} users in batch query", users.len());
        Ok(users)
    }

    /// Update user's last login timestamp
    pub async fn update_last_login(pool: &DbPool, user_id: i32) -> Result<(), SqlxError> {
        debug!("[REPO] Updating last login for user ID: {}", user_id);

        sqlx::query!(
            "UPDATE users SET last_login = NOW(), updated_at = NOW() WHERE id = $1",
            user_id
        )
        .execute(pool)
        .await?;

        info!("[REPO] ✅ Last login updated for user ID: {}", user_id);
        Ok(())
    }

    /// Update username for a wallet address (used in wallet login)
    pub async fn update_username_by_wallet(
        pool: &DbPool,
        wallet_address: &str,
        new_username: &str,
    ) -> Result<User, SqlxError> {
        debug!("[REPO] Updating username for wallet: {}...{} to '{}'",
               &wallet_address[..6], &wallet_address[wallet_address.len()-6..], new_username);

        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET username = $1, updated_at = NOW()
            WHERE wallet_address = $2
            RETURNING
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            "#,
            new_username,
            wallet_address
        )
        .fetch_one(pool)
        .await?;

        info!("[REPO] ✅ Username updated successfully for wallet user");
        Ok(user)
    }

    /// Create user with wallet authentication only (for Freighter login)
    pub async fn create_user_with_wallet(
        pool: &DbPool,
        username: &str,
        email: &str,
        wallet_address: &str,
        role: &str,
    ) -> Result<User, SqlxError> {
        info!("[REPO] Creating wallet-only user:");
        debug!("   Username: {}", username);
        debug!("   Email: {}", email);
        debug!("   Wallet: {}...{}", &wallet_address[..6], &wallet_address[wallet_address.len()-6..]);
        debug!("   Role: {}", role);

        let user_type = match role {
            "chapter_lead" => "ChapterLead",
            _ => "Ambassador",
        };

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                username, email, wallet_address,
                user_type, role, password_hash,
                email_verified, is_active,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, NULL, FALSE, TRUE, NOW(), NOW())
            RETURNING
                id, username, email,
                wallet_address, user_type,
                password_hash, role,
                email_verified as "email_verified!",
                is_active as "is_active!",
                last_login, organization, bio,
                created_at, updated_at
            "#,
            username,
            email,
            wallet_address,
            user_type,
            role
        )
        .fetch_one(pool)
        .await?;

        info!("[REPO] ✅ Wallet user created successfully - ID: {}", user.id);
        Ok(user)
    }
}