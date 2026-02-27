use avian2d::prelude::*;
use bevy::prelude::*;

use super::field;
use crate::app::SimulationState;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            PhysicsPlugins::default()
                .with_length_unit(10.0)
                .set(PhysicsInterpolationPlugin::interpolate_all()),
        )
        .add_plugins(PhysicsDebugPlugin::default())
        // .insert_gizmo_config(PhysicsGizmos::all(), GizmoConfig::default())
        .insert_resource(Gravity(Vec2::NEG_Y * 100.0))
        .add_systems(
            PhysicsSchedule,
            (field::apply_magnetic_force, field::apply_electric_force)
                .chain()
                .before(PhysicsStepSystems::First),
        )
        .add_systems(Update, update_simulation);
    }
}

fn update_simulation(state: Res<SimulationState>, mut physics_time: ResMut<Time<Physics>>) {
    physics_time.set_relative_speed(state.time_scale);
    if state.is_running {
        physics_time.unpause();
    } else {
        physics_time.pause();
    }
}
