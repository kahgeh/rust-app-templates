//! @title Active Search
//! @description Search examples as you type with instant results
//! @html_start
//! <input 
//!     type="text" 
//!     placeholder="Search examples..." 
//!     data-model="search"
//!     data-on-input="@get('/examples/elements/search?q=' + encodeURIComponent(search))"
//!     class="search-input"
//! >
//! <div id="search-results" class="search-results">
//!     <!-- Search results will appear here -->
//! </div>
//! @html_end

use crate::{error::AppError, templates::SearchResultsTemplate};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Clone)]
pub struct SearchResult {
    pub title: String,
    pub description: String,
}

pub async fn search(
    State(_state): State<crate::AppState>,
    headers: HeaderMap,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    if headers.get("datastar-request").is_none() {
        return Err(AppError::not_found(
            "This endpoint only serves HTML fragments",
        ));
    }

    let query = params.q.to_lowercase();

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