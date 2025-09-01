use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub metadata: Metadata,
    pub application: ApplicationSettings,
    pub server: ServerSettings,
    pub telemetry: TelemetrySettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Metadata {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    pub name: String,
    pub environment: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelemetrySettings {
    pub level: String,
    pub otlp_endpoint: Option<String>,
}

static SETTINGS: OnceLock<Settings> = OnceLock::new();

impl Settings {
    pub fn init() -> Result<&'static Settings, ConfigError> {
        if let Some(settings) = SETTINGS.get() {
            return Ok(settings);
        }

        let run_environment = env::var("RUN_ENVIRONMENT").unwrap_or_else(|_| "local".into());

        // Use CARGO_MANIFEST_DIR to find config files relative to the package
        let config_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("config");
        
        let s = Config::builder()
            // Set metadata defaults from Cargo.toml
            .set_default("metadata.name", env!("CARGO_PKG_NAME"))?
            .set_default("metadata.version", env!("CARGO_PKG_VERSION"))?
            // Start with defaults
            .add_source(File::from(config_dir.join("default.toml")))
            // Layer on the environment-specific values
            .add_source(File::from(config_dir.join(format!("{}.toml", run_environment))).required(false))
            // Add in settings from environment variables (with prefix APP)
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;

        let settings: Settings = s.try_deserialize()?;

        SETTINGS
            .set(settings)
            .map_err(|_| ConfigError::Message("Failed to set settings in OnceLock".to_string()))?;

        Ok(SETTINGS.get().unwrap())
    }

    pub fn get() -> &'static Settings {
        SETTINGS.get().expect("Settings not initialized")
    }
}

