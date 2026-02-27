use avian2d::prelude::{LinearVelocity, RigidBody};
use bevy::prelude::*;
use bevy_egui::egui::{self};

use crate::{
    camera::WorldCamera,
    editor::{EntityName, FieldType, PlacementState, SelectionState},
    simulation::components::{Charge, Electric, Field, Magnetic},
};

pub fn editor(
    ui: &mut egui::Ui,
    placement: Option<&mut PlacementState>,
    mut selection: ResMut<SelectionState>,
    entity_query: Query<
        (
            Entity,
            &Transform,
            Option<&EntityName>,
            Option<&Charge>,
            Option<&LinearVelocity>,
            Option<&RigidBody>,
        ),
        (With<crate::editor::EditorEntity>, Without<WorldCamera>),
    >,
    field_query: Query<
        (
            Entity,
            &Transform,
            &Field,
            Option<&Magnetic>,
            Option<&Electric>,
        ),
        Without<WorldCamera>,
    >,
    // mut commands: Commands,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label("Editor");
        ui.separator();

        let placement = match placement {
            Some(placement) => placement,
            None => return,
        };

        egui::CollapsingHeader::new("Entity")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("entity_grid")
                    .num_columns(2)
                    .spacing([8.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("RigidType:");
                        ui.horizontal(|ui| {
                            ui.radio_value(
                                &mut placement.rigid_body,
                                RigidBody::Dynamic,
                                "Dynamic",
                            );
                            ui.radio_value(&mut placement.rigid_body, RigidBody::Static, "Static");
                            ui.radio_value(
                                &mut placement.rigid_body,
                                RigidBody::Kinematic,
                                "Kinematic",
                            );
                        });
                        ui.end_row();

                        ui.label("Pos:");
                        ui.horizontal(|ui| {
                            ui.label("X");
                            ui.add(
                                egui::DragValue::new(&mut placement.transform.translation.x)
                                    .speed(1.0),
                            );
                            ui.label("Y");
                            ui.add(
                                egui::DragValue::new(&mut placement.transform.translation.y)
                                    .speed(1.0),
                            );
                        });
                        ui.end_row();

                        ui.label("Mass:");
                        ui.add(egui::DragValue::new(&mut placement.mass).speed(1.0));
                        ui.end_row();

                        ui.label("Charge:");
                        ui.add(egui::DragValue::new(&mut placement.charge.value).speed(1.0));
                        ui.end_row();

                        ui.label("Dynamic Friction:");
                        ui.add(
                            egui::DragValue::new(&mut placement.friction.dynamic_coefficient)
                                .speed(0.1),
                        );
                        ui.end_row();

                        ui.label("Static Friction:");
                        ui.add(
                            egui::DragValue::new(&mut placement.friction.static_coefficient)
                                .speed(0.1),
                        );
                        ui.end_row();

                        ui.label("Init Vel:");
                        ui.horizontal(|ui| {
                            ui.label("X");
                            ui.add(
                                egui::DragValue::new(&mut placement.initial_velocity.x).speed(1.0),
                            );
                            ui.label("Y");
                            ui.add(
                                egui::DragValue::new(&mut placement.initial_velocity.y).speed(1.0),
                            );
                        });
                        ui.end_row();

                        ui.label("Constant Force:");
                        ui.horizontal(|ui| {
                            ui.label("X");
                            ui.add(
                                egui::DragValue::new(&mut placement.constant_force.x).speed(1.0),
                            );
                            ui.label("Y");
                            ui.add(
                                egui::DragValue::new(&mut placement.constant_force.y).speed(1.0),
                            );
                        });
                        ui.end_row();

                        ui.label("Field:");
                        ui.horizontal(|ui| {
                            ui.radio_value(
                                &mut placement.field_type,
                                crate::editor::FieldType::None,
                                "None",
                            );
                            ui.radio_value(
                                &mut placement.field_type,
                                crate::editor::FieldType::Electric,
                                "Electric",
                            );
                            ui.radio_value(
                                &mut placement.field_type,
                                crate::editor::FieldType::Magnetic,
                                "Magnetic",
                            );
                        });
                        ui.end_row();

                        if placement.field_type != FieldType::None {
                            ui.label("Strength:");
                            ui.add(egui::DragValue::new(&mut placement.field.strength).speed(1.0));
                            ui.end_row();

                            ui.label("Dir:");
                            ui.horizontal(|ui| {
                                ui.label("X");
                                ui.add(
                                    egui::DragValue::new(&mut placement.field.direction.x)
                                        .speed(0.1),
                                );
                                ui.label("Y");
                                ui.add(
                                    egui::DragValue::new(&mut placement.field.direction.y)
                                        .speed(0.1),
                                );
                                ui.label("Z");
                                ui.add(
                                    egui::DragValue::new(&mut placement.field.direction.z)
                                        .speed(0.1),
                                );
                            });
                        }

                        ui.end_row();

                        ui.label("Size:");
                        ui.horizontal(|ui| {
                            ui.label("W");
                            ui.add(egui::DragValue::new(&mut placement.size.x).speed(1.0));
                            ui.label("H");
                            ui.add(egui::DragValue::new(&mut placement.size.y).speed(1.0));
                        });
                        ui.end_row();
                        let shapes = [
                            ("Rect", crate::shared::EntityShape::Rectangle),
                            ("Circle", crate::shared::EntityShape::Circle),
                            ("Tri", crate::shared::EntityShape::Triangle),
                            ("Pent", crate::shared::EntityShape::Pentagon),
                            ("Hex", crate::shared::EntityShape::Hexagon),
                            ("Star", crate::shared::EntityShape::Star),
                            ("Diamond", crate::shared::EntityShape::Diamond),
                            ("Cross", crate::shared::EntityShape::Cross),
                        ];

                        ui.label("Shape:");
                        ui.horizontal_wrapped(|ui| {
                            for (name, shape) in shapes {
                                ui.radio_value(&mut placement.shape, shape, name);
                            }
                        });
                    });
            });

        egui::CollapsingHeader::new("Properties")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(selected) = selection.selected_entity {
                    if let Ok((entity, transform, name, charge, velocity, rigid_body)) =
                        entity_query.get(selected)
                    {
                        let entity_name = name.map(|n| n.0.as_str()).unwrap_or("Unknown");

                        egui::Grid::new("properties_grid")
                            .num_columns(2)
                            .spacing([10.0, 5.0])
                            .show(ui, |ui| {
                                ui.label("Name:");
                                ui.label(entity_name);
                                ui.end_row();

                                ui.label("Entity:");
                                ui.label(format!("{}", entity.index()));
                                ui.end_row();

                                ui.label("Position:");
                                ui.label(format!(
                                    "({:.1}, {:.1})",
                                    transform.translation.x, transform.translation.y
                                ));
                                ui.end_row();

                                if let Some(vel) = velocity {
                                    ui.label("Velocity:");
                                    ui.label(format!("({:.1}, {:.1})", vel.x, vel.y));
                                    ui.end_row();
                                }

                                if let Some(rb) = rigid_body {
                                    ui.label("Body:");
                                    ui.label(format!("{:?}", rb));
                                    ui.end_row();
                                }

                                if let Some(cha) = charge {
                                    ui.label("Charge:");
                                    ui.label(format!("{:?}", cha.value));
                                    ui.end_row();
                                }
                            });
                    } else {
                        ui.label("Selected entity not found");
                    }
                } else {
                    ui.label("No entity selected");
                }
            });

        // [todo] 交互逻辑有问题
        egui::CollapsingHeader::new("Outliner")
            .default_open(true)
            .show(ui, |ui| {
                for (entity, _transform, name, _charge, _velocity, _rigid_body) in
                    entity_query.iter()
                {
                    let is_selected = selection.selected_entity == Some(entity);
                    let display_name = name
                        .map(|n| n.0.clone())
                        .unwrap_or_else(|| format!("Entity #{}", entity.index()));

                    let response = ui.selectable_label(is_selected, display_name);

                    if response.clicked() {
                        selection.selected_entity = Some(entity);
                    }
                }
            });

        egui::CollapsingHeader::new("Fields")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("fields_grid")
                    .num_columns(2)
                    .spacing([10.0, 5.0])
                    .show(ui, |ui| {
                        for (entity, transform, field, magnetic, electric) in field_query.iter() {
                            let field_type = if magnetic.is_some() {
                                "Magnetic"
                            } else if electric.is_some() {
                                "Electric"
                            } else {
                                "Unknown"
                            };

                            ui.label(format!("{} #{}", field_type, entity.index()));
                            ui.label(format!(
                                "({:.0}, {:.0})",
                                transform.translation.x, transform.translation.y
                            ));
                            ui.end_row();

                            ui.label("Strength:");
                            ui.label(format!("{:.1}", field.strength));
                            ui.end_row();

                            ui.label("Direction:");
                            ui.label(format!(
                                "({:.1}, {:.1}, {:.1})",
                                field.direction.x, field.direction.y, field.direction.z
                            ));
                            ui.end_row();
                        }
                    });
            });
    });
}
