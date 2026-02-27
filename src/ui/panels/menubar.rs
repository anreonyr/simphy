use avian2d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;
use bevy_egui::egui;
use rfd::FileDialog;

use crate::{
    camera::components::WorldCamera,
    editor::{EntityName, PlacementState, components::EditorEntity, resources::SelectionState},
    project::{document::Document, export, import},
    settings::editor_prefs::EditorPrefs,
    shared::{EntityShape, InitialState},
    simulation::components::{Charge, Electric, Field, Magnetic},
    ui::UiPanelVisibility,
};

static SHOW_ABOUT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static SHOW_DOCS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn menubar(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    visibility: &mut UiPanelVisibility,
    mut camera: Single<&mut Transform, With<WorldCamera>>,
    editor_prefs: &mut EditorPrefs,
    document: &mut ResMut<Document>,
    selection: &mut ResMut<SelectionState>,
    placement: &mut ResMut<PlacementState>,
    placed_entities: &mut ResMut<crate::editor::resources::PlacedEntities>,
    // entity_query: Query<
    //     (
    //         Entity,
    //         &Transform,
    //         Option<&crate::editor::EntityName>,
    //         Option<&Charge>,
    //         Option<&LinearVelocity>,
    //         Option<&RigidBody>,
    //     ),
    //     (With<EditorEntity>, Without<WorldCamera>),
    // >,
    save_entity_query: Query<
        (
            Entity,
            &Transform,
            Option<&crate::editor::EntityName>,
            Option<&Charge>,
            Option<&LinearVelocity>,
            Option<&RigidBody>,
            Option<&Magnetic>,
            Option<&Electric>,
            Option<&Field>,
        ),
        (With<EditorEntity>, Without<WorldCamera>),
    >,
    commands: &mut Commands,
) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("New").clicked() {
                for entity in placed_entities.entities.iter() {
                    commands.entity(*entity).despawn();
                }
                placed_entities.entities.clear();
                selection.selected_entity = None;
                **placement = PlacementState::default();
                document.path = None;
                document.is_dirty = false;
                document.data = crate::project::file_format::SceneData::default();
            }
            if ui.button("Open").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("YAML", &["yaml", "yml"])
                    .add_filter("RON", &["ron"])
                    .pick_file()
                {
                    match import::import_scene(&path) {
                        Ok(data) => {
                            document.data = data;
                            document.path = Some(path);
                            document.is_dirty = false;
                        }
                        Err(e) => {
                            bevy::log::error!("Failed to open file: {}", e);
                        }
                    }
                }
            }
            if ui.button("Save").clicked() {
                if let Some(ref path) = document.path {
                    let data = collect_scene_data(&save_entity_query);
                    match export::export_scene(path, &data) {
                        Ok(_) => {
                            document.is_dirty = false;
                        }
                        Err(e) => {
                            bevy::log::error!("Failed to save file: {}", e);
                        }
                    }
                } else {
                    if let Some(path) = FileDialog::new()
                        .add_filter("YAML", &["yaml", "yml"])
                        .add_filter("RON", &["ron"])
                        .save_file()
                    {
                        let data = collect_scene_data(&save_entity_query);
                        match export::export_scene(&path, &data) {
                            Ok(_) => {
                                document.path = Some(path);
                                document.is_dirty = false;
                            }
                            Err(e) => {
                                bevy::log::error!("Failed to save file: {}", e);
                            }
                        }
                    }
                }
            }
            if ui.button("Save As").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("YAML", &["yaml", "yml"])
                    .add_filter("RON", &["ron"])
                    .save_file()
                {
                    let data = collect_scene_data(&save_entity_query);
                    match export::export_scene(&path, &data) {
                        Ok(_) => {
                            document.path = Some(path);
                            document.is_dirty = false;
                        }
                        Err(e) => {
                            bevy::log::error!("Failed to save file: {}", e);
                        }
                    }
                }
            }
            ui.separator();
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
        ui.menu_button("Edit", |ui| {
            let can_edit = selection.selected_entity.is_some();
            ui.add_enabled_ui(can_edit, |ui| {
                if ui.button("Delete").clicked() {
                    if let Some(entity) = selection.selected_entity {
                        commands.entity(entity).despawn();
                        placed_entities.entities.retain(|e| *e != entity);
                        selection.selected_entity = None;
                        document.is_dirty = true;
                    }
                }
                if ui.button("Duplicate").clicked() {
                    if let Some(entity) = selection.selected_entity {
                        if let Ok((
                            _,
                            transform,
                            name,
                            _charge,
                            _velocity,
                            _rigid_body,
                            _magnetic,
                            _electric,
                            _field,
                        )) = save_entity_query.get(entity)
                        {
                            let new_name = name
                                .map(|n| format!("{} Copy", n.0))
                                .unwrap_or_else(|| "Entity Copy".to_string());

                            let offset = Vec3::new(50.0, 50.0, 0.0);
                            let mut new_transform =
                                Transform::from_translation(transform.translation + offset);
                            new_transform.scale = transform.scale;

                            let new_entity = commands
                                .spawn((
                                    EditorEntity,
                                    EntityName(new_name),
                                    new_transform,
                                    EntityShape::Circle,
                                    RigidBody::Dynamic,
                                    Collider::circle(25.0),
                                    LinearVelocity::default(),
                                    InitialState {
                                        transform: new_transform,
                                        velocity: Vec2::ZERO,
                                    },
                                ))
                                .id();

                            placed_entities.entities.push(new_entity);
                            selection.selected_entity = Some(new_entity);
                            document.is_dirty = true;
                        }
                    }
                }
            });
        });
        ui.menu_button("View", |ui| {
            if ui.button("Reset Camera").clicked() {
                camera.translation = Vec3::ZERO;
                camera.scale = Vec3::splat(1.0);
            }
            if ui.button("Toggle Grid").clicked() {
                editor_prefs.show_grid = !editor_prefs.show_grid;
            }
            if ui.button("Toggle Debug View").clicked() {
                editor_prefs.show_field_vectors = !editor_prefs.show_field_vectors;
            }
            ui.separator();
            ui.checkbox(&mut visibility.toolbar, "Toolbar");
            ui.checkbox(&mut visibility.editor, "Editor");
            ui.checkbox(&mut visibility.statusbar, "Status Bar");
        });
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                SHOW_ABOUT.store(true, std::sync::atomic::Ordering::SeqCst);
            }
            if ui.button("Documentation").clicked() {
                SHOW_DOCS.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        });
    });

    if SHOW_ABOUT.load(std::sync::atomic::Ordering::SeqCst) {
        egui::Window::new("About SimPhy")
            .collapsible(false)
            .resizable(false)
            .title_bar(true)
            .show(ctx, |ui| {
                ui.label("SimPhy v0.1.0");
                ui.label("2D Physics Simulation");
                ui.separator();
                ui.label("Built with Bevy + avian2d");
                if ui.button("Close").clicked() {
                    SHOW_ABOUT.store(false, std::sync::atomic::Ordering::SeqCst);
                }
            });
    }

    if SHOW_DOCS.load(std::sync::atomic::Ordering::SeqCst) {
        egui::Window::new("Documentation")
            .collapsible(false)
            .resizable(false)
            .title_bar(true)
            .show(ctx, |ui| {
                ui.label("Keyboard Shortcuts");
                ui.separator();
                ui.label("Space - Play/Pause");
                ui.label("R - Reset");
                ui.label("G - Toggle Grid");
                ui.label("D - Toggle Debug View");
                ui.separator();
                ui.label("Mouse:");
                ui.label("Left Click - Select");
                ui.label("Right Click - Place Entity");
                ui.label("Scroll - Zoom");
                ui.label("Middle Drag - Pan");
                if ui.button("Close").clicked() {
                    SHOW_DOCS.store(false, std::sync::atomic::Ordering::SeqCst);
                }
            });
    }
}

