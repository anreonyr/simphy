use bevy::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Magnetic;

#[derive(Component, Copy, Clone)]
pub struct Electric;

#[derive(Component, Debug, Clone, Copy)]
pub struct Field {
    pub strength: f32,
    pub direction: Vec3,
}

impl Field {
    pub fn new(strength: f32, direction: Vec3) -> Self {
        Self {
            strength,
            direction,
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct Charge {
    pub value: f32,
}

impl Charge {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}
