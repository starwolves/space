use std::{collections::HashMap, f32::consts::PI};

use bevy::{
    prelude::{
        info, warn, AssetServer, BuildChildren, Commands, Component, DespawnRecursiveExt, Entity,
        EventReader, EventWriter, Input, KeyCode, MouseButton, Quat, Query, Res, ResMut, Resource,
        SystemSet, Transform, Vec3, Visibility, With,
    },
    scene::SceneBundle,
    transform::TransformBundle,
};
use bevy_rapier3d::prelude::{
    Collider, CollisionGroups, Group, QueryFilter, RapierContext, RigidBody,
};
use cameras::{controllers::fps::ActiveCamera, LookTransform};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use physics::physics::{get_bit_masks, ColliderGroup};
use resources::{
    binds::{KeyBind, KeyBinds},
    grid::{CellFace, TargetCell},
    hud::HudState,
};
use resources::{
    math::{cell_id_to_world, world_to_cell_id, Vec2Int, Vec3Int},
    ui::TextInput,
};

use crate::{
    grid::{
        CellIds, CellTypeId, Gridmap, GroupTypeId, Orthogonal, OrthogonalBases,
        TargetCellWithOrientationWType,
    },
    net::{ConstructCell, DeconstructCell, GridmapClientMessage, GridmapServerMessage},
};

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
            visibility: Visibility::Hidden,
            transform: Transform::from_xyz(0.5, YPLANE_Y_OFFSET, 0.5),
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

    commands.insert_resource(GridmapConstructionState {
        selected: None,
        y_level: 0,
        y_plane: plane_entity,
        is_constructing: false,
        y_plane_position: Vec2Int { x: 0, y: 0 },
        ghost_item: HashMap::default(),
        group_id: None,
    });
}

#[derive(Resource, Clone)]
pub struct GridmapConstructionState {
    pub selected: Option<Vec3Int>,
    pub y_level: i16,
    pub y_plane: Entity,
    pub is_constructing: bool,
    pub y_plane_position: Vec2Int,
    pub ghost_item: HashMap<Vec3Int, GhostTile>,
    pub group_id: Option<GroupTypeId>,
}
#[derive(Clone)]
pub struct GhostTile {
    /// Id of tile type.
    pub tile_type: CellTypeId,
    pub ghost_entity_option: Option<Entity>,
    pub ghost_rotation: u8,
    pub ghost_face: CellFace,
}

