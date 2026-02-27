use super::file_format::SceneData;
use std::path::Path;

pub fn export_yaml(path: &Path, data: &SceneData) -> Result<(), String> {
    let content =
        serde_yaml::to_string(data).map_err(|e| format!("Failed to serialize YAML: {}", e))?;

    std::fs::write(path, content).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

pub fn export_ron(path: &Path, data: &SceneData) -> Result<(), String> {
    let content = ron::to_string(data).map_err(|e| format!("Failed to serialize RON: {}", e))?;

    std::fs::write(path, content).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

pub fn export_scene(path: &Path, data: &SceneData) -> Result<(), String> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("yaml") | Some("yml") => export_yaml(path, data),
        Some("ron") => export_ron(path, data),
        _ => Err("Unsupported file format".to_string()),
    }
}
