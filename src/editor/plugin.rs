use avian2d::prelude::*;
use bevy::prelude::*;

use super::components::{EditorEntity, EntityName, PlacementIndicator, SelectedEntity};
use super::resources::{
    DragState, EntityProperties, PlacedEntities, PlacementState, RigidBodyType, SelectionState,
    Tool,
};
use crate::editor::FieldType;
use crate::shared::{EntityShape, InitialState, WorldMousePosition};
use crate::simulation::components::{Electric, Field, Magnetic};
use crate::ui::GameViewTab;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlacementState>()
            .init_resource::<SelectionState>()
            .init_resource::<EntityProperties>()
            .init_resource::<PlacedEntities>()
            .init_resource::<DragState>()
            .add_systems(
                Update,
                (
                    spawn_placement_indicator,
                    update_placement_indicator,
                    place_entity,
                    select_entity,
                    drag_entity,
                    update_entity_properties,
                    update_selected_entity_visual,
                ),
            );
    }
}

// [todo] 需要重构AI写的依托四

fn spawn_placement_indicator(
    mut commands: Commands,
    placement: Res<PlacementState>,
    indicator_query: Query<Entity, With<PlacementIndicator>>,
) {
    if indicator_query.is_empty() && placement.tool == Tool::Place {
        commands.spawn((
            PlacementIndicator {
                shape: placement.shape,
                size: placement.size,
            },
            Transform::from_xyz(0.0, 0.0, 100.0),
        ));
    }
}

fn update_placement_indicator(
    mut indicator_query: Query<(&mut PlacementIndicator, &mut Transform), With<PlacementIndicator>>,
    placement: Res<PlacementState>,
    world_mouse: Res<WorldMousePosition>,
    game_view_tab: Res<GameViewTab>,
) {
    if placement.tool != Tool::Place {
        return;
    }

    if let Some(pos) = world_mouse.position {
        if game_view_tab.mouse_in {
            for (mut indicator, mut transform) in indicator_query.iter_mut() {
                if indicator.shape != placement.shape || indicator.size != placement.size {
                    indicator.shape = placement.shape;
                    indicator.size = placement.size;
                }
                transform.translation.x = pos.x;
                transform.translation.y = pos.y;
            }
        }
    }
}

fn place_entity(
    mut commands: Commands,
    placement: Res<PlacementState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    world_mouse: Res<WorldMousePosition>,
    mut placed_entities: ResMut<PlacedEntities>,
    game_view_tab: Res<GameViewTab>,
) {
    if placement.tool != Tool::Place {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left)
        && game_view_tab.mouse_in
        && world_mouse.position.is_some()
    {
        let size = placement.size;

        let collider = placement.shape.to_collider(size);

        let transform = Transform::from_translation(world_mouse.position.unwrap().extend(0.0));

        // [todo] 输入的世界坐标不可能应用

        let mut entity_commands = commands.spawn((
            EditorEntity,
            EntityName(format!(
                "{} {}",
                placement.shape.display_name(),
                placed_entities.entities.len() + 1
            )),
            transform,
            collider,
            placement.charge,
            placement.rigid_body,
            placement.shape,
            placement.initial_velocity,
            placement.friction,
            placement.constant_force.to_owned(),
            InitialState::new(placement.transform, transform.translation.truncate()),
        ));

        if placement.field_type != FieldType::None {
            entity_commands.insert(Sensor);
            entity_commands.insert(placement.field);
            entity_commands.insert(CollisionEventsEnabled);
            entity_commands.insert(CollidingEntities::default());
            match placement.field_type {
                FieldType::Magnetic => {
                    entity_commands.insert(Magnetic);
                }
                FieldType::Electric => {
                    entity_commands.insert(Electric);
                }
                _ => {}
            }
        } else {
            // [IMPORTANT] 场不能有Mass
            entity_commands.insert(Mass(placement.mass));
        }

        let entity = entity_commands.id();

        placed_entities.entities.push(entity);
    }
}

