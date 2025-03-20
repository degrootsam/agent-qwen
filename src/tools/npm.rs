use ollama_rs::function;
use std::process::Command;

/// Installs an npm package in a given directory.
///
/// Can be used toinstall JavaScript/TypeScript dependencies in a project.
///
/// # Arguments
/// * package - The name of the npm package to install.
/// * directory - The directory where the package should be installed. Defaults to the current directory.
///
/// # Example
/// AI can call this function like:
///
/// install_npm_package("express", "my_project")
///
/// This will run 'npm install express' inside the 'my_project' directory.
#[function]
pub async fn install_npm_package(
    package: String,
    directory: String, // Now a required argument
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let install_dir = if directory.is_empty() {
        ".".to_string()
    } else {
        directory
    };

    let npm_check = Command::new("npm").arg("--version").output();
    if npm_check.is_err() {
        return Ok("Error: npm is not installed on this system.".to_string());
    }

    let output = Command::new("npm")
        .arg("install")
        .arg(package.clone())
        .current_dir(&install_dir)
        .output()?;

    if output.status.success() {
        Ok(format!(
            "Successfully installed `{}` in `{}`!",
            package, install_dir
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(format!("Error installing `{}`: {}", package, stderr))
    }
}
