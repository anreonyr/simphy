use crate::shared::EntityShape;
use bevy::prelude::*;

#[derive(Clone, Debug, Resource, serde::Serialize, serde::Deserialize)]
pub struct EditorPrefs {
    pub grid_size: f32,
    pub snap_to_grid: bool,
    pub default_shape: EntityShape,
    pub default_entity_size: Vec2,
    pub show_grid: bool,
    pub show_field_vectors: bool,
}

impl Default for EditorPrefs {
    fn default() -> Self {
        Self {
            grid_size: 50.0,
            snap_to_grid: true,
            default_shape: EntityShape::Rectangle,
            default_entity_size: Vec2::new(50.0, 50.0),
            show_grid: true,
            show_field_vectors: false,
        }
    }
}
