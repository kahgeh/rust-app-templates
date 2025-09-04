//! @title Form Demo
//! @description Form submission with hypermedia responses
//! @html_start
//! <form data-on-submit="@post('/examples/elements/submit-form', {contentType: 'form'})">
//!     <div class="form-group">
//!         <label for="name">Name:</label>
//!         <input type="text" id="name" name="name" required>
//!     </div>
//!     <div class="form-group">
//!         <label for="value">Value:</label>
//!         <input type="text" id="value" name="value" required>
//!     </div>
//!     <button type="submit">Submit</button>
//! </form>
//! <div id="form-response">
//!     <!-- Response will be displayed here -->
//! </div>
//! @html_end

use crate::{error::AppError, templates::FormResponseTemplate, AppState};
use axum::{extract::State, http::HeaderMap, response::IntoResponse, Form};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubmitRequest {
    pub name: String,
    pub value: String,
}

pub async fn submit_form(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Form(payload): Form<SubmitRequest>,
) -> Result<impl IntoResponse, AppError> {
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