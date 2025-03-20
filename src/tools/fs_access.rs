use ollama_rs::function;
#[allow(unused_imports)]
use std::error::Error;
use std::fs::{self, File};
use std::path::PathBuf;

/// Function to create a directory at the given location.
/// Returns a message with the result
#[function]
pub async fn create_directory(path: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let directory_path = PathBuf::from(&path);

    if directory_path.exists() {
        return Ok(format!("Directory `{}` already exists.", path));
    }

    fs::create_dir_all(&directory_path)?;
    Ok(format!("Directory `{}` created successfully!", path))
}

/// Function to create a file at the given location.
/// Returns a message with the result
#[function]
pub async fn create_file(path: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file_path = PathBuf::from(&path);

    if file_path.exists() {
        return Ok(format!("File `{}` already exists.", path));
    }

    File::create(&file_path)?;
    Ok(format!("File `{}` created successfully!", path))
}

/// Function to write content to a file.
/// Returns a message with the result
#[function]
pub async fn write_to_file(
    path: String,
    content: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file_path = PathBuf::from(&path);

    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            return Ok(format!(
                "Error: Directory `{}` does not exist. Please create it first.",
                parent.display()
            ));
        }
    }

    // Write content to file
    fs::write(&file_path, content)?;

    Ok(format!("Successfully wrote to `{}`!", path))
}

/// Function to read a directory structure
/// Returns a message with the result
#[function]
pub async fn read_directory(path: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    Ok("Directory structure read successfully:");
};
