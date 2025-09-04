use std::fs;
use std::io::Write;
use std::path::Path;

struct Example {
    id: String,
    title: String,
    description: String,
    html: String,
    file_path: String,
}

fn extract_metadata(file_content: &str, file_path: &str) -> Option<Example> {
    let mut title = String::new();
    let mut description = String::new();
    let mut html_lines = Vec::new();
    let mut in_html = false;
    
    for line in file_content.lines() {
        if line.starts_with("//! @title") {
            title = line.trim_start_matches("//! @title").trim().to_string();
        } else if line.starts_with("//! @description") {
            description = line.trim_start_matches("//! @description").trim().to_string();
        } else if line.starts_with("//! @html_start") {
            in_html = true;
        } else if line.starts_with("//! @html_end") {
            in_html = false;
        } else if in_html && line.starts_with("//!") {
            html_lines.push(line.trim_start_matches("//!").trim());
        }
    }
    
    if title.is_empty() || description.is_empty() {
        return None;
    }
    
    let id = Path::new(file_path)
        .file_stem()?
        .to_str()?
        .to_string()
        .replace('_', "-");
    
    Some(Example {
        id,
        title,
        description,
        html: html_lines.join("\n"),
        file_path: file_path.to_string(),
    })
}

fn main() {
    println!("cargo:rerun-if-changed=src/examples");
    
    let examples_dir = Path::new("src/examples");
    let mut examples = Vec::new();
    
    if examples_dir.exists() {
        for entry in fs::read_dir(examples_dir).expect("Failed to read examples directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("rs") && 
               path.file_name().and_then(|s| s.to_str()) != Some("mod.rs") {
                
                let content = fs::read_to_string(&path)
                    .expect(&format!("Failed to read file: {:?}", path));
                
                if let Some(example) = extract_metadata(&content, path.to_str().unwrap()) {
                    examples.push(example);
                }
            }
        }
    }
    
    // Generate YAML data file
    let yaml_path = Path::new("generated/examples_data.yaml");
    fs::create_dir_all("generated").expect("Failed to create generated directory");
    
    let mut yaml_file = fs::File::create(yaml_path).expect("Failed to create YAML file");
    writeln!(yaml_file, "examples:").expect("Failed to write to YAML file");
    
    for example in &examples {
        writeln!(yaml_file, "  - id: \"{}\"", example.id).expect("Failed to write to YAML");
        writeln!(yaml_file, "    title: \"{}\"", example.title).expect("Failed to write to YAML");
        writeln!(yaml_file, "    description: \"{}\"", example.description).expect("Failed to write to YAML");
        writeln!(yaml_file, "    html: |").expect("Failed to write to YAML");
        for line in example.html.lines() {
            writeln!(yaml_file, "      {}", line).expect("Failed to write to YAML");
        }
        writeln!(yaml_file, "    backend_file: \"{}\"", example.file_path).expect("Failed to write to YAML");
    }
    
    // Generate Rust code to include the examples data
    let rust_path = Path::new("generated/examples_data.rs");
    let mut rust_file = fs::File::create(rust_path).expect("Failed to create Rust file");
    
    writeln!(rust_file, "// Auto-generated file - DO NOT EDIT").expect("Failed to write");
    writeln!(rust_file, "pub const EXAMPLES_DATA: &str = include_str!(\"examples_data.yaml\");").expect("Failed to write");
    writeln!(rust_file, "").expect("Failed to write");
    writeln!(rust_file, "use serde::{{Deserialize, Serialize}};").expect("Failed to write");
    writeln!(rust_file, "").expect("Failed to write");
    writeln!(rust_file, "#[derive(Debug, Clone, Serialize, Deserialize)]").expect("Failed to write");
    writeln!(rust_file, "pub struct ExampleData {{").expect("Failed to write");
    writeln!(rust_file, "    pub id: String,").expect("Failed to write");
    writeln!(rust_file, "    pub title: String,").expect("Failed to write");
    writeln!(rust_file, "    pub description: String,").expect("Failed to write");
    writeln!(rust_file, "    pub html: String,").expect("Failed to write");
    writeln!(rust_file, "    pub backend_file: String,").expect("Failed to write");
    writeln!(rust_file, "}}").expect("Failed to write");
    writeln!(rust_file, "").expect("Failed to write");
    writeln!(rust_file, "#[derive(Debug, Deserialize)]").expect("Failed to write");
    writeln!(rust_file, "pub struct ExamplesRoot {{").expect("Failed to write");
    writeln!(rust_file, "    pub examples: Vec<ExampleData>,").expect("Failed to write");
    writeln!(rust_file, "}}").expect("Failed to write");
    writeln!(rust_file, "").expect("Failed to write");
    writeln!(rust_file, "pub fn get_examples() -> Vec<ExampleData> {{").expect("Failed to write");
    writeln!(rust_file, "    let examples_root: ExamplesRoot = serde_yaml_ng::from_str(EXAMPLES_DATA)").expect("Failed to write");
    writeln!(rust_file, "        .expect(\"Failed to parse examples data\");").expect("Failed to write");
    writeln!(rust_file, "    examples_root.examples").expect("Failed to write");
    writeln!(rust_file, "}}").expect("Failed to write");
}