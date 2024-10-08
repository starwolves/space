use std::{collections::HashMap, f32::consts::PI};

use bevy::ecs::system::Local;
use bevy::log::warn;
use bevy::math::Dir3;
use bevy::prelude::TransformBundle;
use bevy::transform::components::GlobalTransform;
use bevy::{
    gltf::GltfMesh,
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::{
        AlphaMode, AssetServer, Assets, Color, Commands, Component, Entity, Event, EventReader,
        EventWriter, Handle, KeyCode, MouseButton, PbrBundle, Quat, Query, Res, ResMut, Resource,
        StandardMaterial, SystemSet, Transform, Vec3, Visibility, With,
    },
};

use bevy_xpbd_3d::prelude::{
    Collider, CollisionLayers, RigidBody, SpatialQuery, SpatialQueryFilter,
};
use cameras::{controllers::fps::ActiveCamera, LookTransform};
use entity::despawn::DespawnEntity;
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use physics::physics::{get_bit_masks, ColliderGroup};
use resources::pawn::ClientPawn;
use resources::{
    grid::{CellFace, TargetCell},
    hud::HudState,
    input::{InputBuffer, KeyBind, KeyBinds, KeyCodeEnum},
};
use resources::{
    math::{cell_id_to_world, world_to_cell_id, Vec2Int, Vec3Int},
    ui::TextInput,
};

use crate::grid::LayerTargetCell;
use crate::{
    grid::{
        CellIds, CellTypeId, Gridmap, GroupTypeId, Orthogonal, OrthogonalBases,
        TargetCellWithOrientationWType,
    },
    net::{ConstructCell, DeconstructCell, GridmapClientMessage, GridmapServerMessage},
};

#[derive(Component)]
pub struct SelectCellCameraYPlane;

