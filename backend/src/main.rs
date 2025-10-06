use axum::{
    routing::{get, post, patch},
    Router,
    middleware,
};
use backend::{create_pool, handlers, Config, AppState, auth_middleware, require_chapter_lead};
use tower_http::cors::CorsLayer;
use tower_cookies::CookieManagerLayer;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("═══════════════════════════════════════════════════════════");
    info!("🚀 STELLAR EUROPE BACKEND STARTING");
    info!("═══════════════════════════════════════════════════════════");

    // Load environment variables FIRST
    dotenvy::dotenv().ok();

    // Load and validate configuration
    info!("📋 Loading configuration...");
    let config = Config::from_env().map_err(|e| anyhow::anyhow!(e))?;
    config.validate().map_err(|e| anyhow::anyhow!(e))?;

    // Create database pool
    info!("🗄️  Connecting to database...");
    let pool = create_pool().await?;

    // Run migrations
    info!("⚙️  Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("✅ Migrations complete");

    // Create combined app state
    let app_state = AppState { pool, config: config.clone() };

    // Configure CORS (CRITICAL: must allow credentials for cookies!)
    use axum::http::{HeaderValue, Method};

    const ALLOWED_ORIGINS: &[&str] = &[
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8000",
        "http://127.0.0.1:8000",
    ];

    let origins: Vec<HeaderValue> = ALLOWED_ORIGINS
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true); // ESSENTIAL for cookie-based auth!

    // ===== PUBLIC ROUTES (No authentication required) =====
    let public_routes = Router::new()
        // Legacy wallet-based signup
        .route("/api/signup", post(handlers::signup))

        // New password-based authentication
        .route("/api/auth/signup", post(handlers::signup_with_password))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/wallet", post(handlers::wallet_login))
        .route("/api/auth/logout", post(handlers::logout))

        // Health check
        .route("/health", get(|| async { "OK" }));

    // ===== PROTECTED ROUTES (Authentication required) =====
    let protected_routes = Router::new()
        .route("/api/auth/me", get(handlers::me))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    // ===== CHAPTER LEAD ROUTES (chapter_lead or admin role required) =====
    let chapter_lead_routes = Router::new()
        .route("/api/events", post(handlers::create_event))
        .route("/api/events", get(handlers::list_events))
        .route("/api/events/{id}/kpi", patch(handlers::update_event_kpi))
        .route("/api/events/{id}/post-event", patch(handlers::update_post_event_data))
        .route("/api/scf-projects", post(handlers::create_project))
        .route("/api/scf-projects", get(handlers::list_projects))
        .route("/api/scf-projects/{id}", get(handlers::get_project))
        .route("/api/scf-projects/{id}/status", patch(handlers::update_project_status))
        .route_layer(middleware::from_fn(require_chapter_lead))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    // ===== ADMIN ROUTES (admin role required) =====
    // Future: Add admin-only routes here
    // let admin_routes = Router::new()
    //     .route("/api/admin/users", get(handlers::admin::list_users))
    //     .route_layer(middleware::from_fn(require_admin))
    //     .route_layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));

    // ===== COMBINE ALL ROUTES =====
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(chapter_lead_routes)
        // .merge(admin_routes) // Uncomment when admin routes are added
        .with_state(app_state)
        .layer(CookieManagerLayer::new()) // MUST be added AFTER routes
        .layer(cors);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    info!("═══════════════════════════════════════════════════════════");
    info!("✅ SERVER READY ON http://127.0.0.1:8080");
    info!("═══════════════════════════════════════════════════════════");
    info!("");
    info!("📡 API ENDPOINTS:");
    info!("");
    info!("🔓 PUBLIC (No Auth):");
    info!("   • POST  /api/signup - Legacy wallet signup");
    info!("   • POST  /api/auth/signup - New password signup");
    info!("   • POST  /api/auth/login - Login (email/password)");
    info!("   • POST  /api/auth/wallet - Login (Freighter wallet)");
    info!("   • POST  /api/auth/logout - Logout");
    info!("   • GET   /health - Health check");
    info!("");
    info!("🔐 PROTECTED (Auth Required):");
    info!("   • GET   /api/auth/me - Get current user");
    info!("");
    info!("📋 CHAPTER LEAD (chapter_lead or admin):");
    info!("   • POST  /api/events - Create event");
    info!("   • GET   /api/events - List events");
    info!("   • PATCH /api/events/{{id}}/kpi - Update event KPI");
    info!("   • POST  /api/scf-projects - Create SCF project");
    info!("   • GET   /api/scf-projects - List SCF projects");
    info!("   • GET   /api/scf-projects/{{id}} - Get SCF project");
    info!("   • PATCH /api/scf-projects/{{id}}/status - Update project status");
    info!("");
    info!("═══════════════════════════════════════════════════════════");

    axum::serve(listener, app).await?;
    Ok(())
}
