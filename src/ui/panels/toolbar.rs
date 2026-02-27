use bevy_egui::egui;

use crate::editor::{PlacementState, Tool};

pub fn toolbar(ui: &mut egui::Ui, placement: &mut PlacementState) {
    ui.horizontal(|ui| {
        ui.label("Tool");
        ui.separator();
        ui.selectable_value(&mut placement.tool, Tool::Pan, "Pan");
        ui.selectable_value(&mut placement.tool, Tool::Select, "Select");
        ui.selectable_value(&mut placement.tool, Tool::Move, "Move");
        ui.selectable_value(&mut placement.tool, Tool::Place, "Place");
        ui.selectable_value(&mut placement.tool, Tool::Delete, "Delete");
    });
}
