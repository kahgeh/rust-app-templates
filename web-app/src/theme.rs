use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Dim,
    Grape,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

/// Generate CSS variables for the selected theme using Open Props color scales
pub fn get_theme_variables(theme: &Theme) -> String {
    match theme {
        Theme::Light => {
            r#"
  /* Light theme - default Open Props light mode */
  color-scheme: light;
  
  /* Background gradient - diagonal from top-left to bottom-right */
  --surface-0: linear-gradient(135deg, var(--gray-2), var(--gray-5));
  
  /* Text colors */
  --text-1: var(--gray-12);
  --text-2: var(--gray-7);
  
  /* Surface colors */
  --surface-1: var(--gray-1);
  --surface-2: var(--gray-3);
  --surface-3: var(--gray-5);
  --surface-4: var(--gray-6);
  
  /* Shadows */
  --surface-shadow: var(--gray-3-hsl);
  --shadow-strength: 10%;
            "#
        }
        Theme::Dark => {
            r#"
  /* Dark theme - default Open Props dark mode */
  color-scheme: dark;
  
  /* Background gradient */
  --surface-0: conic-gradient(from 90deg at 50% 0%, #2a2a2a, 50%, #3a3a3a, #2a2a2a); 
  
  /* Text colors */
  --text-1: var(--gray-1);
  --text-2: var(--gray-5);
  
  /* Surface colors */
  --surface-1: var(--gray-10);
  --surface-2: var(--gray-9);
  --surface-3: var(--gray-8);
  --surface-4: var(--gray-7);
  
  /* Shadows */
  --surface-shadow: var(--gray-12-hsl);
  --shadow-strength: 80%;
            "#
        }
        Theme::Dim => {
            r#"
  /* Dim theme - muted dark theme using slate/blue-gray */
  color-scheme: dark;
  
  /* Background gradient */
  --surface-0: var(--gradient-6);  /* Subtle dim gradient */
  
  /* Text colors */
  --text-1: var(--gray-3);
  --text-2: var(--gray-4);
  
  /* Surface colors */
  --surface-1: var(--gray-8);
  --surface-2: var(--gray-9);  /* Darker background for code blocks */
  --surface-3: var(--gray-6);
  --surface-4: var(--gray-5);
  
  /* Shadows */
  --surface-shadow: var(--slate-12-hsl);
  --shadow-strength: 50%;
            "#
        }
        Theme::Grape => {
            r#"
  /* Grape theme - purple-tinted dark theme */
  color-scheme: dark;
  
  /* Background gradient */
  --surface-0: var(--gradient-23);  /* Purple gradient */
  
  /* Text colors */
  --text-1: var(--purple-1);
  --text-2: var(--purple-3);
  
  /* Surface colors - using purple and pink hues */
  --surface-1: hsl(280 30% 8%);   /* Deep purple-black */
  --surface-2: hsl(280 25% 11%);  /* Slightly lighter purple */
  --surface-3: hsl(280 20% 14%);  /* Purple-gray */
  --surface-4: hsl(280 15% 17%);  /* Light purple-gray */
  
  /* Accent colors */
  --brand: var(--purple-6);
  --text-highlight: var(--pink-5);
  
  /* Shadows with purple tint */
  --surface-shadow: 280 20% 10%;
  --shadow-strength: 60%;
            "#
        }
    }
    .trim()
    .to_string()
}

/// Generate syntax highlighting CSS variables for the current theme
pub fn get_syntax_highlighting_variables(theme: &Theme) -> String {
    match theme {
        Theme::Light => {
            r#"
  /* Light theme syntax highlighting - GitHub Light theme for WCAG AA compliance */
  --token-text: var(--gray-12);       /* #24292f - plain text in code */
  --token-keyword: var(--red-6);      /* #cf222e - keywords */
  --token-string: var(--blue-9);      /* #0a3069 - strings */
  --token-number: var(--orange-7);    /* #953800 - numbers */
  --token-comment: var(--gray-7);     /* #57606a - comments */
  --token-function: var(--purple-7);  /* #8250df - functions */
  --token-type: var(--orange-7);      /* #953800 - types */
  --token-operator: var(--gray-12);   /* #24292f - operators */
  --token-punctuation: var(--gray-7); /* #57606a - punctuation */
  --token-variable: var(--gray-12);   /* #24292f - variables */
  --token-constant: var(--blue-7);    /* #0550ae - constants */
  --token-tag: var(--red-6);          /* #cf222e - HTML/JSX tags */
  --token-attribute: var(--blue-7);   /* #0550ae - attributes */
  --token-namespace: var(--purple-7); /* #8250df - namespaces */
            "#
        }
        Theme::Dark => {
            r#"
  /* Dark theme syntax highlighting */
  --token-text: var(--gray-1);        /* plain text in code */
  --token-keyword: var(--pink-3);
  --token-string: var(--green-3);
  --token-number: var(--orange-3);
  --token-comment: var(--gray-5);
  --token-function: var(--cyan-3);
  --token-type: var(--blue-3);
  --token-operator: var(--gray-1);
  --token-punctuation: var(--gray-2);
  --token-variable: var(--purple-3);
  --token-constant: var(--pink-3);
  --token-tag: var(--red-3);
  --token-attribute: var(--orange-3);
  --token-namespace: var(--teal-3);
            "#
        }
        Theme::Dim => {
            r#"
  /* Dim theme syntax highlighting - muted but with good contrast against dark background */
  --token-text: var(--gray-5);        /* plain text - lighter gray for contrast */
  --token-keyword: var(--purple-5);   /* keywords - muted purple */
  --token-string: var(--green-5);     /* strings - muted green */
  --token-number: var(--orange-5);    /* numbers - muted orange */
  --token-comment: var(--gray-6);     /* comments - subtle gray */
  --token-function: var(--blue-5);    /* functions - muted blue */
  --token-type: var(--cyan-5);        /* types - muted cyan */
  --token-operator: var(--gray-5);    /* operators - same as text */
  --token-punctuation: var(--gray-6); /* punctuation - subtle */
  --token-variable: var(--indigo-5);  /* variables - muted indigo */
  --token-constant: var(--pink-5);    /* constants - muted pink */
  --token-tag: var(--red-5);          /* tags - muted red */
  --token-attribute: var(--orange-5); /* attributes - muted orange */
  --token-namespace: var(--teal-5);   /* namespaces - muted teal */
            "#
        }
        Theme::Grape => {
            r#"
  /* Grape theme syntax highlighting */
  --token-text: var(--purple-1);      /* plain text in code */
  --token-keyword: var(--purple-3);
  --token-string: var(--green-3);
  --token-number: var(--orange-3);
  --token-comment: var(--purple-6);
  --token-function: var(--pink-3);
  --token-type: var(--purple-4);
  --token-operator: var(--purple-2);
  --token-punctuation: var(--purple-3);
  --token-variable: var(--pink-4);
  --token-constant: var(--purple-3);
  --token-tag: var(--pink-3);
  --token-attribute: var(--orange-3);
  --token-namespace: var(--teal-3);
            "#
        }
    }
    .trim()
    .to_string()
}

/// Parse theme from string (for handling user input)
impl Theme {
    pub fn from_str(s: &str) -> Option<Theme> {
        match s.to_lowercase().as_str() {
            "light" => Some(Theme::Light),
            "dark" => Some(Theme::Dark),
            "dim" => Some(Theme::Dim),
            "grape" => Some(Theme::Grape),
            _ => None,
        }
    }
}
