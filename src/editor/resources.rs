use crate::{
    shared::EntityShape,
    simulation::{Charge, Field},
};
use avian2d::prelude::{ConstantForce, Friction, LinearVelocity, Mass, RigidBody};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tool {
    #[default]
    Pan,
    Select,
    Move,
    Place,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RigidBodyType {
    #[default]
    Dynamic,
    Static,
    Kinematic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldType {
    #[default]
    None,
    Magnetic,
    Electric,
}

#[derive(Resource)]
pub struct PlacementState {
    pub tool: Tool,
    pub size: Vec2,
    pub mass: f32,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub shape: EntityShape,
    pub field_type: FieldType,
    pub field: Field,
    pub charge: Charge,
    pub friction: Friction,
    pub initial_velocity: LinearVelocity,
    pub constant_force: ConstantForce,
}

impl Default for PlacementState {
    fn default() -> Self {
        Self {
            tool: Tool::Pan,
            mass: 1.0,
            size: Vec2::new(50.0, 50.0),
            transform: Transform::from_translation(Vec3::ZERO),
            rigid_body: RigidBody::Dynamic,
            shape: EntityShape::default(),
            field_type: FieldType::None,
            field: Field::new(0.0, Vec3::Z),
            charge: Charge::new(0.0),
            friction: Friction::ZERO,
            initial_velocity: LinearVelocity::ZERO,
            constant_force: ConstantForce::new(0.0, 0.0),
        }
    }
}

#[derive(Resource, Default)]
pub struct PlacedEntities {
    pub entities: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct IndicatorInfo {
    pub world_position: Vec2,
    pub aligned_position: Vec2,
    pub can_place: bool,
    pub placed_count: usize,
    pub pointer_over_ui: bool,
}

#[derive(Resource, Default)]
pub struct EntityProperties {
    pub entity: Option<Entity>,
    pub position: Vec3,
    pub scale: Vec2,
    pub color: Color,
    pub shape: EntityShape,
    pub rigid_body_type: RigidBodyType,
    pub uniform_scale: bool,
}

#[derive(Resource, Default)]
pub struct ClearRequest(pub bool);

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct DragState {
    pub dragging: bool,
    pub entity: Option<Entity>,
    pub offset: Vec2,
}