fn select_entity(
    mouse_button: Res<ButtonInput<MouseButton>>,
    world_mouse: Res<WorldMousePosition>,
    placement: Res<PlacementState>,
    mut selection: ResMut<SelectionState>,
    mut properties: ResMut<EntityProperties>,
    mut commands: Commands,
    mut placed_entities: ResMut<PlacedEntities>,
    mut document: ResMut<crate::project::Document>,
    query: Query<(Entity, &Transform, &EntityShape), With<EditorEntity>>,
    game_view_tab: Res<GameViewTab>,
) {
    if mouse_button.just_pressed(MouseButton::Left) && game_view_tab.mouse_in {
        if let Some(world_pos) = world_mouse.position {
            let mut closest_entity: Option<Entity> = None;
            let mut closest_dist = f32::MAX;

            for (entity, transform, _shape) in query.iter() {
                let dist = (transform.translation.truncate() - world_pos).length();
                let radius = 25.0;

                if dist < radius && dist < closest_dist {
                    closest_dist = dist;
                    closest_entity = Some(entity);
                }
            }

            if let Some(entity) = closest_entity {
                if placement.tool == Tool::Delete {
                    commands.entity(entity).despawn();
                    placed_entities.entities.retain(|e| *e != entity);
                    selection.selected_entity = None;
                    properties.entity = None;
                    document.is_dirty = true;
                } else {
                    selection.selected_entity = Some(entity);
                    if let Ok((_entity, transform, shape)) = query.get(entity) {
                        properties.entity = Some(entity);
                        properties.position = transform.translation;
                        properties.scale = Vec2::ONE * 50.0;
                        properties.shape = *shape;
                        properties.rigid_body_type = RigidBodyType::Dynamic;
                    }
                }
            } else {
                selection.selected_entity = None;
                properties.entity = None;
            }
        }
    }
}

fn drag_entity(
    placement: Res<PlacementState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    world_mouse: Res<WorldMousePosition>,
    selection: Res<SelectionState>,
    mut drag_state: ResMut<DragState>,
    mut query: Query<&mut Transform, With<EditorEntity>>,
    game_view_tab: Res<GameViewTab>,
) {
    if placement.tool != Tool::Move {
        return;
    }

    if !game_view_tab.mouse_in {
        return;
    }

    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(entity) = selection.selected_entity {
            if let Ok(transform) = query.get(entity) {
                if let Some(pos) = world_mouse.position {
                    let entity_pos = transform.translation.truncate();
                    drag_state.dragging = true;
                    drag_state.entity = Some(entity);
                    drag_state.offset = entity_pos - pos;
                }
            }
        }
    }

    if mouse_button.pressed(MouseButton::Left) && drag_state.dragging {
        if let Some(entity) = drag_state.entity {
            if let Some(pos) = world_mouse.position {
                if let Ok(mut transform) = query.get_mut(entity) {
                    let new_pos = pos + drag_state.offset;
                    transform.translation.x = new_pos.x;
                    transform.translation.y = new_pos.y;
                }
            }
        }
    }

    if mouse_button.just_released(MouseButton::Left) {
        drag_state.dragging = false;
        drag_state.entity = None;
    }
}

fn update_entity_properties(
    mut properties: ResMut<EntityProperties>,
    query: Query<&Transform, With<SelectedEntity>>,
) {
    if let Some(entity) = properties.entity {
        if let Ok(transform) = query.get(entity) {
            properties.position = transform.translation;
        }
    }
}

fn update_selected_entity_visual(
    mut commands: Commands,
    selection: Res<SelectionState>,
    selected_query: Query<Entity, With<SelectedEntity>>,
) {
    for entity in selected_query.iter() {
        commands.entity(entity).remove::<SelectedEntity>();
    }

    if let Some(entity) = selection.selected_entity {
        commands.entity(entity).insert(SelectedEntity);
    }
}
