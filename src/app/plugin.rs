use bevy::prelude::*;
use bevy::window::WindowResolution;

use super::state::{reset_simulation, SimulationState};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::Windowed,
                decorations: true,
                resolution: WindowResolution::new(1280, 720),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SimulationState>()
        .add_systems(PreUpdate, reset_simulation);
    }
}
