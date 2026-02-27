use crate::shared::EntityShape;
use bevy::prelude::*;

#[derive(Component)]
pub struct EditorEntity;

#[derive(Component)]
pub struct EntityName(pub String);

#[derive(Component)]
pub struct PlacementIndicator {
    pub shape: EntityShape,
    pub size: Vec2,
}

#[derive(Component)]
pub struct PlacedEntity {
    pub _bounds: Rect,
    pub shape: EntityShape,
    pub color: Color,
}

#[derive(Component)]
pub struct SelectedEntity;
