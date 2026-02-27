use bevy::{camera::Viewport, prelude::*, window::PrimaryWindow};
use bevy_egui::{
    EguiContextSettings, EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, CentralPanel, SidePanel, TopBottomPanel},
};

use super::panels::{editor, menubar, toolbar};
use super::resources::{GameViewTab, UiPanelVisibility, UiState};
use crate::{
    app::SimulationState,
    camera::components::WorldCamera,
    editor::{PlacementState, SelectionState, resources::PlacedEntities},
    project::document::Document,
    settings::editor_prefs::EditorPrefs,
    simulation::components::{Charge, Electric, Field, Magnetic},
    ui::statusbar,
};
use avian2d::prelude::{LinearVelocity, RigidBody};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .insert_resource(UiState::default())
            .insert_resource(GameViewTab::default())
            .insert_resource(UiPanelVisibility::default())
            .insert_resource(EditorPrefs::default())
            .insert_resource(Document::new())
            .add_systems(
                EguiPrimaryContextPass,
                (
                    ui_system,
                    embed_camera_viewport,
                    draw_cursor,
                    draw_grid,
                    draw_field_vectors,
                )
                    .chain(),
            );
    }
}

fn ui_system(
    mut tab: ResMut<GameViewTab>,
    mut egui_context: EguiContexts,
    mut state: ResMut<SimulationState>,
    mut panel_visibility: ResMut<UiPanelVisibility>,
    mut placement: ResMut<PlacementState>,
    mut selection: ResMut<SelectionState>,
    mut placed_entities: ResMut<PlacedEntities>,
    mut document: ResMut<Document>,
    entity_query: Query<
        (
            Entity,
            &Transform,
            Option<&crate::editor::EntityName>,
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
    camera_query: Single<&mut Transform, With<WorldCamera>>,
    mut editor_prefs: ResMut<EditorPrefs>,
    mut commands: Commands,
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
        (With<crate::editor::EditorEntity>, Without<WorldCamera>),
    >,
) -> Result {
    tab.visible = false;
    let ctx = egui_context.ctx_mut()?;

    TopBottomPanel::top("Menu").show(ctx, |ui| {
        menubar(
            ui,
            ctx,
            &mut panel_visibility,
            camera_query,
            &mut editor_prefs,
            &mut document,
            &mut selection,
            &mut placement,
            &mut placed_entities,
            // entity_query,
            save_entity_query,
            &mut commands,
        );
    });

    if panel_visibility.toolbar {
        TopBottomPanel::top("Tool").show(ctx, |ui| {
            toolbar(ui, &mut placement);
        });
    }

    if panel_visibility.statusbar {
        TopBottomPanel::bottom("Status").show(ctx, |ui| {
            statusbar(ui, state.as_mut());
        });
    }

    if panel_visibility.editor {
        SidePanel::left("Editor")
            .default_width(250.0)
            .show(ctx, |ui| {
                editor::editor(
                    ui,
                    Some(&mut placement),
                    selection,
                    entity_query,
                    field_query,
                    // commands,
                );
            });
    }

    CentralPanel::default()
        .frame(egui::Frame::canvas(&ctx.style()).fill(egui::Color32::TRANSPARENT))
        .show(ctx, |ui| {
            tab.viewport_rect = ui.clip_rect();
            tab.visible = true;
            tab.mouse_in = tab.viewport_rect.contains(
                ui.input(|i| i.pointer.hover_pos())
                    .unwrap_or(egui::Pos2::ZERO),
            );
        });

    Ok(())
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform), With<WorldCamera>>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        gizmos.circle_2d(world_pos, 10., bevy::color::palettes::css::WHITE);
    }
}

