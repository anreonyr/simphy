use crate::shared::generate_arc_vertices;
use crate::simulation::{Charge, Electric, Field, Magnetic};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn dev_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Magnetic Field"),
        Transform::from_xyz(2000.0, 0.0, 0.0),
        Magnetic,
        Field::new(40.0, Vec3::new(0.0, 0.0, -1.0)),
        Sensor,
        RigidBody::Static,
        Collider::rectangle(4000.0, 8000.0),
        DebugRender::default(),
        CollisionEventsEnabled,
        // CollidingEntities::default(),
    ));

    commands.spawn((
        Name::new("Electric Field"),
        Transform::from_xyz(-2000.0, 0.0, 0.0),
        Electric,
        Field::new(10000.0, Vec3::new(1.0, 1.0, 0.0)),
        Sensor,
        RigidBody::Static,
        Collider::rectangle(4000.0, 8000.0),
        DebugRender::default(),
        CollisionEventsEnabled,
        // CollidingEntities::default(),
    ));

    commands.spawn((
        Name::new("Ball"),
        Transform::from_xyz(-1000.0, 1000.0, 0.0),
        Charge::new(10.0),
        RigidBody::Dynamic,
        Collider::circle(10.0),
        // DebugRender::default(),
    ));

    // Arc collider example
    // let arc_vertices = generate_arc_vertices(100.0, 0.0, -std::f32::consts::PI, 32);
    // commands.spawn((
    //     RigidBody::Static,
    //     Collider::polyline(arc_vertices, None),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    //     DebugRender::default(),
    // ));
}
