use crate::{
    error::AppError,
    templates::{
        DataItemsTemplate, ExamplesTemplate, FormResponseTemplate, IndexTemplate,
        SearchResultsTemplate,
    },
    theme::{get_theme_variables, Theme},
    AppState,
};
use axum::{
    extract::{Query, State},
    http::{header, HeaderMap},
    response::{Html, IntoResponse, Response},
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubmitRequest {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Deserialize)]
pub struct ThemeQuery {
    pub theme: String,
}

#[derive(Clone)]
pub struct SearchResult {
    pub title: String,
    pub description: String,
}

// Helper function to extract theme from cookies
fn extract_theme_from_headers(headers: &HeaderMap) -> Theme {
    // Try to get theme from cookie
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Parse cookies to find theme
            for cookie in cookie_str.split(';') {
                let parts: Vec<&str> = cookie.trim().split('=').collect();
                if parts.len() == 2 && parts[0] == "theme" {
                    return Theme::from_str(parts[1]).unwrap_or(Theme::default());
                }
            }
        }
    }
    Theme::default()
}

// Main index page handler
pub async fn index(State(state): State<AppState>, headers: HeaderMap) -> IndexTemplate {
    // Extract theme from request (will be in signals once we handle them)
    let theme = extract_theme_from_headers(&headers);
    let theme_css = get_theme_variables(&theme);

    IndexTemplate {
        title: state.settings.application.name.clone(),
        environment: state.settings.application.environment.clone(),
        theme_css,
    }
}

// Examples page handler
pub async fn examples(State(state): State<AppState>, headers: HeaderMap) -> ExamplesTemplate {
    // Extract theme from request
    let theme = extract_theme_from_headers(&headers);
    let theme_css = get_theme_variables(&theme);

    ExamplesTemplate {
        title: state.settings.application.name.clone(),
        environment: state.settings.application.environment.clone(),
        theme_css,
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

// Search endpoint for active search functionality
pub async fn search(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    if headers.get("datastar-request").is_none() {
        return Err(AppError::not_found(
            "This endpoint only serves HTML fragments",
        ));
    }

    let query = params.q.to_lowercase();

    // Define all available examples
    let all_examples = vec![
        SearchResult {
            title: "Active Search".to_string(),
            description: "Search examples as you type".to_string(),
        },
        SearchResult {
            title: "Hypermedia Demo".to_string(),
            description: "Dynamic content loading with Datastar".to_string(),
        },
        SearchResult {
            title: "Form Demo".to_string(),
            description: "Form submission with hypermedia responses".to_string(),
        },
        SearchResult {
            title: "Server-Side Theme Switcher".to_string(),
            description: "Change themes with server-generated CSS variables".to_string(),
        },
    ];

    // Filter examples based on search query
    let results: Vec<SearchResult> = if query.is_empty() {
        all_examples
    } else {
        all_examples
            .into_iter()
            .filter(|example| {
                example.title.to_lowercase().contains(&query)
                    || example.description.to_lowercase().contains(&query)
            })
            .collect()
    };

    let template = SearchResultsTemplate { results };
    Ok(template.into_response())
}

// Theme switch endpoint - returns updated CSS variables via Datastar
pub async fn switch_theme(
    Query(params): Query<ThemeQuery>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    // Only serve to Datastar requests
    if headers.get("datastar-request").is_none() {
        return Err(AppError::not_found(
            "This endpoint only serves Datastar responses",
        ));
    }

    // Parse the theme from the query parameter
    let theme = Theme::from_str(&params.theme).unwrap_or(Theme::Light);
    let theme_css = get_theme_variables(&theme);

    // Return a style element that will be patched into the page
    let html = format!(
        r#"<style id="theme">
        :root {{
            {}
        }}
    </style>"#,
        theme_css
    );

    // Build response with cookie
    let mut response = Html(html).into_response();

    // Set cookie that expires in 1 year
    let cookie_value = format!(
        "theme={}; Path=/; Max-Age=31536000; SameSite=Lax",
        params.theme
    );
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie_value.parse().unwrap());

    Ok(response)
}
