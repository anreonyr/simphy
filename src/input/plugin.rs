use std::f32;

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::app::SimulationState;
use crate::camera::components::WorldCamera;
use crate::editor::PlacementState;
use crate::editor::Tool;
use crate::ui::GameViewTab;

#[derive(Default)]
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<BoardContext>()
            .add_input_context::<MouseContext>()
            .add_observer(handle_toggle_simulation)
            .add_observer(handle_camera_pan)
            .add_observer(handle_camera_zoom)
            .add_systems(Startup, setup_context);
    }
}

#[derive(Component)]
pub struct BoardContext;

#[derive(Component)]
pub struct MouseContext;

fn setup_context(mut commands: Commands) {
    commands.spawn((
        BoardContext,
        actions!(BoardContext[
            (Action::<ToggleSimulation>::new(), bindings![KeyCode::Space]),
        ]),
    ));
    commands.spawn((
        MouseContext,
        Actions::<MouseContext>::spawn(SpawnWith(
            |ctx: &mut bevy::ecs::relationship::RelatedSpawner<'_, ActionOf<MouseContext>>| {
                let mouseleft = ctx
                    .spawn((Action::<MouseLeft>::new(), bindings![MouseButton::Left]))
                    .id();
                ctx.spawn((
                    Action::<CameraPan>::new(),
                    Chord::single(mouseleft),
                    bindings![(Binding::mouse_motion(), Negate::x())],
                ));
                ctx.spawn((
                    Action::<CameraZoom>::new(),
                    Scale::splat(0.1),
                    Negate::x(),
                    bindings![(Binding::mouse_wheel(), SwizzleAxis::YXZ)],
                ));
            },
        )),
    ));
}

#[derive(InputAction)]
#[action_output(bool)]
pub struct ToggleSimulation;

#[derive(InputAction)]
#[action_output(bool)]
struct MouseLeft;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct CameraPan;

#[derive(InputAction)]
#[action_output(f32)]
pub struct CameraZoom;

pub fn handle_toggle_simulation(
    _action: On<Start<ToggleSimulation>>,
    mut state: ResMut<SimulationState>,
) {
    state.is_running = !state.is_running;
}

pub fn handle_camera_pan(
    action: On<Fire<CameraPan>>,
    mut camera: Single<&mut Transform, With<WorldCamera>>,
    game_view_tab: Res<GameViewTab>,
    placement: Res<PlacementState>,
) {
    if !game_view_tab.mouse_in || placement.tool != Tool::Pan {
        return;
    }
    let scale = camera.scale.x;
    camera.translation += action.value.extend(0.0) * scale;
}

pub fn handle_camera_zoom(
    action: On<Fire<CameraZoom>>,
    mut camera: Single<&mut Transform, With<WorldCamera>>,
    game_view_tab: Res<GameViewTab>,
) {
    if !game_view_tab.mouse_in {
        return;
    }
    let zoom_factor = 1.0 + action.value;
    let new_scale = (camera.scale.x * zoom_factor).clamp(0.1, f32::INFINITY);
    camera.scale = Vec3::splat(new_scale);
}
