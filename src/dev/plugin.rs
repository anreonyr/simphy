use super::setup;
use bevy::prelude::*;

#[derive(Default)]
pub struct DevSetupPlugin;

impl Plugin for DevSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup::dev_setup);
    }
}
