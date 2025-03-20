use ollama_rs::function;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File, Item};

/// Parses a Rust file and extracts function definitions.
pub fn parse_rust_file(file_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let syntax_tree: File = syn::parse_file(&content)?;

    let mut functions = Vec::new();

    for item in syntax_tree.items {
        if let Item::Fn(func) = item {
            let func_name = func.sig.ident.to_string();
            functions.push(format!(
                "Function: {} -> {}",
                func_name,
                quote::quote!(#func.sig)
            ));
        }
    }

    Ok(functions)
}

/// High-level function that summarizes Rust code.
#[function]
pub async fn summarize_rust_code(
    file_path: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let path = PathBuf::from(file_path);

    match parse_rust_file(&path) {
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