pub fn insert_plane_resource(
    mut commands: Commands,
    mut local: Local<bool>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    plane_res: Res<PlaneAsset>,
) {
    if *local {
        return;
    }
    let m = materials.add(StandardMaterial {
        base_color: Color::srgba(0., 255., 255., 0.5),
        emissive: Color::WHITE.into(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let masks = get_bit_masks(ColliderGroup::GridmapSelection);

    match assets_gltfmesh.get(&plane_res.0) {
        Some(mesh) => {
            let plane_entity = commands
                .spawn(RigidBody::Static)
                .insert(SelectCellCameraYPlane)
                .insert(PbrBundle {
                    mesh: mesh.primitives[0].mesh.clone(),
                    material: m.clone(),
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(0.5, YPLANE_Y_OFFSET, 0.5),
                    ..Default::default()
                })
                .insert(Collider::cuboid(5000., 0.1, 5000.))
                .insert(TransformBundle::from(Transform::IDENTITY))
                .insert(CollisionLayers::from_bits(masks.0, masks.1))
                .insert(NotShadowCaster)
                .insert(NotShadowReceiver)
                .id();
            commands.insert_resource(GridmapConstructionState {
                selected: None,
                y_level: 0,
                y_plane: plane_entity,
                is_constructing: false,
                y_plane_position: Vec2Int { x: 0, y: 0 },
                ghost_items: HashMap::default(),
                group_id: None,
                ghost_material: m,
                rotated_ghost_ids: HashMap::default(),
                first_show: false,
            });
            *local = true;
        }
        None => {
            //warn!("Could not load plane asset: {:?}", plane_asset);
        }
    }
}
#[derive(Resource)]
pub struct PlaneAsset(pub Handle<GltfMesh>);
pub const PLANE_PATH: &str = "gridmap/ylevel_grid_plane/plane.glb#Mesh0";
pub fn load_plane_asset(asset_server: Res<AssetServer>, mut commands: Commands) {
    let plane_asset: Handle<GltfMesh> = asset_server.load(PLANE_PATH);
    commands.insert_resource(PlaneAsset(plane_asset));
}

/// Doesnt exist until its assets are loaded.
#[derive(Resource, Clone)]
pub struct GridmapConstructionState {
    pub selected: Option<Vec3Int>,
    pub y_level: i16,
    pub y_plane: Entity,
    pub is_constructing: bool,
    pub y_plane_position: Vec2Int,
    pub ghost_items: HashMap<Vec3Int, GhostTile>,
    pub rotated_ghost_ids: HashMap<Vec3Int, Vec3Int>,
    pub group_id: Option<GroupTypeId>,
    pub ghost_material: Handle<StandardMaterial>,
    pub first_show: bool,
}
#[derive(Clone, Debug)]
pub struct GhostTile {
    /// Id of tile type.
    pub tile_type: CellTypeId,
    pub ghost_entity_option: Option<Entity>,
    pub ghost_rotation: u8,
    pub ghost_face: CellFace,
    pub is_detail: bool,
}
#[derive(Event)]
pub struct ShowYLevelPlane {
    pub show: bool,
}
pub(crate) fn show_ylevel_plane(
    mut events: EventReader<ShowYLevelPlane>,
    mut state: ResMut<GridmapConstructionState>,
    mut query: Query<&mut Visibility, With<SelectCellCameraYPlane>>,
    mut events2: EventWriter<SetYPlanePosition>,
) {
    for event in events.read() {
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
                state.first_show = true;
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
    camera_query: Query<Entity, With<ClientPawn>>,
    mut state: ResMut<GridmapConstructionState>,
    ylevel_query: Query<Entity, With<SelectCellCameraYPlane>>,
    mut transforms: Query<&mut Transform>,
) {
    if !state.is_constructing {
        return;
    }
    match camera_query.get_single() {
        Ok(ent) => {
            let transform = transforms.get(ent).unwrap();
            let camera_cell_id = world_to_cell_id(transform.translation);
            if state.y_plane_position.x != camera_cell_id.x
                || state.y_plane_position.y != camera_cell_id.z
                || state.first_show
            {
                state.first_show = false;
                match ylevel_query.get(state.y_plane) {
                    Ok(entity) => {
                        let mut transform = transforms.get_mut(entity).unwrap();
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
    }
}
#[derive(Event)]
pub struct SetYPlanePosition {
    pub y: i16,
}

pub const YPLANE_Y_OFFSET: f32 = 0.1;

pub(crate) fn set_yplane_position(
    mut events: EventReader<SetYPlanePosition>,
    mut state: ResMut<GridmapConstructionState>,
    mut query: Query<&mut Transform, With<SelectCellCameraYPlane>>,
) {
    for event in events.read() {
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
            key_code: KeyCodeEnum::Keyboard(KeyCode::KeyE),
            description: "Increase y-level of gridmap construction.".to_string(),
            name: "Map Construction add height".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        YPLANE_MOVE_DOWN_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::KeyQ),
            description: "Decrease y-level of gridmap construction.".to_string(),
            name: "Map Construction decrease height".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        ROTATE_CONSTRUCTION_LEFT_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::ArrowLeft),
            description: "Rotates map construction.".to_string(),
            name: "Map Construction Rotate Left".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        ROTATE_CONSTRUCTION_RIGHT_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::ArrowRight),
            description: "Rotates map construction.".to_string(),
            name: "Map Construction Rotate Right".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        ROTATE_CONSTRUCTION_UP_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::ArrowUp),
            description: "Rotates map construction.".to_string(),
            name: "Map Construction Rotate Up".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        ROTATE_CONSTRUCTION_DOWN_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::ArrowDown),
            description: "Rotates map construction.".to_string(),
            name: "Map Construction Rotate Down".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        CONSTRUCT_CELL.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Mouse(MouseButton::Left),
            description: "Use button to constuct gridmap cell.".to_string(),
            name: "Construct cell.".to_string(),
            customizable: false,
        },
    );
    binds.list.insert(
        DECONSTRUCT_CELL.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Mouse(MouseButton::Right),
            description: "Use button to deconstuct gridmap cell".to_string(),
            name: "Deconstruct cell.".to_string(),
            customizable: false,
        },
    );
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum YPlaneSet {
    Input,
    Show,
    Position,
}

pub(crate) fn input_yplane_position(
    keys: Res<InputBuffer>,
    state: Res<GridmapConstructionState>,
    mut events: EventWriter<SetYPlanePosition>,
    focus: Res<TextInput>,
) {
    if state.is_constructing && focus.focused_input.is_none() {
        if keys.just_pressed(YPLANE_MOVE_DOWN_BIND) {
            events.send(SetYPlanePosition {
                y: state.y_level - 1,
            });
        }
        if keys.just_pressed(YPLANE_MOVE_UP_BIND) {
            events.send(SetYPlanePosition {
                y: state.y_level + 1,
            });
        }
    }
}
#[derive(Resource, Default)]
pub(crate) struct NewGhostBuffer {
    pub buffer: Vec<ConstructionCellSelectionChanged>,
}

#[derive(Component)]
pub struct GhostTileComponent;

pub(crate) fn update_ghost_cell(
    mut state: ResMut<GridmapConstructionState>,
    gridmap: Res<Gridmap>,
    mut events: EventReader<ConstructionCellSelectionChanged>,
    mut commands: Commands,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    mut buffer: ResMut<NewGhostBuffer>,
    mut despawn: EventWriter<DespawnEntity>,
) {
    if !state.is_constructing {
        return;
    }
    for changed in events.read() {
        for (_, tile) in state.ghost_items.iter() {
            match tile.ghost_entity_option {
                Some(ghost_entity) => {
                    despawn.send(DespawnEntity {
                        entity: ghost_entity,
                    });
                }
                None => {}
            }
        }

        let full_id;
        match state.selected {
            Some(id) => {
                full_id = id;
            }
            None => {
                //warn!("None cell id selected.");
                continue;
            }
        }
        match state.group_id {
            Some(groupid) => {
                // Commands spawn group cells at correct positions with the right rotations.

                if !changed.only_selection_changed {
                    state.ghost_items.clear();
                    state.rotated_ghost_ids.clear();
                }
                match gridmap.groups.get(&groupid) {
                    Some(group) => {
                        for (local_id, tile) in group.iter() {
                            match gridmap.tile_properties.get(&tile.tile_type) {
                                Some(properties) => {
                                    let prev_item;

                                    let f;
                                    if !changed.only_selection_changed {
                                        f = properties.cell_type.default_face();
                                    } else {
                                        f = tile.face.clone();
                                    }

                                    match state.ghost_items.get(local_id) {
                                        Some(i) => {
                                            prev_item = i.clone();
                                        }
                                        None => {
                                            prev_item = GhostTile {
                                                tile_type: tile.tile_type,
                                                ghost_entity_option: None,
                                                ghost_rotation: tile.orientation,
                                                ghost_face: f.clone(),
                                                is_detail: properties.is_detail,
                                            };
                                        }
                                    }
                                    let mut t = gridmap.get_cell_transform(
                                        TargetCell {
                                            id: full_id + *local_id,
                                            face: f,
                                        },
                                        tile.orientation,
                                    );

                                    t.scale = Vec3::from([1.05; 3]);

                                    match assets_gltfmesh
                                        .get(&properties.mesh_option.clone().unwrap())
                                    {
                                        Some(gltf) => {
                                            let ghost_entity = commands
                                                .spawn(GhostTileComponent)
                                                .insert(PbrBundle {
                                                    mesh: gltf.primitives[0].mesh.clone(),
                                                    material: state.ghost_material.clone(),
                                                    transform: t,
                                                    ..Default::default()
                                                })
                                                .insert(NotShadowCaster)
                                                .insert(NotShadowReceiver)
                                                .id();
                                            let new_tile = GhostTile {
                                                tile_type: tile.tile_type,
                                                ghost_entity_option: Some(ghost_entity),
                                                ghost_rotation: prev_item.ghost_rotation,
                                                ghost_face: prev_item.ghost_face.clone(),
                                                is_detail: properties.is_detail,
                                            };

                                            state.ghost_items.insert(*local_id, new_tile);
                                        }
                                        None => {
                                            warn!("Couldnt find ghost material asset gltf.");
                                        }
                                    }
                                }
                                None => {
                                    warn!("Couldnt find tiletype.");
                                    continue;
                                }
                            }
                        }
                    }
                    None => {
                        warn!("Couldnt find group. (1): {:?}", groupid);
                    }
                }
            }
            None => {
                // Commands spawn single cell.

                let mut ids = vec![];
                for (cell, _) in state.ghost_items.iter() {
                    let i = &Vec3Int { x: 0, y: 0, z: 0 };
                    if cell != i {
                        ids.push(*cell);
                    }
                }
                for c in ids {
                    state.ghost_items.remove(&c);
                }
                let m = state.ghost_material.clone();
                match state.ghost_items.get_mut(&Vec3Int { x: 0, y: 0, z: 0 }) {
                    Some(g) => match gridmap.tile_properties.get(&g.tile_type) {
                        Some(properties) => {
                            if !changed.only_selection_changed {
                                g.ghost_face = properties.cell_type.default_face();
                            }
                            let mut t = gridmap.get_cell_transform(
                                TargetCell {
                                    id: full_id,
                                    face: g.ghost_face.clone(),
                                },
                                g.ghost_rotation,
                            );

                            t.scale = Vec3::from([1.05; 3]);

                            match assets_gltfmesh.get(&properties.mesh_option.clone().unwrap()) {
                                Some(mesh) => {
                                    let ghost_entity = commands
                                        .spawn(GhostTileComponent)
                                        .insert(PbrBundle {
                                            mesh: mesh.primitives[0].mesh.clone(),
                                            material: m,
                                            transform: t,
                                            ..Default::default()
                                        })
                                        .insert(NotShadowCaster)
                                        .insert(NotShadowReceiver)
                                        .id();
                                    g.ghost_entity_option = Some(ghost_entity);
                                }
                                None => {
                                    warn!("gltf mesh not found.");
                                }
                            }
                        }
                        None => {
                            warn!("Couldnt find tiletype.");
                            continue;
                        }
                    },
                    None => {
                        //warn!("No local tiletype.");
                    }
                }
            }
        }

        buffer.buffer.push(changed.clone());
    }
}

pub(crate) fn apply_ghost_rotation(
    keys: Res<InputBuffer>,
    mut state: ResMut<GridmapConstructionState>,
    gridmap: Res<Gridmap>,
    mut ghost_query: Query<&mut Transform, With<GhostTileComponent>>,
    mut events: ResMut<NewGhostBuffer>,
) {
    if !state.is_constructing {
        return;
    }

    let mut changed = false;
    for _ in events.buffer.iter() {
        changed = true;
    }
    events.buffer.clear();
    let mut rot_changed = false;

    if keys.just_pressed(ROTATE_CONSTRUCTION_LEFT_BIND) {
        // x
        changed = true;
        rot_changed = true;
    }
    if keys.just_pressed(ROTATE_CONSTRUCTION_RIGHT_BIND) {
        changed = true;
        rot_changed = true;
    }
    if keys.just_pressed(ROTATE_CONSTRUCTION_DOWN_BIND) {
        changed = true;
        rot_changed = true;
    }
    if keys.just_pressed(ROTATE_CONSTRUCTION_UP_BIND) {
        changed = true;
        rot_changed = true;
    }
    if !changed {
        return;
    }

    let state_selected = state.selected.clone();

    let mut new_ids: HashMap<Vec3Int, Vec3Int> = HashMap::default();

    for (local_id, tile) in state.ghost_items.iter_mut() {
        let int = &Vec3Int { x: 0, y: 0, z: 0 };

        let mut new_id = local_id.clone();

        match tile.ghost_entity_option {
            Some(ghost_entity) => match ghost_query.get_mut(ghost_entity) {
                Ok(mut ghost_transform) => match state_selected {
                    Some(selected_id) => {
                        let mut new_face;
                        let mut new_rotation = tile.ghost_rotation;

                        match gridmap.tile_properties.get(&tile.tile_type) {
                            Some(properties) => {
                                new_face = tile.ghost_face.clone();
                                match properties.cell_type {
                                    crate::grid::CellType::Wall => {
                                        if keys.just_pressed(ROTATE_CONSTRUCTION_LEFT_BIND) {
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
                                        } else if keys.just_pressed(ROTATE_CONSTRUCTION_RIGHT_BIND)
                                        {
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
                                        } else if keys.just_pressed(ROTATE_CONSTRUCTION_DOWN_BIND) {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::Y, PI);
                                            new_rotation = rotation.get_orthogonal_index();
                                        } else if keys.just_pressed(ROTATE_CONSTRUCTION_UP_BIND) {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::Z, PI / 2.0);
                                            new_rotation = rotation.get_orthogonal_index();
                                        }
                                    }
                                    crate::grid::CellType::Floor => {
                                        let rotations = vec![0, 22, 10, 16];
                                        let mut rotation_i = 0;
                                        for rot in rotations.iter() {
                                            if rot == &tile.ghost_rotation {
                                                break;
                                            }
                                            rotation_i += 1;
                                        }
                                        if rotation_i > rotations.len() - 1 {
                                            rotation_i = 0;
                                        }
                                        let rotation_i_increased;
                                        if rotation_i == rotations.len() - 1 {
                                            rotation_i_increased = 0;
                                        } else {
                                            rotation_i_increased = rotation_i + 1;
                                        }
                                        let rotation_i_decreased;
                                        if rotation_i == 0 {
                                            rotation_i_decreased = rotations.len() - 1;
                                        } else {
                                            rotation_i_decreased = rotation_i - 1;
                                        }

                                        if keys.just_pressed(ROTATE_CONSTRUCTION_LEFT_BIND) {
                                            new_rotation = rotations[rotation_i_increased];
                                        } else if keys.just_pressed(ROTATE_CONSTRUCTION_RIGHT_BIND)
                                        {
                                            new_rotation = rotations[rotation_i_decreased];
                                        } else if keys.just_pressed(ROTATE_CONSTRUCTION_DOWN_BIND) {
                                            let mut rotation = OrthogonalBases::default().bases
                                                [tile.ghost_rotation as usize];
                                            rotation *= Quat::from_axis_angle(Vec3::X, PI);
                                            new_rotation = rotation.get_orthogonal_index();
                                        }
                                    }
                                    crate::grid::CellType::Center => {
                                        if properties.x_rotations.len() > 0 {
                                            let mut rot_i = 0;
                                            for rot in properties.x_rotations.iter() {
                                                if tile.ghost_rotation == *rot {
                                                    break;
                                                }
                                                rot_i += 1;
                                            }
                                            if rot_i > properties.x_rotations.len() - 1 {
                                                rot_i = 0;
                                            }
                                            if keys.just_pressed(ROTATE_CONSTRUCTION_RIGHT_BIND) {
                                                rot_i += 1;

                                                if rot_i > properties.x_rotations.len() - 1 {
                                                    rot_i = 0;
                                                }
                                            } else if keys
                                                .just_pressed(ROTATE_CONSTRUCTION_LEFT_BIND)
                                            {
                                                if rot_i == 0 {
                                                    rot_i = properties.x_rotations.len() - 1;
                                                } else {
                                                    rot_i -= 1;
                                                }
                                            }

                                            let new_rot = properties.x_rotations[rot_i];

                                            let rotation =
                                                OrthogonalBases::default().bases[new_rot as usize];
                                            new_rotation = rotation.get_orthogonal_index();
                                        } else {
                                            if keys.just_pressed(ROTATE_CONSTRUCTION_LEFT_BIND) {
                                                let mut rotation = OrthogonalBases::default().bases
                                                    [tile.ghost_rotation as usize];
                                                rotation *= Quat::from_axis_angle(Vec3::X, PI / 2.);
                                                new_rotation = rotation.get_orthogonal_index();
                                            } else if keys
                                                .just_pressed(ROTATE_CONSTRUCTION_RIGHT_BIND)
                                            {
                                                let mut rotation = OrthogonalBases::default().bases
                                                    [tile.ghost_rotation as usize];
                                                rotation *= Quat::from_axis_angle(Vec3::Z, PI / 2.);
                                                new_rotation = rotation.get_orthogonal_index();
                                            }
                                        }

                                        if properties.vertical_rotation {
                                            if properties.y_rotations.len() > 0 {
                                                let mut rot_i = 0;
                                                for rot in properties.y_rotations.iter() {
                                                    if tile.ghost_rotation == *rot {
                                                        break;
                                                    }
                                                    rot_i += 1;
                                                }
                                                if rot_i > properties.y_rotations.len() - 1 {
                                                    rot_i = 0;
                                                }
                                                if keys.just_pressed(ROTATE_CONSTRUCTION_DOWN_BIND)
                                                {
                                                    rot_i += 1;

                                                    if rot_i > properties.y_rotations.len() - 1 {
                                                        rot_i = 0;
                                                    }
                                                } else if keys
                                                    .just_pressed(ROTATE_CONSTRUCTION_UP_BIND)
                                                {
                                                    if rot_i == 0 {
                                                        rot_i = properties.y_rotations.len() - 1;
                                                    } else {
                                                        rot_i -= 1;
                                                    }
                                                }

                                                let new_rot = properties.y_rotations[rot_i];

                                                let rotation = OrthogonalBases::default().bases
                                                    [new_rot as usize];
                                                new_rotation = rotation.get_orthogonal_index();
                                            } else {
                                                if keys.just_pressed(ROTATE_CONSTRUCTION_DOWN_BIND)
                                                {
                                                    let mut rotation = OrthogonalBases::default()
                                                        .bases
                                                        [tile.ghost_rotation as usize];
                                                    rotation *=
                                                        Quat::from_axis_angle(Vec3::Y, PI / 2.);
                                                    new_rotation = rotation.get_orthogonal_index();
                                                }
                                            }
                                        }
                                    }
                                    crate::grid::CellType::WallDetail => {
                                        if properties.x_rotations.len() > 0 {
                                            let mut rot_i = 0;
                                            for rot in properties.x_rotations.iter() {
                                                if tile.ghost_rotation == *rot {
                                                    break;
                                                }
                                                rot_i += 1;
                                            }
                                            if rot_i > properties.x_rotations.len() - 1 {
                                                rot_i = 0;
                                            }
                                            if keys.just_pressed(ROTATE_CONSTRUCTION_RIGHT_BIND) {
                                                rot_i += 1;

                                                if rot_i > properties.x_rotations.len() - 1 {
                                                    rot_i = 0;
                                                }
                                            } else if keys
                                                .just_pressed(ROTATE_CONSTRUCTION_LEFT_BIND)
                                            {
                                                if rot_i == 0 {
                                                    rot_i = properties.x_rotations.len() - 1;
                                                } else {
                                                    rot_i -= 1;
                                                }
                                            }

                                            let new_rot = properties.x_rotations[rot_i];

                                            let rotation =
                                                OrthogonalBases::default().bases[new_rot as usize];
                                            new_rotation = rotation.get_orthogonal_index();
                                        }

                                        if keys.just_pressed(ROTATE_CONSTRUCTION_UP_BIND) {
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
                                        } else if keys.just_pressed(ROTATE_CONSTRUCTION_DOWN_BIND) {
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
                                        }
                                    }
                                }

                                if local_id != int {
                                    let mut point = Vec3::new(
                                        local_id.x as f32,
                                        local_id.y as f32,
                                        local_id.z as f32,
                                    );
                                    let relative_rotation;
                                    let properties;
                                    match gridmap.tile_properties.get(&tile.tile_type) {
                                        Some(p) => {
                                            properties = p;
                                        }
                                        None => {
                                            warn!("Couldnt find tiletype. {:?}", tile.tile_type);
                                            continue;
                                        }
                                    }

                                    if !properties.vertical_rotation {
                                        let x_rotations;

                                        if properties.x_rotations.len() > 0 {
                                            x_rotations = properties.x_rotations.clone();
                                            relative_rotation = x_rotations
                                                .iter()
                                                .position(|&r| r == new_rotation)
                                                .unwrap();
                                        } else {
                                            let rotations = vec![0, 22, 10, 16];
                                            relative_rotation = rotations
                                                .iter()
                                                .position(|&r| r == new_rotation)
                                                .unwrap();
                                            //relative_rotation = new_rotation as usize;
                                        }
                                    } else {
                                        let y_rotations;

                                        if properties.y_rotations.len() > 0 {
                                            y_rotations = properties.y_rotations.clone();

                                            relative_rotation = y_rotations
                                                .iter()
                                                .position(|&r| r == new_rotation)
                                                .unwrap();
                                        } else {
                                            let rotations = vec![0, 22, 10, 16];
                                            relative_rotation = rotations
                                                .iter()
                                                .position(|&r| r == new_rotation)
                                                .unwrap();
                                            //relative_rotation = new_rotation as usize;
                                        }
                                    }

                                    let mut quat;
                                    if !properties.vertical_rotation {
                                        match relative_rotation {
                                            0 => quat = Quat::from_rotation_y(0. * PI),
                                            1 => quat = Quat::from_rotation_y(1.5 * PI),
                                            2 => quat = Quat::from_rotation_y(1. * PI),
                                            3 => quat = Quat::from_rotation_y(0.5 * PI),
                                            _ => {
                                                warn!(
                                                    "Relative rotation is not supported {}.",
                                                    relative_rotation
                                                );
                                                continue;
                                            }
                                        }
                                    } else {
                                        match relative_rotation {
                                            0 => quat = Quat::from_rotation_z(0. * PI),
                                            1 => quat = Quat::from_rotation_z(1.5 * PI),
                                            2 => quat = Quat::from_rotation_z(1. * PI),
                                            3 => quat = Quat::from_rotation_z(0.5 * PI),
                                            _ => {
                                                warn!("Relative vrotation is not supported.");
                                                continue;
                                            }
                                        }
                                    }
                                    quat = quat.normalize();
                                    point = quat.mul_vec3(point);

                                    new_id = Vec3Int {
                                        x: point.x.round() as i16,
                                        y: point.y.round() as i16,
                                        z: point.z.round() as i16,
                                    };

                                    new_ids.insert(*local_id, new_id);
                                }
                                let full_id = new_id + selected_id;

                                ghost_transform.scale = Vec3::from([1.05; 3]);
                                if rot_changed {
                                    tile.ghost_face = new_face;
                                }

                                tile.ghost_rotation = new_rotation;

                                *ghost_transform = gridmap.get_cell_transform(
                                    TargetCell {
                                        id: full_id,
                                        face: tile.ghost_face.clone(),
                                    },
                                    tile.ghost_rotation,
                                );
                            }
                            None => {}
                        }
                    }
                    None => {
                        warn!("Couldnt find cell properties.");
                    }
                },
                Err(_) => {
                    warn!("Couldnt find ghost components: {:?}.", ghost_entity);
                }
            },
            None => {}
        }
    }

    if new_ids.len() > 0 {
        for (old_id, new_id) in new_ids {
            state.rotated_ghost_ids.insert(old_id, new_id);
        }
    }
}
#[derive(Event, Clone)]
pub struct ConstructionCellSelectionChanged {
    pub only_selection_changed: bool,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum ConstructionSelection {
    Changed,
}
pub(crate) fn select_cell_in_front_camera(
    camera_query: Query<(&GlobalTransform, &LookTransform)>,
    active_camera: Res<ActiveCamera>,
    spatial_query: SpatialQuery,
    mut state: ResMut<GridmapConstructionState>,
    mut events: EventWriter<ConstructionCellSelectionChanged>,
) {
    if !state.is_constructing {
        return;
    }
    let camera_look_transform;
    let camera_transform;
    match active_camera.option {
        Some(camera_entity) => match camera_query.get(camera_entity) {
            Ok((transform, look_transform)) => {
                camera_look_transform = look_transform;
                camera_transform = transform;
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

    let ray_pos = camera_transform.translation();
    let max_toi = 32.0;
    let solid = true;
    let collider_groups = get_bit_masks(ColliderGroup::GridmapSelection);

    let filter = SpatialQueryFilter::from_mask(collider_groups.1);

    let intersection_position;
    let dir3d;
    match Dir3::new(ray_dir) {
        Ok(dir) => {
            dir3d = dir;
        }
        Err(rr) => {
            warn!("Invalid direction. {}", rr);
            return;
        }
    }

    if let Some(hit) = spatial_query.cast_ray(ray_pos, dir3d, max_toi, solid, filter) {
        intersection_position = ray_pos + ray_dir * hit.time_of_impact;
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
                    only_selection_changed: true,
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
    gridmap: Res<Gridmap>,
) {
    for message in net.read() {
        match &message.message {
            GridmapServerMessage::GhostCellType(type_id) => {
                match type_id {
                    CellIds::CellType(id) => {
                        match select_state
                            .ghost_items
                            .get_mut(&Vec3Int { x: 0, y: 0, z: 0 })
                        {
                            Some(tile) => {
                                tile.tile_type = *id;
                            }
                            None => {
                                let is_detail;
                                match gridmap.tile_properties.get(id) {
                                    Some(properties) => {
                                        is_detail = properties.is_detail;
                                    }
                                    None => {
                                        warn!("Couldnt find tile properties.");
                                        continue;
                                    }
                                }
                                select_state.ghost_items.insert(
                                    Vec3Int { x: 0, y: 0, z: 0 },
                                    GhostTile {
                                        tile_type: *id,
                                        ghost_entity_option: None,
                                        ghost_rotation: 0,
                                        ghost_face: CellFace::default(),
                                        is_detail: is_detail,
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
                    only_selection_changed: false,
                });
            }
            _ => (),
        }
    }
}

pub const CONSTRUCT_CELL: &str = "ConstructCell";
pub const DECONSTRUCT_CELL: &str = "DeconstructCell";

pub(crate) fn client_mouse_click_input(
    buttons: Res<InputBuffer>,
    state: Res<GridmapConstructionState>,
    mut net: EventWriter<OutgoingReliableClientMessage<GridmapClientMessage>>,
    hud_state: Res<HudState>,
) {
    if !state.is_constructing || hud_state.expanded {
        return;
    }

    if buttons.just_pressed(CONSTRUCT_CELL) {
        if state.ghost_items.len() == 0 {
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

        //let mut block_construction = false;

        for (local_id, tile) in state.ghost_items.iter() {
            let used_id;

            match state.rotated_ghost_ids.get(local_id) {
                Some(new_id) => used_id = new_id,
                None => used_id = local_id,
            }

            let target = TargetCellWithOrientationWType {
                id: cell_id + *used_id,
                face: tile.ghost_face.clone(),
                orientation: tile.ghost_rotation,
                tile_type: tile.tile_type,
            };

            /*match gridmap
                .get_cell(TargetCell {
                    id: target.id,
                    face: target.face.clone(),
                })
                .is_some()
            {
                block_construction = true;
            }*/

            construct_cells.push(target);
        }
        //if !block_construction {
        net.send(OutgoingReliableClientMessage {
            message: GridmapClientMessage::ConstructCells(ConstructCell {
                cells: construct_cells,
                group_option: Some(cell_id),
            }),
        });
        //}
    }
    if buttons.just_pressed(DECONSTRUCT_CELL) {
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

        for (local_id, tile) in state.ghost_items.iter() {
            let rotated_id;
            if local_id == &(Vec3Int { x: 0, y: 0, z: 0 }) {
                rotated_id = local_id.clone();
            } else {
                match state.rotated_ghost_ids.get(local_id) {
                    Some(rot) => {
                        rotated_id = rot.clone();
                    }
                    None => {
                        warn!("no rotated ghost id found.");
                        continue;
                    }
                }
            }
            construct_cells.push(LayerTargetCell {
                target: TargetCell {
                    id: cell_id + rotated_id,
                    face: tile.ghost_face.clone(),
                },
                is_detail: tile.is_detail,
            });
        }

        net.send(OutgoingReliableClientMessage {
            message: GridmapClientMessage::DeconstructCells(DeconstructCell {
                cells: construct_cells,
            }),
        });
    }
}
