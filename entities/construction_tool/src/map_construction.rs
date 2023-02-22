use bevy::{
    prelude::{
        info, warn, AssetServer, BuildChildren, Commands, Component, Entity, EventReader,
        EventWriter, Input, KeyCode, Query, Res, ResMut, Resource, Transform, Vec3, Visibility,
        With,
    },
    scene::SceneBundle,
    transform::TransformBundle,
};
use bevy_rapier3d::prelude::{
    Collider, CollisionGroups, Group, QueryFilter, RapierContext, RigidBody,
};
use cameras::{controllers::fps::ActiveCamera, LookTransform};
use entity::spawn::ClientEntityServerEntity;
use gridmap::grid::{Cell, Gridmap};
use inventory::server::inventory::Inventory;
use math::grid::Vec3Int;
use physics::physics::{get_bit_masks, ColliderGroup};

use crate::construction_tool::ConstructionTool;
#[derive(Component)]
pub struct SelectCellCameraYPlane;

pub fn create_select_cell_cam_state(mut commands: Commands, asset_server: Res<AssetServer>) {
    let plane_asset = asset_server.load("models/ylevel_grid_plane/plane.glb#Scene0");

    let masks = get_bit_masks(ColliderGroup::GridmapSelection);

    let plane_entity = commands
        .spawn(RigidBody::Fixed)
        .insert(SelectCellCameraYPlane)
        .insert(SceneBundle {
            scene: plane_asset,
            visibility: Visibility::INVISIBLE,
            transform: Transform::from_xyz(0.5, 0.2, 0.5),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Collider::halfspace(Vec3::Y).unwrap())
                .insert(TransformBundle::from(Transform::IDENTITY))
                .insert(CollisionGroups::new(
                    Group::from_bits(masks.0).unwrap(),
                    Group::from_bits(masks.1).unwrap(),
                ));
        })
        .id();
    commands.insert_resource(SelectCellCameraState {
        selected: None,
        y_level: 0,
        y_plane: plane_entity,
        y_plane_shown: false,
    });
}

#[derive(Resource)]
pub struct SelectCellCameraState {
    pub selected: Option<CellSelection>,
    pub y_level: i16,
    pub y_plane: Entity,
    pub y_plane_shown: bool,
}

pub struct CellSelection {
    pub entity: Entity,
    pub id: Vec3Int,
}

pub struct ShowYLevelPlane {
    pub show: bool,
}
pub(crate) fn show_ylevel_plane(
    mut events: EventReader<ShowYLevelPlane>,
    mut state: ResMut<SelectCellCameraState>,
    mut query: Query<&mut Visibility, With<SelectCellCameraYPlane>>,
    mut events2: EventWriter<SetYPlanePosition>,
) {
    for event in events.iter() {
        match query.get_mut(state.y_plane) {
            Ok(mut visibility) => {
                visibility.is_visible = event.show;
                state.y_plane_shown = event.show;
                if event.show {
                    events2.send(SetYPlanePosition { y: state.y_level });
                }
            }
            Err(_) => {
                warn!("Coudlnt find yplane.");
            }
        }
    }
}

pub struct SetYPlanePosition {
    pub y: i16,
}

pub(crate) fn set_yplane_position(
    mut events: EventReader<SetYPlanePosition>,
    mut state: ResMut<SelectCellCameraState>,
    mut query: Query<&mut Transform, With<SelectCellCameraYPlane>>,
) {
    for event in events.iter() {
        state.y_level = event.y;
        match query.get_mut(state.y_plane) {
            Ok(mut transform) => {
                transform.translation.y = event.y as f32 + 0.2;
            }
            Err(_) => {
                warn!("Couldnt query plane.");
            }
        }
    }
}

pub(crate) fn input_yplane_position(
    keys: Res<Input<KeyCode>>,
    state: Res<SelectCellCameraState>,
    mut events: EventWriter<SetYPlanePosition>,
) {
    if keys.just_pressed(KeyCode::Q) {
        events.send(SetYPlanePosition {
            y: state.y_level - 1,
        });
    }
    if keys.just_pressed(KeyCode::E) {
        events.send(SetYPlanePosition {
            y: state.y_level + 1,
        });
    }
}

pub(crate) fn select_cell_in_front_camera(
    camera_query: Query<&LookTransform>,
    active_camera: Res<ActiveCamera>,
    inventory: Res<Inventory>,
    construction_tool_query: Query<&ConstructionTool>,
    gridmap: Res<Gridmap>,
    rapier_context: Res<RapierContext>,
    map: Res<ClientEntityServerEntity>,
    cells_query: Query<&Cell>,
    mut state: ResMut<SelectCellCameraState>,
    mut yplane: EventWriter<ShowYLevelPlane>,
) {
    let active_inventory_entity;

    match inventory.active_item {
        Some(active_inventory_item_server) => match map.map.get(&active_inventory_item_server) {
            Some(active_inventory_item) => {
                active_inventory_entity = *active_inventory_item;
            }
            None => {
                warn!("Couldnt get client entity from map.");
                return;
            }
        },
        None => {
            return;
        }
    }
    let construction_tool_component;

    match construction_tool_query.get(active_inventory_entity) {
        Ok(component) => {
            construction_tool_component = component;
        }
        Err(_) => {
            return;
        }
    }

    if !state.y_plane_shown {
        yplane.send(ShowYLevelPlane { show: true });
    }

    let camera_look_transform;

    match active_camera.option {
        Some(camera_entity) => match camera_query.get(camera_entity) {
            Ok(transform) => {
                camera_look_transform = transform;
            }

            Err(_) => {
                warn!("Couldnt query active camera.");
                return;
            }
        },
        None => {
            warn!("No active camera found");
            return;
        }
    }
    let ray_dir;
    match camera_look_transform.look_direction() {
        Some(dir) => {
            ray_dir = dir;
        }
        None => {
            warn!("Couldnt get camera look_direction()");
            return;
        }
    }

    let ray_pos = camera_look_transform.eye;
    let max_toi = 10.0;
    let solid = true;
    let collider_groups = get_bit_masks(ColliderGroup::GridmapSelection);

    let filter = QueryFilter::new().groups(CollisionGroups::new(
        Group::from_bits(collider_groups.0).unwrap(),
        Group::from_bits(collider_groups.1).unwrap(),
    ));

    let mut intersected_cell_id = None;

    rapier_context.intersections_with_ray(
        ray_pos,
        ray_dir,
        max_toi,
        solid,
        filter,
        |entity, intersection| {
            // Callback called on each collider hit by the ray.
            let hit_point = intersection.point;
            let hit_normal = intersection.normal;
            println!(
                "Entity {:?} hit at point {} with normal {}",
                entity, hit_point, hit_normal
            );

            match cells_query.get(entity) {
                Ok(cell) => {
                    intersected_cell_id = Some(cell.id);
                    return false;
                }
                Err(_) => {}
            }

            true // Return `false` instead if we want to stop searching for other hits.
        },
    );

    match intersected_cell_id {
        Some(id) => {}
        None => {}
    }
}
