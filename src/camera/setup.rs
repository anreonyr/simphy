use bevy::{camera::visibility::RenderLayers, prelude::*};
use bevy_egui::{EguiGlobalSettings, PrimaryEguiContext};

use super::components::{UiCamera, WorldCamera};

pub fn setup_camera(mut commands: Commands, mut egui_global_settings: ResMut<EguiGlobalSettings>) {
    egui_global_settings.auto_create_primary_context = false;

    commands.spawn((
        WorldCamera,
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
    ));

    commands.spawn((
        UiCamera,
        Camera2d,
        PrimaryEguiContext,
        RenderLayers::none(),
        Camera {
            order: 1,
            ..default()
        },
    ));
}
