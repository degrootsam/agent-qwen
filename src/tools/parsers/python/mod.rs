use ollama_rs::function;
use std::fs;
use std::path::Path;
use std::{error::Error, path::PathBuf};
use tree_sitter::{Language, Parser, Tree};

unsafe extern "C" {
    fn tree_sitter_python() -> Language;
}

/// Parses a Python file and extracts function definitions.
pub fn parse_python_file(file_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;

    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_python() };
    parser.set_language(&language)?;

    let tree: Tree = parser
        .parse(&content, None)
        .ok_or("Failed to parse the Python file")?;

    let mut cursor = tree.walk();
    let root_node = tree.root_node();

    let mut functions = Vec::new();

    for node in root_node.children(&mut cursor) {
        if node.kind() == "function_definition" {
            if let Some(identifier) = node.child_by_field_name("name") {
                let func_name = content[identifier.byte_range()].to_string();
                functions.push(format!("Function: {}()", func_name));
            }
        }
    }

    Ok(functions)
}

/// High-level function that summarizes Python code.
#[function]
pub async fn summarize_python_code(
    file_path: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let path = PathBuf::from(file_path);

    match parse_python_file(&path) {
        Ok(functions) => {
            if functions.is_empty() {
                Ok("No functions found in the file.".to_string())
            } else {
                Ok(format!("Extracted functions:\n{}", functions.join("\n")))
            }
        }
        Err(err) => Ok(format!("Error parsing Rust file: {}", err)),
    }
}
