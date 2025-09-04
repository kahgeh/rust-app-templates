//! @title Server-Side Theme Switcher
//! @description Change themes with server-generated CSS variables
//! @html_start
//! <div style="display: flex; flex-wrap: wrap; gap: var(--size-3); align-items: center;">
//!     <label style="display: flex; align-items: center; gap: var(--size-2); cursor: pointer;">
//!         <input 
//!             type="radio" 
//!             name="theme" 
//!             value="light"
//!             data-on-click="$theme = 'light'; @get('/examples/theme/switch?theme=light')"
//!             data-attr-checked="$theme === 'light'">
//!         <span>☀️ Light</span>
//!     </label>
//!     <label style="display: flex; align-items: center; gap: var(--size-2); cursor: pointer;">
//!         <input 
//!             type="radio" 
//!             name="theme" 
//!             value="dark"
//!             data-on-click="$theme = 'dark'; @get('/examples/theme/switch?theme=dark')"
//!             data-attr-checked="$theme === 'dark'">
//!         <span>🌙 Dark</span>
//!     </label>
//!     <label style="display: flex; align-items: center; gap: var(--size-2); cursor: pointer;">
//!         <input 
//!             type="radio" 
//!             name="theme" 
//!             value="dim"
//!             data-on-click="$theme = 'dim'; @get('/examples/theme/switch?theme=dim')"
//!             data-attr-checked="$theme === 'dim'">
//!         <span>🌫️ Dim</span>
//!     </label>
//!     <label style="display: flex; align-items: center; gap: var(--size-2); cursor: pointer;">
//!         <input 
//!             type="radio" 
//!             name="theme" 
//!             value="grape"
//!             data-on-click="$theme = 'grape'; @get('/examples/theme/switch?theme=grape')"
//!             data-attr-checked="$theme === 'grape'">
//!         <span>🍇 Grape</span>
//!     </label>
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