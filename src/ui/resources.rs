use bevy::prelude::Resource;
use bevy_egui::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Default)]
pub struct ExitRequest(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource)]
pub struct UiPanelVisibility {
    pub toolbar: bool,
    pub editor: bool,
    pub statusbar: bool,
}

impl Default for UiPanelVisibility {
    fn default() -> Self {
        UiPanelVisibility {
            toolbar: true,
            editor: true,
            statusbar: true,
        }
    }
}

#[derive(Resource)]
pub struct UiState;

impl Default for UiState {
    fn default() -> Self {
        UiState
    }
}

#[derive(Resource)]
pub struct GameViewTab {
    pub viewport_rect: egui::Rect,
    pub visible: bool,
    pub mouse_in: bool,
    pub scale_factor: f32,
}

impl Default for GameViewTab {
    fn default() -> Self {
        Self {
            viewport_rect: egui::Rect::NOTHING,
            visible: false,
            mouse_in: false,
            scale_factor: 1.0,
        }
    }
}