fn collect_scene_data(
    query: &Query<
        (
            Entity,
            &Transform,
            Option<&crate::editor::EntityName>,
            Option<&Charge>,
            Option<&LinearVelocity>,
            Option<&RigidBody>,
            Option<&Magnetic>,
            Option<&Electric>,
            Option<&Field>,
        ),
        (With<EditorEntity>, Without<WorldCamera>),
    >,
) -> crate::project::file_format::SceneData {
    use crate::project::file_format::*;

    let mut entities = Vec::new();

    for (_entity, transform, name, charge, velocity, rigid_body, magnetic, electric, field) in
        query.iter()
    {
        let rigid_body_data = rigid_body.map(|rb| RigidBodyData {
            body_type: format!("{:?}", rb),
        });

        let collider_data = None;

        let charge_data = charge.map(|c| c.value);

        let field_data = field.map(|f| {
            let field_type = if magnetic.is_some() {
                "magnetic".to_string()
            } else if electric.is_some() {
                "electric".to_string()
            } else {
                "unknown".to_string()
            };
            FieldData {
                field_type,
                strength: f.strength,
                direction: Vec2::new(f.direction.x, f.direction.y),
            }
        });

        entities.push(SceneEntityData {
            name: name.map(|n| n.0.clone()).unwrap_or_default(),
            transform: TransformData {
                translation: transform.translation,
                rotation: 0.0,
                scale: transform.scale,
            },
            rigid_body: rigid_body_data,
            collider: collider_data,
            charge: charge_data,
            field: field_data,
        });
    }

    SceneData { entities }
}
