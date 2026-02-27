use super::setup;
use crate::shared::utils;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::shared::WorldMousePosition>()
            .add_systems(Startup, setup::setup_camera)
            .add_systems(Update, utils::update_world_mouse_position);
    }
}
