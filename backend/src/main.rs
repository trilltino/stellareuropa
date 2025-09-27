use axum::{
    routing::{get, post},
    Router,
};
use backend::{create_pool, handlers};
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let pool = create_pool().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/api/signup", post(handlers::signup))
        .route("/api/events", post(handlers::create_event))
        .route("/api/events", get(handlers::list_events))
        .route("/health", get(|| async { "OK" }))
        .layer(CorsLayer::permissive())
        .with_state(pool);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("📡 Server running on http://127.0.0.1:8080");
    println!("🔗 API Endpoints:");
    println!("   • POST /api/signup - User registration");
    println!("   • POST /api/events - Create events with KPI planning");
    println!("   • GET  /api/events - List events");
    println!("   • GET  /health    - Health check");

    axum::serve(listener, app).await?;
    Ok(())
}