use crate::settings::Settings;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&settings.telemetry.level));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_file(true) // Show file path
        .with_line_number(true); // Show line numbers

    let registry = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    // Note: OpenTelemetry integration simplified for now
    // TODO: Update to use new OpenTelemetry 0.30 API when documentation is available
    if settings.telemetry.otlp_endpoint.is_some() {
        tracing::warn!(
            "OTLP endpoint configured but OpenTelemetry integration pending update to 0.30 API"
        );
    }

    registry.init();

    // Log service metadata on startup
    tracing::info!(
        service.name = %settings.metadata.name,
        service.version = %settings.metadata.version,
        environment = %settings.application.environment,
        "Service initialized"
    );

    Ok(())
}

pub fn shutdown_telemetry() {
    // Placeholder for telemetry shutdown
}

