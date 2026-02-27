use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::shared::InitialState;
use crate::editor::components::EditorEntity;

#[derive(Resource)]
pub struct SimulationState {
    pub is_running: bool,
    pub time_scale: f32,
    pub needs_reset: bool,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            is_running: false,
            time_scale: 1.0,
            needs_reset: false,
        }
    }
}

pub fn reset_simulation(
    mut query: Query<(&InitialState, &mut Transform, &mut LinearVelocity), With<EditorEntity>>,
    mut state: ResMut<SimulationState>,
) {
    if state.needs_reset {
        state.needs_reset = false;
        for (initial_state, mut transform, mut velocity) in query.iter_mut() {
            *transform = initial_state.transform;
            *velocity = LinearVelocity(initial_state.velocity);
        }
    }
}
