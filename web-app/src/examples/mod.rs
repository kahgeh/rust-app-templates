pub mod active_search;
pub mod form_demo;
pub mod hypermedia_demo;
pub mod theme_switcher;

pub use active_search::{search, SearchResult};
pub use form_demo::submit_form;
pub use hypermedia_demo::get_items;
pub use theme_switcher::switch_theme;