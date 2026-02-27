use avian2d::prelude::*;
use bevy::prelude::*;

use super::components::{Charge, Electric, Field, Magnetic};

pub fn apply_magnetic_force(
    mut magnets: Query<(&Field, &CollidingEntities), With<Magnetic>>,
    mut entities: Query<(&Charge, Forces)>,
) {
    for (field, colliding_entities) in &mut magnets {
        if colliding_entities.0.is_empty() {
            return;
        }

        for entity in &colliding_entities.0 {
            if let Ok((charge, mut forces)) = entities.get_mut(*entity) {
                let velocity = forces.linear_velocity();
                let q = charge.value;
                let b = field.strength;

                let magnetic_force =
                    field.direction.normalize().cross(velocity.extend(0.0)) * q * b;
                forces.apply_force(magnetic_force.xy());
            }
        }
    }
}

pub fn apply_electric_force(
    mut magnets: Query<(&Field, &CollidingEntities), With<Electric>>,
    mut entities: Query<(&Charge, Forces)>,
) {
    for (field, colliding_entities) in &mut magnets {
        if colliding_entities.0.is_empty() {
            return;
        }

        for entity in &colliding_entities.0 {
            if let Ok((charge, mut forces)) = entities.get_mut(*entity) {
                let q = charge.value;
                let b = field.strength;

                let electric_force = field.direction.normalize() * q * b;
                forces.apply_force(electric_force.xy());
            }
        }
    }
}
