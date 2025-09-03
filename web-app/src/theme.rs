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
  
  /* Background gradient */
  --surface-0: var(--gradient-28);  /* Subtle light gradient */
  
  /* Text colors */
  --text-1: var(--gray-12);
  --text-2: var(--gray-7);
  
  /* Surface colors */
  --surface-1: var(--gray-0);
  --surface-2: var(--gray-1);
  --surface-3: var(--gray-2);
  --surface-4: var(--gray-3);
  
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
  --surface-2: var(--gray-7);
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
