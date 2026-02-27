use avian2d::prelude::Collider;
use bevy::prelude::*;

use super::utils::{
    generate_cross_vertices, generate_regular_polygon_vertices, generate_star_vertices,
};

#[derive(Component)]
pub struct InitialState {
    pub transform: Transform,
    pub velocity: Vec2,
}

impl InitialState {
    pub fn new(transform: Transform, velocity: Vec2) -> Self {
        Self {
            transform,
            velocity,
        }
    }
}

#[derive(
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Component,
)]
pub enum EntityShape {
    #[default]
    Rectangle,
    Circle,
    Triangle,
    Pentagon,
    Hexagon,
    Star,
    Diamond,
    Cross,
}

impl EntityShape {
    pub fn to_collider(&self, size: Vec2) -> Collider {
        let half_size = size / 2.0;
        match self {
            EntityShape::Rectangle => Collider::rectangle(size.x, size.y),
            EntityShape::Circle => Collider::circle(half_size.x.max(half_size.y)),
            EntityShape::Triangle => {
                let vertices = generate_regular_polygon_vertices(3, half_size.x.max(half_size.y));
                Collider::convex_hull(vertices).unwrap()
            }
            EntityShape::Pentagon => {
                let vertices = generate_regular_polygon_vertices(5, half_size.x.max(half_size.y));
                Collider::convex_hull(vertices).unwrap()
            }
            EntityShape::Hexagon => {
                let vertices = generate_regular_polygon_vertices(6, half_size.x.max(half_size.y));
                Collider::convex_hull(vertices).unwrap()
            }
            EntityShape::Star => {
                let radius = half_size.x.max(half_size.y);
                let vertices = generate_star_vertices(10, radius, radius / 2.0);
                Collider::convex_hull(vertices).unwrap()
            }
            EntityShape::Diamond => {
                let vertices = generate_regular_polygon_vertices(4, half_size.x.max(half_size.y));
                Collider::convex_hull(vertices).unwrap()
            }
            EntityShape::Cross => {
                let vertices = generate_cross_vertices(size.x, size.y);
                Collider::convex_hull(vertices).unwrap()
            }
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            EntityShape::Rectangle => "Rectangle",
            EntityShape::Circle => "Circle",
            EntityShape::Triangle => "Triangle",
            EntityShape::Pentagon => "Pentagon",
            EntityShape::Hexagon => "Hexagon",
            EntityShape::Star => "Star",
            EntityShape::Diamond => "Diamond",
            EntityShape::Cross => "Cross",
        }
    }
}
