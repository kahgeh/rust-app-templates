//! @title Hypermedia Demo
//! @description Dynamic content loading with Datastar
//! @html_start
//! <button data-on-click="@get('/examples/elements/get-items')">
//!     Fetch Data
//! </button>
//! <div id="data-content">
//!     <p class="message">Click the button to load data</p>
//! </div>
//! @html_end

use crate::{error::AppError, templates::DataItemsTemplate, AppState};
use axum::{extract::State, http::HeaderMap, response::IntoResponse};

pub async fn get_items(
    State(_state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
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

    Ok(template.into_response())
}