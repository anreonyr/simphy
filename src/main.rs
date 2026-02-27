pub mod app;
pub mod camera;
#[cfg(feature = "dev")]
pub mod dev;
pub mod editor;
pub mod input;
pub mod project;
pub mod settings;
pub mod shared;
pub mod simulation;
pub mod ui;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(app::AppPlugin)
        .add_plugins(simulation::SimulationPlugin)
        .add_plugins(input::ActionPlugin)
        .add_plugins(editor::EditorPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(camera::CameraPlugin);

    #[cfg(feature = "dev")]
    app.add_plugins(dev::DevSetupPlugin);

    app.run();
}
