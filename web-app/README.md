# Web App

A modern Rust web application built with Axum, featuring:

## Features
- 🚀 **Axum** web framework for high-performance async HTTP
- 📦 **Askama** templating with type-safe templates
- ⚡ **Datastar** hypermedia for dynamic interactions
- 🎨 **Open Props** CSS design tokens for modern styling
- 🔧 **Configuration management** with environment-specific settings
- 📊 **OpenTelemetry** ready (simplified for v0.30)
- 🛡️ **Structured error handling** with proper HTTP responses

## Running the Application

From the workspace root:
```bash
cargo run -p web-app
```

The application will start on `http://localhost:8080`

## Project Structure
```
web-app/
├── src/
│   ├── main.rs         # Application entry point
│   ├── handlers.rs     # Request handlers
│   ├── templates.rs    # Template definitions
│   ├── error.rs        # Error handling
│   ├── settings.rs     # Configuration management
│   └── telemetry.rs    # Logging and tracing
├── templates/
│   ├── base.html       # Base template
│   ├── index.html      # Homepage
│   └── components/     # Reusable components
└── static/
    └── styles.css      # Custom styles using Open Props
```

## Configuration

Configuration files are located in the workspace root `config/` directory:
- `default.toml` - Base configuration
- `development.toml` - Development overrides
- `production.toml` - Production overrides

## API Endpoints

- `GET /` - Homepage with interactive demos
- `GET /health` - Health check endpoint
- `GET /api/data` - Sample data API
- `POST /api/submit` - Form submission endpoint