fn draw_grid(
    editor_prefs: Res<EditorPrefs>,
    camera_query: Single<(&Camera, &GlobalTransform), With<WorldCamera>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
) {
    if !editor_prefs.show_grid {
        return;
    }

    let (camera, camera_transform) = *camera_query;
    let grid_size = editor_prefs.grid_size;

    let viewport_size = camera
        .viewport
        .as_ref()
        .map(|v| Vec2::new(v.physical_size.x as f32, v.physical_size.y as f32))
        .unwrap_or(window.physical_size().as_vec2());

    let scale = camera_transform.scale().x;
    let camera_pos = camera_transform.translation().truncate();

    let half_width = viewport_size.x / 2.0 * scale;
    let half_height = viewport_size.y / 2.0 * scale;

    let min_x = ((camera_pos.x - half_width) / grid_size).floor() as i32;
    let max_x = ((camera_pos.x + half_width) / grid_size).ceil() as i32;
    let min_y = ((camera_pos.y - half_height) / grid_size).floor() as i32;
    let max_y = ((camera_pos.y + half_height) / grid_size).ceil() as i32;

    let grid_color = LinearRgba::gray(0.05);

    // 绘制垂直线 - 固定在世界坐标
    let mut x = min_x;
    while x <= max_x {
        let world_x = x as f32 * grid_size;
        gizmos.line_2d(
            Vec2::new(world_x, min_y as f32 * grid_size),
            Vec2::new(world_x, max_y as f32 * grid_size),
            grid_color,
        );
        x += 1;
    }

    // 绘制水平线 - 固定在世界坐标
    let mut y = min_y;
    while y <= max_y {
        let world_y = y as f32 * grid_size;
        gizmos.line_2d(
            Vec2::new(min_x as f32 * grid_size, world_y),
            Vec2::new(max_x as f32 * grid_size, world_y),
            grid_color,
        );
        y += 1;
    }
}

fn draw_field_vectors(
    editor_prefs: Res<EditorPrefs>,
    magnetic_query: Query<(&Field, &Transform), With<Magnetic>>,
    electric_query: Query<(&Field, &Transform), With<Electric>>,
    mut gizmos: Gizmos,
) {
    if !editor_prefs.show_field_vectors {
        return;
    }

    let cross_size = 6.0;

    let mag_color = LinearRgba::new(0.0, 0.8, 1.0, 0.8);
    let ele_color = LinearRgba::new(1.0, 0.5, 0.0, 0.8);

    for (field, transform) in magnetic_query.iter() {
        let center = transform.translation.truncate();

        if field.direction.z > 0.0 {
            gizmos.circle_2d(center, 2.0, mag_color);
        } else if field.direction.z < 0.0 {
            gizmos.line_2d(
                center + Vec2::new(-cross_size, -cross_size),
                center + Vec2::new(cross_size, cross_size),
                mag_color,
            );
            gizmos.line_2d(
                center + Vec2::new(-cross_size, cross_size),
                center + Vec2::new(cross_size, -cross_size),
                mag_color,
            );
        }
    }

    for (field, transform) in electric_query.iter() {
        let pos = transform.translation.truncate();
        let dir = Vec2::new(field.direction.x, field.direction.y);
        let len = field.strength * 0.5;
        let dir_normalized = if dir.length() > 0.0 {
            dir.normalize()
        } else {
            Vec2::ZERO
        };

        let end = pos + dir_normalized * len;
        gizmos.line_2d(pos, end, ele_color);
        gizmos.circle_2d(pos, 5.0, ele_color);
    }
}

fn embed_camera_viewport(
    mut game_view_tab: ResMut<GameViewTab>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut camera: Single<&mut Camera, With<WorldCamera>>,
    egui_settings: Single<&EguiContextSettings>,
) {
    if !game_view_tab.visible {
        camera.is_active = false;
        return;
    }

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;
    game_view_tab.scale_factor = scale_factor;
    let viewport_pos = game_view_tab.viewport_rect.left_top().to_vec2() * scale_factor;
    let viewport_size = game_view_tab.viewport_rect.size() * scale_factor;
    let physical_position = UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
    let physical_size = UVec2::new(viewport_size.x as u32, viewport_size.y as u32);
    let rect = physical_position + physical_size;
    let window_size = window.physical_size();

    if rect.x <= window_size.x && rect.y <= window_size.y {
        camera.is_active = true;
        camera.viewport = Some(Viewport {
            physical_position,
            physical_size,
            ..default()
        });
    } else {
        camera.is_active = false;
    }
}
