use anyhow::Result;
use inkjet::{Highlighter, Language, formatter};

pub fn highlight_code(code: &str, language: &str) -> Result<String> {
    let lang = match language.to_lowercase().as_str() {
        "rust" | "rs" => Language::Rust,
        "javascript" | "js" => Language::Javascript,
        "html" => Language::Html,
        _ => Language::Plaintext,
    };

    let mut highlighter = Highlighter::new();
    let formatter = formatter::Html;
    
    let highlighted = highlighter.highlight_to_string(lang, &formatter, code)?;
    
    Ok(highlighted)
}