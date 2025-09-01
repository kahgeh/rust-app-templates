# Rust App Templates

A modern Rust web application starter template featuring:
- **Axum 0.8** - Fast and ergonomic web framework
- **Askama 0.14** - Type-safe, compiled templates with axum integration via askama_web  
- **Datastar 0.3** - Hypermedia-driven interactions
- **Open Props** - CSS custom properties for consistent design
- **config-rs 0.15** - Layered configuration management
- **OpenTelemetry 0.30** - Distributed tracing and observability

## Features

✅ Type-safe HTML templating with Askama  
✅ Server-side rendering with hypermedia patterns  
✅ Real-time updates via Server-Sent Events (SSE)  
✅ Environment-based configuration  
✅ OpenTelemetry instrumentation  
✅ Modern CSS with Open Props design tokens  
✅ Error handling with custom error types  
✅ Static file serving  

## Quick Start

### Prerequisites

- Rust 1.75 or later
- Cargo

### Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd rust-app-templates
```

2. Build the project:
```bash
cargo build
```

3. Run the development server:
```bash
cargo run
```

The server will start on `http://localhost:8080` by default.

## Project Structure

```
rust-app-templates/
├── web-app/
│   ├── src/
│   │   ├── main.rs           # Application entry point
│   │   ├── settings.rs       # Configuration management with OnceLock
│   │   ├── telemetry.rs      # Telemetry setup with service metadata
│   │   ├── error.rs          # Error handling
│   │   ├── handlers.rs       # Request handlers (hypermedia endpoints)
│   │   └── templates.rs      # Askama template definitions
│   ├── templates/
│   │   ├── base.html         # Base layout template with Datastar
│   │   ├── index.html        # Homepage template
│   │   ├── components/       # Reusable template components
│   │   │   └── card.html     # Card component
│   │   └── fragments/        # HTML fragments for hypermedia responses
│   │       ├── data_items.html
│   │       └── form_response.html
│   ├── static/
│   │   └── styles.css        # Custom CSS with Open Props
│   ├── config/
│   │   ├── default.toml      # Default configuration
│   │   └── local.toml        # Local development overrides
│   ├── Cargo.toml            # Package dependencies
│   └── README.md             # Web app specific documentation
├── Cargo.toml                # Workspace configuration
├── Cargo.lock                # Dependency lock file
├── README.md                 # This file
├── build.sh                  # Build script
└── run.sh                    # Run script
```

## Configuration

The application uses a layered configuration approach:

1. **Default Configuration** (`config/default.toml`)
2. **Environment-specific** (`config/{environment}.toml`)
3. **Environment Variables** (with `APP__` prefix)

### Configuration Options

```toml
[server]
host = "0.0.0.0"
port = 8080

[telemetry]
level = "info"
otlp_endpoint = "http://localhost:4317"

[application]
name = "Rust Web Starter"
environment = "development"

# Metadata is auto-populated from Cargo.toml
# [metadata]
# name = env!("CARGO_PKG_NAME")
# version = env!("CARGO_PKG_VERSION")
```

### Environment Variables

Override configuration using environment variables:

```bash
# Server configuration
export APP_SERVER_PORT=3000
export APP_SERVER_HOST="0.0.0.0"

# Telemetry configuration
export APP_TELEMETRY_LEVEL="debug"
export APP_TELEMETRY_OTLP_ENDPOINT="http://otel-collector:4317"

# Application configuration
export APP_APPLICATION_NAME="My App"
export APP_APPLICATION_ENVIRONMENT="production"

# Run environment (determines which config file to load)
export RUN_ENVIRONMENT=local  # loads config/local.toml
```

## Development

### Running in Development Mode

```bash
# From the workspace root
cargo run --bin web-app

# From the web-app directory
cd web-app && cargo run

# With specific environment
RUN_ENVIRONMENT=local cargo run --bin web-app

# With debug logging
RUST_LOG=debug cargo run --bin web-app
```

### Building for Production

```bash
# Build optimized binary
cargo build --release

# Run production build
RUN_ENVIRONMENT=production ./target/release/rust-app-templates
```

## API Endpoints

- `GET /` - Homepage with interactive hypermedia examples
- `GET /health` - Health check endpoint (returns JSON)
- `GET /get-items` - Fetch data items (returns HTML fragment, requires Datastar header)
- `POST /submit-form` - Submit form data (accepts form-encoded data, returns HTML fragment)

## Templating with Askama

Templates are defined in `src/templates.rs` and rendered from `templates/` directory:

```rust
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub title: &'a str,
    pub environment: &'a str,
}
```

## Hypermedia with Datastar

Datastar enables hypermedia-driven interactions without writing JavaScript:

```html
<!-- Click handler -->
<button data-on-click="$counter++">Increment</button>

<!-- Display reactive value -->
<span data-text="$counter"></span>

<!-- Server interaction -->
<button data-on-click="@get('/api/data')">Fetch Data</button>

<!-- SSE subscription -->
<div data-subscribe-sse="/sse">
    <!-- Content updated via SSE -->
</div>
```

## Styling with Open Props

The application uses Open Props for consistent design tokens:

```css
/* Using Open Props variables */
.card {
    background: var(--surface-1);
    border-radius: var(--radius-3);
    padding: var(--size-6);
    box-shadow: var(--shadow-2);
}
```

## Observability

### OpenTelemetry Setup

Enable telemetry in configuration:

```toml
[telemetry]
enabled = true
endpoint = "http://localhost:4317"
service_name = "my-service"
```

### Running with Jaeger

```bash
# Start Jaeger
docker run -d --name jaeger \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  jaegertracing/all-in-one:latest

# Run application with telemetry
APP__TELEMETRY__ENABLED=true cargo run

# View traces at http://localhost:16686
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Docker Support

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-app-templates /app/
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static
COPY --from=builder /app/config /app/config
EXPOSE 8080
CMD ["./rust-app-templates"]
```

Build and run:

```bash
docker build -t rust-app-templates .
docker run -p 8080:8080 rust-app-templates
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## License

MIT License - see LICENSE file for details