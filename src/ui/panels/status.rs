use bevy_egui::egui;

use crate::app::SimulationState;

pub fn statusbar(ui: &mut egui::Ui, state: &mut SimulationState) {
    ui.horizontal(|ui| {
        if state.is_running {
            if ui.button("⏸").clicked() {
                state.is_running = false;
            }
        } else {
            if ui.button("▶").clicked() {
                state.is_running = true;
            }
        }

        ui.separator();

        if ui.button("↺").clicked() {
            state.needs_reset = true;
            state.is_running = false;
        }

        ui.separator();

        let speed_options = ["0.25x", "0.5x", "1x", "2x"];
        let current_index = match state.time_scale {
            0.25 => 0,
            0.5 => 1,
            1.0 => 2,
            2.0 => 3,
            _ => 2,
        };

        egui::ComboBox::from_id_salt("speed_selector")
            .selected_text(speed_options[current_index])
            .show_ui(ui, |ui| {
                for option in speed_options.iter() {
                    let speed = match *option {
                        "0.25x" => 0.25,
                        "0.5x" => 0.5,
                        "1x" => 1.0,
                        "2x" => 2.0,
                        _ => 1.0,
                    };
                    ui.selectable_value(&mut state.time_scale, speed, *option);
                }
            });
    });
}
