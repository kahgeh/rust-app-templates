use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod error;
mod examples;
mod examples_gen;
mod handlers;
mod settings;
mod syntax_highlight;
mod telemetry;
mod templates;
mod theme;

use settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize configuration
    let settings = Settings::init()?;
    tracing::info!("Configuration loaded");

    // Initialize telemetry
    telemetry::init_telemetry(settings)?;
    tracing::info!("Telemetry initialized");

    // Create application state
    let app_state = AppState {
        settings: Arc::new(settings.clone()),
    };

    // Build router
    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/examples", get(handlers::examples))
        .route("/health", get(health))
        .route("/examples/elements/get-items", get(handlers::get_items))
        .route("/examples/elements/submit-form", post(handlers::submit_form))
        .route("/examples/search", get(handlers::search))
        .route("/examples/theme/switch", get(handlers::switch_theme))
        .route("/examples/code/{example_id}", get(handlers::get_example_code))
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server.port));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Set up graceful shutdown
    let handle = axum_server::Handle::new();
    let shutdown_handle = handle.clone();

    // Spawn task to listen for shutdown signal
    tokio::spawn(async move {
        shutdown_signal().await;
        tracing::info!("Shutdown signal received, starting graceful shutdown...");
        shutdown_handle.graceful_shutdown(Some(std::time::Duration::from_secs(30)));
    });

    // Run server with graceful shutdown
    axum_server::from_tcp(listener.into_std()?)
        .handle(handle)
        .serve(app.into_make_service())
        .await?;

    tracing::info!("Server shut down gracefully");
    telemetry::shutdown_telemetry();

    Ok(())
}

