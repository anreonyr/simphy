use super::file_format::SceneData;
use std::path::Path;

pub fn import_yaml(path: &Path) -> Result<SceneData, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let data: SceneData =
        serde_yaml::from_str(&content).map_err(|e| format!("Failed to parse YAML: {}", e))?;

    Ok(data)
}

pub fn import_ron(path: &Path) -> Result<SceneData, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let data: SceneData =
        ron::from_str(&content).map_err(|e| format!("Failed to parse RON: {}", e))?;

    Ok(data)
}

pub fn import_scene(path: &Path) -> Result<SceneData, String> {
    match path.extension().and_then(|e| e.to_str()) {
        Some("yaml") | Some("yml") => import_yaml(path),
        Some("ron") => import_ron(path),
        _ => Err("Unsupported file format".to_string()),
    }
}