pub struct ShowYLevelPlane {
    pub show: bool,
}
pub(crate) fn show_ylevel_plane(
    mut events: EventReader<ShowYLevelPlane>,
    mut state: ResMut<GridmapConstructionState>,
    mut query: Query<&mut Visibility, With<SelectCellCameraYPlane>>,
    mut events2: EventWriter<SetYPlanePosition>,
) {
    for event in events.iter() {
        match query.get_mut(state.y_plane) {
            Ok(mut visibility) => {
                let visibility2;
                if event.show {
                    visibility2 = Visibility::Inherited;
                } else {
                    visibility2 = Visibility::Hidden;
                }
                *visibility = visibility2;
                state.is_constructing = event.show;
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

pub(crate) fn move_ylevel_plane(
    camera_query: Query<&LookTransform>,
    camera: Res<ActiveCamera>,
    mut state: ResMut<GridmapConstructionState>,
    mut ylevel_query: Query<&mut Transform, With<SelectCellCameraYPlane>>,
) {
    if !state.is_constructing {
        return;
    }
    match camera.option {
        Some(camera_entity) => match camera_query.get(camera_entity) {
            Ok(look_transform) => {
                let camera_cell_id = world_to_cell_id(look_transform.eye);
                if state.y_plane_position.x != camera_cell_id.x
                    && state.y_plane_position.y != camera_cell_id.z
                {
                    match ylevel_query.get_mut(state.y_plane) {
                        Ok(mut transform) => {
                            let new_transform = cell_id_to_world(camera_cell_id);
                            transform.translation.x = new_transform.x + 0.5;
                            transform.translation.z = new_transform.z + 0.5;
                            state.y_plane_position = Vec2Int {
                                x: camera_cell_id.x,
                                y: camera_cell_id.z,
                            }
                        }
                        Err(_) => {
                            warn!("Couldnt find yplane.");
                        }
                    }
                }
            }
            Err(_) => {
                warn!("Couldnt query camera.");
            }
        },
        None => {
            warn!("Couldnt find camera.");
        }
    }
}

pub struct SetYPlanePosition {
    pub y: i16,
}

pub const YPLANE_Y_OFFSET: f32 = 0.1;

pub(crate) fn set_yplane_position(
    mut events: EventReader<SetYPlanePosition>,
    mut state: ResMut<GridmapConstructionState>,
    mut query: Query<&mut Transform, With<SelectCellCameraYPlane>>,
) {
    for event in events.iter() {
        state.y_level = event.y;
        match query.get_mut(state.y_plane) {
            Ok(mut transform) => {
                transform.translation.y = event.y as f32 + YPLANE_Y_OFFSET;
            }
            Err(_) => {
                warn!("Couldnt query plane.");
            }
        }
    }
}

pub const YPLANE_MOVE_UP_BIND: &str = "yplaneMoveUp";
pub const YPLANE_MOVE_DOWN_BIND: &str = "yplaneMoveDown";
pub const ROTATE_CONSTRUCTION_LEFT_BIND: &str = "rotateMapConstructionLeft";
pub const ROTATE_CONSTRUCTION_RIGHT_BIND: &str = "rotateMapConstructionRight";
pub const ROTATE_CONSTRUCTION_UP_BIND: &str = "rotateMapConstructionUp";
pub const ROTATE_CONSTRUCTION_DOWN_BIND: &str = "rotateMapConstructionDown";

pub(crate) fn register_input(mut binds: ResMut<KeyBinds>) {
    binds.list.insert(
        YPLANE_MOVE_UP_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::E,
            description: "Increase y-level of gridmap construction.".to_string(),
            name: "Map Construction add height".to_string(),
        },
    );
    binds.list.insert(
        YPLANE_MOVE_DOWN_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::Q,
            description: "Decrease y-level of gridmap construction.".to_string(),
            name: "Map Construction decrease height".to_string(),
        },
    );
    binds.list.insert(
        ROTATE_CONSTRUCTION_LEFT_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::Left,
            description: "Rotates map construction.".to_string(),
            name: "Map Construction Rotate Left".to_string(),
        },
    );
    binds.list.insert(
        ROTATE_CONSTRUCTION_RIGHT_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::Right,
            description: "Rotates map construction.".to_string(),
            name: "Map Construction Rotate Right".to_string(),
        },
    );
    binds.list.insert(
        ROTATE_CONSTRUCTION_UP_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::Up,
            description: "Rotates map construction.".to_string(),
            name: "Map Construction Rotate Up".to_string(),
        },
    );
    binds.list.insert(
        ROTATE_CONSTRUCTION_DOWN_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::Down,
            description: "Rotates map construction.".to_string(),
            name: "Map Construction Rotate Down".to_string(),
        },
    );
}

