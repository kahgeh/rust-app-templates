// Auto-generated file - DO NOT EDIT
pub const EXAMPLES_DATA: &str = include_str!("examples_data.yaml");

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleData {
    pub id: String,
    pub title: String,
    pub description: String,
    pub html: String,
    pub backend_file: String,
}

#[derive(Debug, Deserialize)]
pub struct ExamplesRoot {
    pub examples: Vec<ExampleData>,
}

pub fn get_examples() -> Vec<ExampleData> {
    let examples_root: ExamplesRoot = serde_yaml_ng::from_str(EXAMPLES_DATA)
        .expect("Failed to parse examples data");
    examples_root.examples
}
