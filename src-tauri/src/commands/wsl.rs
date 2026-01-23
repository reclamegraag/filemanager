use crate::fs::{AppError, WslDistro};
use std::process::Command;

#[tauri::command]
pub fn get_wsl_distros() -> Vec<WslDistro> {
    let mut distros = Vec::new();

    // Try to get WSL distributions using wsl.exe
    if let Ok(output) = Command::new("wsl.exe")
        .args(["--list", "--quiet"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for line in stdout.lines() {
                let name = line.trim().replace('\0', "").replace('\u{feff}', "");
                if !name.is_empty() {
                    let path = format!("\\\\wsl$\\{}", name);
                    distros.push(WslDistro {
                        name: name.clone(),
                        path,
                        is_default: distros.is_empty(), // First one is default
                    });
                }
            }
        }
    }

    distros
}

#[tauri::command]
pub async fn wsl_copy(
    source: String,
    dest: String,
    use_wsl_native: bool,
) -> Result<(), AppError> {
    if use_wsl_native {
        // Use wsl.exe cp for bulk/large operations
        let output = Command::new("wsl.exe")
            .args(["cp", "-r", &source, &dest])
            .output()
            .map_err(|e| AppError::Io(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Io(stderr.to_string()));
        }
    } else {
        // Use standard Windows copy via UNC path
        std::fs::copy(&source, &dest)
            .map_err(|e| AppError::Io(e.to_string()))?;
    }

    Ok(())
}
