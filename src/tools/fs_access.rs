use ollama_rs::function;
#[allow(unused_imports)]
use std::error::Error;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// Creates a directory at the specified path.
///
/// If the directory already exists, no changes are made.
///
/// # Arguments
/// * `path` - The absolute or relative path where the directory should be created.
///
/// # Returns
/// A success or error message.
#[function]
pub async fn create_directory(path: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let directory_path = PathBuf::from(&path);

    if directory_path.exists() {
        return Ok(format!("Directory `{}` already exists.", path));
    }

    fs::create_dir_all(&directory_path)?;
    Ok(format!("Directory `{}` created successfully!", path))
}

/// Creates a file at the specified path.
///
/// If the file already exists, no changes are made.
/// If `content` is provided, it writes the content to the file.
///
/// # Arguments
/// * `path` - The file path where the file should be created.
/// * `content` - (Optional) Text content to write to the file. Defaults to an empty string.
///
/// # Returns
/// A success or error message.
#[function]
pub async fn create_file(
    path: String,
    content: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file_path = PathBuf::from(&path);

    if file_path.exists() {
        return Ok(format!("File `{}` already exists.", path));
    }

    File::create(&file_path)?;

    if !content.is_empty() {
        fs::write(&file_path, content)?;
        return Ok(format!(
            "File `{}` created and content written successfully!",
            path
        ));
    }
    Ok(format!("File `{}` created successfully!", path))
}

/// Writes content to a file at the specified path.
///
/// If the file does not exist, an error is returned.
///
/// # Arguments
/// * `path` - The file path where content should be written.
/// * `content` - The text content to write into the file.
///
/// # Returns
/// A success or error message.
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

/// Gathers the directory structure for a given path (defaults to current directory).
///
/// This function returns a structured list of files and directories, which the AI can use for context.
///
/// # Arguments
/// * `path` - The directory to analyze (optional, defaults to ".").
///
/// # Example Usage:
/// ```
/// gather_directory_context(Some("src"))
/// ```
/// Output:
/// ```json
/// {
///   "directories": [
///     "src/tools",
///     "src/parser"
///   ],
///   "files": [
///     "src/main.rs",
///     "src/lib.rs"
///   ]
/// }
/// ```
#[function]
pub async fn gather_directory_context(
    path: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let base_path = if path.is_empty() {
        ".".to_string()
    } else {
        path
    };
    let base_path = Path::new(&base_path);

    if !base_path.exists() || !base_path.is_dir() {
        return Ok(format!(
            "Error: The path `{}` does not exist or is not a directory.",
            base_path.display()
        ));
    }

    let mut directories = vec![];
    let mut files = vec![];

    // Recursively walk through the directory
    for entry in walkdir::WalkDir::new(base_path)
        .into_iter()
        .filter_map(Result::ok)
    {
        let entry_path = entry
            .path()
            .strip_prefix(base_path)
            .unwrap_or(entry.path())
            .to_path_buf();

        if entry.file_type().is_dir() {
            directories.push(entry_path.to_string_lossy().to_string());
        } else {
            files.push(entry_path.to_string_lossy().to_string());
        }
    }

    let result = serde_json::json!({
        "directories": directories,
        "files": files
    });

    Ok(result.to_string())
}

/// Reads the contents of a file.
///
/// # Arguments
/// * `path` - The file path to read from.
///
/// # Returns
/// The file contents or an error message.
#[function]
pub async fn read_file(path: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file_path = PathBuf::from(&path);

    if !file_path.exists() {
        return Ok(format!("Error: File `{}` does not exist.", path));
    }

    let content = fs::read_to_string(&file_path)?;

    if content.trim().is_empty() {
        return Ok(format!("File `{}` is empty.", path));
    }

    Ok(format!("Contents of `{}`:\n{}", path, content))
}