pub(crate) fn input_yplane_position(
    keys: Res<Input<KeyCode>>,
    state: Res<GridmapConstructionState>,
    mut events: EventWriter<SetYPlanePosition>,
    focus: Res<TextInput>,
    binds: Res<KeyBinds>,
) {
    if state.is_constructing && focus.focused_input.is_none() {
        if keys.just_pressed(binds.bind(YPLANE_MOVE_DOWN_BIND)) {
            events.send(SetYPlanePosition {
                y: state.y_level - 1,
            });
        }
        if keys.just_pressed(binds.bind(YPLANE_MOVE_UP_BIND)) {
            events.send(SetYPlanePosition {
                y: state.y_level + 1,
            });
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum GhostTileLabel {
    Update,
}
#[derive(Component)]
pub struct GhostTileComponent;

pub(crate) fn update_ghost_cell(
    mut state: ResMut<GridmapConstructionState>,
    gridmap: Res<Gridmap>,
    mut events: EventReader<ConstructionCellSelectionChanged>,
    mut commands: Commands,
) {
    if !state.is_constructing {
        return;
    }

    for _ in events.iter() {
        for (_, tile) in state.ghost_item.iter() {
            match tile.ghost_entity_option {
                Some(ghost_entity) => {
                    commands.entity(ghost_entity).despawn_recursive();
                }
                None => {}
            }
        }

        match state.group_id {
            Some(groupid) => {
                // Commands spawn group cells at correct positions with the right rotations.
                state.ghost_item.clear();

                match gridmap.groups.get(&groupid) {
                    Some(group) => {
                        for (local_id, tile) in group.iter() {
                            match gridmap.main_cell_properties.get(&tile.tile_type) {
                                Some(properties) => {
                                    let ghost_entity = commands
                                        .spawn(GhostTileComponent)
                                        .insert(SceneBundle {
                                            scene: properties.mesh_option.clone().unwrap(),
                                            transform: Transform::default(),
                                            ..Default::default()
                                        })
                                        .id();

                                    let new_tile = GhostTile {
                                        tile_type: tile.tile_type,
                                        ghost_entity_option: Some(ghost_entity),
                                        ghost_rotation: tile.orientation,
                                        ghost_face: tile.face.clone(),
                                    };

                                    state.ghost_item.insert(*local_id, new_tile);
                                }
                                None => {
                                    warn!("Couldnt find tiletype.");
                                    continue;
                                }
                            }
                        }
                    }
                    None => {
                        warn!("Couldnt find group.");
                    }
                }
            }
            None => {
                // Commands spawn single cell.

                let mut ids = vec![];
                for (cell, _) in state.ghost_item.iter() {
                    let i = &Vec3Int { x: 0, y: 0, z: 0 };
                    if cell != i {
                        ids.push(*cell);
                    }
                }
                for c in ids {
                    state.ghost_item.remove(&c);
                }

                match state.ghost_item.get_mut(&Vec3Int { x: 0, y: 0, z: 0 }) {
                    Some(mut g) => match gridmap.main_cell_properties.get(&g.tile_type) {
                        Some(properties) => {
                            let ghost_entity = commands
                                .spawn(GhostTileComponent)
                                .insert(SceneBundle {
                                    scene: properties.mesh_option.clone().unwrap(),
                                    transform: Transform::default(),
                                    ..Default::default()
                                })
                                .id();
                            g.ghost_entity_option = Some(ghost_entity);
                            match properties.cell_type {
                                crate::grid::CellType::Wall => g.ghost_face = CellFace::default(),
                                crate::grid::CellType::Floor => g.ghost_face = CellFace::Floor,
                                crate::grid::CellType::Center => g.ghost_face = CellFace::Center,
                            }
                        }
                        None => {
                            warn!("Couldnt find tiletype.");
                            continue;
                        }
                    },
                    None => {
                        warn!("No local tiletype.");
                    }
                }
            }
        }
    }
}

pub(crate) fn input_ghost_rotation(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<GridmapConstructionState>,
    gridmap: Res<Gridmap>,
    mut ghost_query: Query<&mut Transform, With<GhostTileComponent>>,
    mut events: EventReader<ConstructionCellSelectionChanged>,
    binds: Res<KeyBinds>,
) {
    if !state.is_constructing {
        return;
    }

    let mut changed = false;
    for _ in events.iter() {
        changed = true;
    }

    if keys.just_pressed(binds.bind(ROTATE_CONSTRUCTION_LEFT_BIND)) {
        // x
        changed = true;
    }
    if keys.just_pressed(binds.bind(ROTATE_CONSTRUCTION_RIGHT_BIND)) {
        changed = true;
    }
    if keys.just_pressed(binds.bind(ROTATE_CONSTRUCTION_DOWN_BIND)) {
        changed = true;
    }
    if keys.just_pressed(binds.bind(ROTATE_CONSTRUCTION_UP_BIND)) {
        changed = true;
    }
    if !changed {
        return;
    }

    let state_selected = state.selected.clone();

    for (local_id, tile) in state.ghost_item.iter_mut() {
        let int = &Vec3Int { x: 0, y: 0, z: 0 };

        let mut new_id = local_id.clone();

        if local_id != int {
            let mut point = Transform::from_translation(Vec3::new(
                local_id.x as f32,
                local_id.y as f32,
                local_id.z as f32,
            ));

            point.rotate(OrthogonalBases::default().bases[tile.ghost_rotation as usize]);

            new_id = Vec3Int {
                x: point.translation.x as i16,
                y: point.translation.y as i16,
                z: point.translation.z as i16,
            }
        }

        match tile.ghost_entity_option {
            Some(ghost_entity) => match ghost_query.get_mut(ghost_entity) {
                Ok(mut ghost_transform) => match state_selected {
                    Some(selected_id) => {
                        let mut new_face = tile.ghost_face.clone();

                        let mut new_rotation = tile.ghost_rotation;

                        let full_id = new_id + selected_id;

                        match gridmap.main_cell_properties.get(&tile.tile_type) {
                            Some(properties) => {
                                match properties.cell_type {
                                    crate::grid::CellType::Wall => {
                                        if keys
                                            .just_pressed(binds.bind(ROTATE_CONSTRUCTION_LEFT_BIND))
                                        {
                                            match tile.ghost_face {
                                                CellFace::FrontWall => {
                                                    new_face = CellFace::LeftWall;
                                                }
                                                CellFace::RightWall => {
                                                    new_face = CellFace::FrontWall;
                                                }
                                                CellFace::BackWall => {
                                                    new_face = CellFace::RightWall;
                                                }
                                                CellFace::LeftWall => {
                                                    new_face = CellFace::BackWall;
                                                }
                                                _ => {
                                                    warn!("Invalid wall rotation.");
                                                }
                                            }
                                        } else if keys.just_pressed(
                                            binds.bind(ROTATE_CONSTRUCTION_RIGHT_BIND),
                                        ) {
                                            match tile.ghost_face {
                                                CellFace::FrontWall => {
                                                    new_face = CellFace::RightWall;
                                                }
                                                CellFace::RightWall => {
                                                    new_face = CellFace::BackWall;
                                                }
                                                CellFace::BackWall => {
                                                    new_face = CellFace::LeftWall;
                                                }
                                                CellFace::LeftWall => {
                                                    new_face = CellFace::FrontWall;
                                                }
                                                _ => {
                                                    warn!("Invalid wall rotation.");
                                                }
                                            }
                                        } else if keys
                                            .just_pressed(binds.bind(ROTATE_CONSTRUCTION_DOWN_BIND))
                                        {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::Y, PI);
                                            new_rotation = rotation.get_orthogonal_index();
                                        } else if keys
                                            .just_pressed(binds.bind(ROTATE_CONSTRUCTION_UP_BIND))
                                        {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::Z, PI / 2.0);
                                            new_rotation = rotation.get_orthogonal_index();
                                        }
                                    }
                                    crate::grid::CellType::Floor => {
                                        if keys
                                            .just_pressed(binds.bind(ROTATE_CONSTRUCTION_LEFT_BIND))
                                        {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::Y, PI / 2.);
                                            new_rotation = rotation.get_orthogonal_index();
                                        } else if keys.just_pressed(
                                            binds.bind(ROTATE_CONSTRUCTION_RIGHT_BIND),
                                        ) {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::Y, PI / 2.);
                                            new_rotation = rotation.get_orthogonal_index();
                                        } else if keys
                                            .just_pressed(binds.bind(ROTATE_CONSTRUCTION_DOWN_BIND))
                                        {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::X, PI);
                                            new_rotation = rotation.get_orthogonal_index();
                                        }
                                    }
                                    crate::grid::CellType::Center => {
                                        if keys
                                            .just_pressed(binds.bind(ROTATE_CONSTRUCTION_LEFT_BIND))
                                        {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::X, PI / 2.);
                                            new_rotation = rotation.get_orthogonal_index();
                                        } else if keys.just_pressed(
                                            binds.bind(ROTATE_CONSTRUCTION_RIGHT_BIND),
                                        ) {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::Z, PI / 2.);
                                            new_rotation = rotation.get_orthogonal_index();
                                        } else if keys
                                            .just_pressed(binds.bind(ROTATE_CONSTRUCTION_DOWN_BIND))
                                        {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::Y, PI / 2.);
                                            new_rotation = rotation.get_orthogonal_index();
                                        }
                                    }
                                }

                                *ghost_transform = gridmap.get_cell_transform(
                                    TargetCell {
                                        id: full_id,
                                        face: new_face.clone(),
                                    },
                                    new_rotation,
                                );
                                tile.ghost_face = new_face;
                                tile.ghost_rotation = new_rotation;
                            }
                            None => {}
                        }
                    }
                    None => {
                        warn!("Couldnt find cell properties.");
                    }
                },
                Err(_) => {
                    warn!("Couldnt find ghost transform.");
                }
            },
            None => {}
        }
    }
}

