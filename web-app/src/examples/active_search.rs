//! @title Active Search
//! @description Search examples as you type with instant results
//! @html_start
//! <input 
//!     type="text" 
//!     placeholder="Search examples..." 
//!     data-bind-search
//!     data-on-input__debounce.200ms="@get('/examples/search')"
//!     class="search-input"
//! >
//! <!-- Search results replace the contents of #example-cards-container -->
//! @html_end

use crate::{error::AppError, templates::ExampleSearchResultsTemplate, syntax_highlight::highlight_code};
use askama::Template;
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::{IntoResponse, Sse},
};
use serde::Deserialize;
use serde_json::Value;
use datastar::prelude::*;
use async_stream::stream;
use core::convert::Infallible;

#[derive(Deserialize, Debug)]
pub struct SearchRequestExtractor {
    datastar: Option<String>,
}

impl SearchRequestExtractor {
    pub fn get_search_text(&self) -> String {
        self.datastar
            .as_ref()
            .and_then(|json_str| serde_json::from_str::<Value>(json_str).ok())
            .and_then(|data| data.get("search").and_then(|v| v.as_str()).map(String::from))
            .unwrap_or_default()
    }
}

#[derive(Clone)]
pub struct SearchResult {
    pub title: String,
    pub description: String,
}

pub async fn search(
    State(_state): State<crate::AppState>,
    headers: HeaderMap,
    Query(extractor): Query<SearchRequestExtractor>,
) -> Result<impl IntoResponse, AppError> {
    if headers.get("datastar-request").is_none() {
        return Err(AppError::not_found(
            "This endpoint only serves HTML fragments",
        ));
    }

    // Extract search text from datastar parameter
    let query = extractor.get_search_text().to_lowercase();
    
    // Log the search request
    tracing::info!(
        search_query = %query,
        is_empty = query.is_empty(),
        "Search request received"
    );

    // Get all examples from generated data
    let all_examples = crate::examples_gen::get_examples();

    // Filter examples based on search query and add syntax highlighting
    let results: Vec<_> = if query.is_empty() {
        all_examples
    } else {
        all_examples
            .into_iter()
            .filter(|example| {
                example.title.to_lowercase().contains(&query)
                    || example.description.to_lowercase().contains(&query)
                    || example.id.to_lowercase().contains(&query)
            })
            .collect()
    }
    .into_iter()
    .map(|ex| {
        let highlighted_html = highlight_code(&ex.html, "html").unwrap_or_else(|_| ex.html.clone());
        crate::templates::ExampleWithHighlight {
            id: ex.id,
            title: ex.title,
            description: ex.description,
            html: ex.html,
            highlighted_html,
            backend_file: ex.backend_file,
        }
    })
    .collect();

    // Log the search results
    tracing::info!(
        results_count = results.len(),
        "Returning search results"
    );

    let template = ExampleSearchResultsTemplate { examples: results };
    
    // Render the template to HTML
    let html = template.render().map_err(|e| AppError::InternalServerError(anyhow::anyhow!("Template error: {}", e)))?;
    
    // Wrap results in the example-cards div to maintain structure
    let wrapped_html = format!(r#"<div id="example-cards">{}</div>"#, html);
    
    // Use datastar SDK to create proper SSE response with Inner mode
    Ok(Sse::new(stream! {
        let patch = PatchElements::new(wrapped_html)
            .mode(datastar::consts::ElementPatchMode::Inner)
            .selector("#example-cards-container");
        
        let sse_event = patch.write_as_axum_sse_event();
        yield Ok::<_, Infallible>(sse_event);
    }).into_response())
}