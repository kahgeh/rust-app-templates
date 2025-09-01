use crate::{
    error::AppError,
    templates::{DataItemsTemplate, FormResponseTemplate, IndexTemplate},
    AppState,
};
use axum::{
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubmitRequest {
    pub name: String,
    pub value: String,
}

// Main index page handler
pub async fn index(State(state): State<AppState>) -> IndexTemplate {
    IndexTemplate {
        title: state.settings.application.name.clone(),
        environment: state.settings.application.environment.clone(),
    }
}

// Hypermedia endpoint to get data - returns HTML fragment
pub async fn get_items(
    State(_state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Only serve fragments for Datastar requests
    if headers.get("datastar-request").is_none() {
        return Err(AppError::not_found(
            "This endpoint only serves HTML fragments",
        ));
    }

    let template = DataItemsTemplate {
        message: "Hello from Rust Web Starter!".to_string(),
        timestamp: chrono::Utc::now(),
        items: vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
            "Item 3".to_string(),
        ],
    };

    // Return HTML fragment for DOM patching
    Ok(template.into_response())
}

// Hypermedia endpoint to process form data - returns HTML fragment
pub async fn submit_form(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Form(payload): Form<SubmitRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    if payload.name.is_empty() {
        return Err(AppError::bad_request("Name cannot be empty"));
    }
    if headers.get("datastar-request").is_none() {
        return Err(AppError::not_found(
            "This endpoint only serves HTML fragments",
        ));
    }

    tracing::info!("Received submission: {} = {}", payload.name, payload.value);

    let template = FormResponseTemplate {
        message: format!("Successfully processed: {}", payload.name),
    };

    let response = template.into_response();
    Ok(response)
}
