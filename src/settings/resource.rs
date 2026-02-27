use bevy::prelude::*;

use super::editor_prefs::EditorPrefs;
use super::ui_settings::UiSettings;

#[derive(Resource)]
pub struct Settings {
    pub editor: EditorPrefs,
    pub ui: UiSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: EditorPrefs::default(),
            ui: UiSettings::default(),
        }
    }
}
