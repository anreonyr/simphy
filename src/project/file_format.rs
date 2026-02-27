use bevy::prelude::*;

#[derive(Component)]
pub struct SceneEntity {
    pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SceneData {
    pub entities: Vec<SceneEntityData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SceneEntityData {
    pub name: String,
    pub transform: TransformData,
    pub rigid_body: Option<RigidBodyData>,
    pub collider: Option<ColliderData>,
    pub charge: Option<f32>,
    pub field: Option<FieldData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TransformData {
    pub translation: Vec3,
    pub rotation: f32,
    pub scale: Vec3,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RigidBodyData {
    pub body_type: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ColliderData {
    pub shape: String,
    pub half_extents: Option<Vec2>,
    pub radius: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FieldData {
    pub field_type: String,
    pub strength: f32,
    pub direction: Vec2,
}
