use crate::{
    syntax_highlight::highlight_code,
    templates::{BackendCodeTemplate, ExamplesTemplate, IndexTemplate},
    theme::{get_syntax_highlighting_variables, get_theme_variables, Theme},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::header,
    http::HeaderMap,
    response::IntoResponse,
};
use std::fs;

pub use crate::examples::{
    get_items, search, submit_form, switch_theme, SearchResult,
};

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
    let syntax_css = get_syntax_highlighting_variables(&theme);

    IndexTemplate {
        title: state.settings.application.name.clone(),
        environment: state.settings.application.environment.clone(),
        theme_css,
        syntax_css,
    }
}

// Examples page handler
pub async fn examples(State(state): State<AppState>, headers: HeaderMap) -> ExamplesTemplate {
    // Extract theme from request
    let theme = extract_theme_from_headers(&headers);
    let theme_css = get_theme_variables(&theme);
    let syntax_css = get_syntax_highlighting_variables(&theme);
    
    // Get generated examples data and add syntax highlighting
    let mut examples = crate::examples_gen::get_examples();
    for example in &mut examples {
        // Apply syntax highlighting to HTML code
        if let Ok(highlighted) = highlight_code(&example.html, "html") {
            example.highlighted_html = highlighted;
        } else {
            example.highlighted_html = example.html.clone();
        }
    }

    ExamplesTemplate {
        title: state.settings.application.name.clone(),
        environment: state.settings.application.environment.clone(),
        theme_css,
        syntax_css,
        examples,
    }
}

// Handler to serve backend code for examples
pub async fn get_example_code(Path(example_id): Path<String>, headers: HeaderMap) -> impl IntoResponse {
    // Only serve fragments for Datastar requests
    if headers.get("datastar-request").is_none() {
        return BackendCodeTemplate {
            example_id: example_id.clone(),
            code: "// This endpoint only serves Datastar fragments".to_string(),
        }.into_response();
    }
    
    let file_path = format!("src/examples/{}.rs", example_id.replace('-', "_"));
    
    let raw_code = match fs::read_to_string(&file_path) {
        Ok(content) => {
            // Filter out the metadata comment lines
            content
                .lines()
                .skip_while(|line| line.starts_with("//!"))
                .collect::<Vec<&str>>()
                .join("\n")
                .trim_start()
                .to_string()
        }
        Err(e) => {
            // Try alternative path from workspace root
            let alt_path = format!("web-app/src/examples/{}.rs", example_id.replace('-', "_"));
            match fs::read_to_string(&alt_path) {
                Ok(content) => {
                    content
                        .lines()
                        .skip_while(|line| line.starts_with("//!"))
                        .collect::<Vec<&str>>()
                        .join("\n")
                        .trim_start()
                        .to_string()
                }
                Err(_) => format!("// Example code not found\n// Tried: {}\n// Alt: {}\n// Error: {}", file_path, alt_path, e),
            }
        }
    };
    
    // Apply syntax highlighting
    let code = highlight_code(&raw_code, "rust").unwrap_or(raw_code);
    
    BackendCodeTemplate {
        example_id,
        code,
    }.into_response()
}
