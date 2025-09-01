use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use chrono::{DateTime, Utc};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub environment: String,
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