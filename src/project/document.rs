use std::path::PathBuf;

use super::file_format::SceneData;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Document {
    pub path: Option<PathBuf>,
    pub is_dirty: bool,
    pub data: SceneData,
}

impl Document {
    pub fn new() -> Self {
        Self {
            path: None,
            is_dirty: false,
            data: SceneData::default(),
        }
    }
}