pub struct ConstructionCellSelectionChanged {
    pub changed_tile_type: bool,
}

pub(crate) fn select_cell_in_front_camera(
    camera_query: Query<&LookTransform>,
    active_camera: Res<ActiveCamera>,
    rapier_context: Res<RapierContext>,
    mut state: ResMut<GridmapConstructionState>,
    mut events: EventWriter<ConstructionCellSelectionChanged>,
) {
    if !state.is_constructing {
        return;
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
    let max_toi = 32.0;
    let solid = true;
    let collider_groups = get_bit_masks(ColliderGroup::GridmapSelection);

    let filter = QueryFilter::new().groups(CollisionGroups::new(
        Group::from_bits(collider_groups.0).unwrap(),
        Group::from_bits(collider_groups.1).unwrap(),
    ));

    let intersection_position;

    if let Some((_entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
    {
        intersection_position = ray_pos + ray_dir * toi;
    } else {
        return;
    }
    let world_id = world_to_cell_id(intersection_position);

    let n = Vec3Int {
        x: world_id.x,
        y: state.y_level,
        z: world_id.z,
    };

    match state.selected {
        Some(s) => {
            if s != n {
                events.send(ConstructionCellSelectionChanged {
                    changed_tile_type: false,
                });
            }
        }
        None => {}
    }
    state.selected = Some(n);
}

pub(crate) fn change_ghost_tile_request(
    mut net: EventReader<IncomingReliableServerMessage<GridmapServerMessage>>,
    mut events: EventWriter<ConstructionCellSelectionChanged>,
    mut select_state: ResMut<GridmapConstructionState>,
) {
    for message in net.iter() {
        match &message.message {
            GridmapServerMessage::GhostCellType(type_id) => {
                match type_id {
                    CellIds::CellType(id) => {
                        match select_state
                            .ghost_item
                            .get_mut(&Vec3Int { x: 0, y: 0, z: 0 })
                        {
                            Some(tile) => {
                                tile.tile_type = *id;
                            }
                            None => {
                                select_state.ghost_item.insert(
                                    Vec3Int { x: 0, y: 0, z: 0 },
                                    GhostTile {
                                        tile_type: *id,
                                        ghost_entity_option: None,
                                        ghost_rotation: 0,
                                        ghost_face: CellFace::default(),
                                    },
                                );
                            }
                        }
                        select_state.group_id = None;
                    }
                    CellIds::GroupType(id) => {
                        select_state.group_id = Some(*id);
                    }
                }
                events.send(ConstructionCellSelectionChanged {
                    changed_tile_type: true,
                });
            }
            _ => (),
        }
    }
}

pub(crate) fn client_mouse_click_input(
    buttons: Res<Input<MouseButton>>,
    state: Res<GridmapConstructionState>,
    mut net: EventWriter<OutgoingReliableClientMessage<GridmapClientMessage>>,
    hud_state: Res<HudState>,
) {
    if !state.is_constructing || hud_state.expanded {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        if state.ghost_item.len() == 0 {
            return;
        }

        let cell_id;

        match state.selected {
            Some(c) => {
                cell_id = c;
            }
            None => {
                return;
            }
        }

        let mut construct_cells = vec![];

        for (local_id, tile) in state.ghost_item.iter() {
            construct_cells.push(TargetCellWithOrientationWType {
                id: cell_id + *local_id,
                face: tile.ghost_face.clone(),
                orientation: tile.ghost_rotation,
                tile_type: tile.tile_type,
            });
        }
        info!(
            "construct_cells({}): {:?}",
            construct_cells.len(),
            construct_cells
        );
        net.send(OutgoingReliableClientMessage {
            message: GridmapClientMessage::ConstructCells(ConstructCell {
                cells: construct_cells,
                group_option: Some(cell_id),
            }),
        });
    }
    if buttons.just_pressed(MouseButton::Right) {
        let cell_id;

        match state.selected {
            Some(c) => {
                cell_id = c;
            }
            None => {
                return;
            }
        }

        let mut construct_cells = vec![];

        for (local_id, tile) in state.ghost_item.iter() {
            construct_cells.push(TargetCell {
                id: cell_id + *local_id,
                face: tile.ghost_face.clone(),
            });
        }
        net.send(OutgoingReliableClientMessage {
            message: GridmapClientMessage::DeconstructCells(DeconstructCell {
                cells: construct_cells,
            }),
        });
    }
}
