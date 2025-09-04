//! @title Server-Side Theme Switcher
//! @description Change themes with server-generated CSS variables
//! @html_start
//! <div style="display: flex; flex-direction: column; gap: var(--size-3);">
//!     <button 
//!         data-on-click="$theme = 'light'; @get('/examples/theme/switch?theme=light')"
//!         style="padding: var(--size-3); background: var(--gray-2); color: var(--gray-12); border: var(--border-size-2) solid var(--gray-4); border-radius: var(--radius-2);">
//!         ☀️ Light Theme
//!     </button>
//!     <button 
//!         data-on-click="$theme = 'dark'; @get('/examples/theme/switch?theme=dark')"
//!         style="padding: var(--size-3); background: var(--gray-11); color: var(--gray-1); border: var(--border-size-2) solid var(--gray-9); border-radius: var(--radius-2);">
//!         🌙 Dark Theme
//!     </button>
//!     <button 
//!         data-on-click="$theme = 'dim'; @get('/examples/theme/switch?theme=dim')"
//!         style="padding: var(--size-3); background: var(--slate-11); color: var(--slate-1); border: var(--border-size-2) solid var(--slate-9); border-radius: var(--radius-2);">
//!         🌫️ Dim Theme
//!     </button>
//!     <button 
//!         data-on-click="$theme = 'grape'; @get('/examples/theme/switch?theme=grape')"
//!         style="padding: var(--size-3); background: var(--purple-11); color: var(--purple-1); border: var(--border-size-2) solid var(--purple-9); border-radius: var(--radius-2);">
//!         🍇 Grape Theme
//!     </button>
//! </div>
//! <div style="margin-top: var(--size-4); padding: var(--size-3); background: var(--surface-2); border-radius: var(--radius-2);">
//!     <p style="color: var(--text-1); margin: 0;">Current theme: <strong data-text="$theme"></strong></p>
//!     <p style="color: var(--text-2); margin: 0; margin-top: var(--size-2); font-size: var(--font-size-0);">This theme is persisted and sent with every request!</p>
//! </div>
//! @html_end

use crate::{
    error::AppError,
    theme::{get_theme_variables, Theme},
};
use axum::{
    extract::Query,
    http::{header, HeaderMap},
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ThemeQuery {
    pub theme: String,
}

pub async fn switch_theme(
    Query(params): Query<ThemeQuery>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    if headers.get("datastar-request").is_none() {
        return Err(AppError::not_found(
            "This endpoint only serves Datastar responses",
        ));
    }

    let theme = Theme::from_str(&params.theme).unwrap_or(Theme::Light);
    let theme_css = get_theme_variables(&theme);

    let html = format!(
        r#"<style id="theme">
        :root {{
            {}
        }}
    </style>"#,
        theme_css
    );

    let mut response = Html(html).into_response();

    let cookie_value = format!(
        "theme={}; Path=/; Max-Age=31536000; SameSite=Lax",
        params.theme
    );
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie_value.parse().unwrap());

    Ok(response)
}