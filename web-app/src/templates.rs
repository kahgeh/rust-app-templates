use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use chrono::{DateTime, Utc};
use crate::handlers::SearchResult;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub environment: String,
    pub theme_css: String,
}

#[derive(Template)]
#[template(path = "fragments/data_items.html")]
pub struct DataItemsTemplate {
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub items: Vec<String>,
}

#[derive(Template)]
#[template(path = "fragments/form_response.html")]
pub struct FormResponseTemplate {
    pub message: String,
}

#[derive(Clone)]
pub struct ExampleWithHighlight {
    pub id: String,
    pub title: String,
    pub description: String,
    pub html: String,
    pub highlighted_html: String,
    pub backend_file: String,
}

#[derive(Template)]
#[template(path = "examples.html")]
pub struct ExamplesTemplate {
    pub title: String,
    pub environment: String,
    pub theme_css: String,
    pub syntax_css: String,
    pub examples: Vec<ExampleWithHighlight>,
}

#[derive(Template)]
#[template(path = "fragments/search_results.html")]
pub struct SearchResultsTemplate {
    pub results: Vec<SearchResult>,
}

#[derive(Template)]
#[template(path = "fragments/backend_code.html")]
pub struct BackendCodeTemplate {
    pub example_id: String,
    pub code: String,
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!("Template rendering error: {}", err);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )
                    .into_response()
            }
        }
    }
}

impl IntoResponse for DataItemsTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!("Template rendering error: {}", err);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )
                    .into_response()
            }
        }
    }
}

impl IntoResponse for FormResponseTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!("Template rendering error: {}", err);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )
                    .into_response()
            }
        }
    }
}

impl IntoResponse for ExamplesTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!("Template rendering error: {}", err);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )
                    .into_response()
            }
        }
    }
}

impl IntoResponse for SearchResultsTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!("Template rendering error: {}", err);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )
                    .into_response()
            }
        }
    }
}

impl IntoResponse for BackendCodeTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => {
                tracing::error!("Template rendering error: {}", err);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )
                    .into_response()
            }
        }
    }
}