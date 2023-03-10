use std::f32::consts::PI;

use bevy::{
    prelude::{
        warn, AssetServer, BuildChildren, Commands, Component, Entity, EventReader, EventWriter,
        Handle, Input, KeyCode, MouseButton, Quat, Query, Res, ResMut, Resource, SystemSet,
        Transform, Vec3, Visibility, With,
    },
    scene::{Scene, SceneBundle},
    transform::TransformBundle,
};
use bevy_rapier3d::prelude::{
    Collider, CollisionGroups, Group, QueryFilter, RapierContext, RigidBody,
};
use cameras::{controllers::fps::ActiveCamera, LookTransform};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use physics::physics::{get_bit_masks, ColliderGroup};
use resources::{
    grid::{CellFace, TargetCell},
    hud::HudState,
};
use resources::{
    math::{cell_id_to_world, world_to_cell_id, Vec2Int, Vec3Int},
    ui::TextInput,
};

use crate::{
    grid::{Gridmap, Orthogonal, OrthogonalBases},
    net::{ConstructCell, DeconstructCell, GridmapClientMessage, GridmapServerMessage},
};

#[derive(Component)]
pub struct SelectCellCameraYPlane;

pub fn create_select_cell_cam_state(mut commands: Commands, asset_server: Res<AssetServer>) {
    let plane_asset = asset_server.load("models/ylevel_grid_plane/plane.glb#Scene0");
    let empty_asset = asset_server.load("models/empty/empty.glb#Scene0");

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

    let ghost_entity = commands
        .spawn(GhostTileComponent)
        .insert(SceneBundle {
            scene: empty_asset,
            ..Default::default()
        })
        .id();

    commands.insert_resource(GridmapConstructionState {
        selected: None,
        y_level: 0,
        y_plane: plane_entity,
        is_constructing: false,
        y_plane_position: Vec2Int { x: 0, y: 0 },
        ghost_tile: None,
        ghost_entity,
        ghost_rotation: 0,
        ghost_face: CellFace::default(),
    });
}

#[derive(Resource)]
pub struct GridmapConstructionState {
    pub selected: Option<Vec3Int>,
    pub y_level: i16,
    pub y_plane: Entity,
    pub is_constructing: bool,
    pub y_plane_position: Vec2Int,
    pub ghost_tile: Option<GhostTile>,
    pub ghost_entity: Entity,
    pub ghost_rotation: u8,
    pub ghost_face: CellFace,
}

pub struct GhostTile {
    /// Id of tile type.
    pub tile_type: u16,
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

pub(crate) fn input_yplane_position(
    keys: Res<Input<KeyCode>>,
    state: Res<GridmapConstructionState>,
    mut events: EventWriter<SetYPlanePosition>,
    focus: Res<TextInput>,
) {
    if state.is_constructing && focus.focused_input.is_none() {
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
}
pub(crate) fn input_ghost_rotation(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<GridmapConstructionState>,
    gridmap: Res<Gridmap>,
    mut ghost_query: Query<&mut Transform, With<GhostTileComponent>>,
    mut events: EventReader<ConstructionCellSelectionChanged>,
) {
    if !state.is_constructing {
        return;
    }

    let mut changed = false;
    for _ in events.iter() {
        changed = true;
    }

    if keys.just_pressed(KeyCode::Left) {
        // x
        changed = true;
    }
    if keys.just_pressed(KeyCode::Right) {
        changed = true;
    }
    if keys.just_pressed(KeyCode::Down) {
        changed = true;
    }
    if keys.just_pressed(KeyCode::Up) {
        changed = true;
    }
    if !changed {
        return;
    }

    match &state.ghost_tile {
        Some(ghost_tile) => match ghost_query.get_mut(state.ghost_entity) {
            Ok(mut ghost_transform) => match state.selected {
                Some(selected_id) => {
                    match gridmap.main_cell_properties.get(&ghost_tile.tile_type) {
                        Some(properties) => {
                            let mut new_face = state.ghost_face.clone();
                            let mut new_rotation = state.ghost_rotation;
                            match properties.cell_type {
                                crate::grid::CellType::Wall => {
                                    if keys.just_pressed(KeyCode::Left) {
                                        match state.ghost_face {
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
                                    } else if keys.just_pressed(KeyCode::Right) {
                                        match state.ghost_face {
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
                                    } else if keys.just_pressed(KeyCode::Down) {
                                        let mut rotation = OrthogonalBases::default().bases
                                            [state.ghost_rotation as usize];
                                        rotation *= Quat::from_axis_angle(Vec3::Y, PI);
                                        new_rotation = rotation.get_orthogonal_index();
                                    } else if keys.just_pressed(KeyCode::Up) {
                                        let mut rotation = OrthogonalBases::default().bases
                                            [state.ghost_rotation as usize];
                                        rotation *= Quat::from_axis_angle(Vec3::Z, PI / 2.0);
                                        new_rotation = rotation.get_orthogonal_index();
                                    }
                                }
                                crate::grid::CellType::Floor => {
                                    if keys.just_pressed(KeyCode::Left) {
                                        let mut rotation = OrthogonalBases::default().bases
                                            [state.ghost_rotation as usize];
                                        rotation *= Quat::from_axis_angle(Vec3::Y, PI / 2.);
                                        new_rotation = rotation.get_orthogonal_index();
                                    } else if keys.just_pressed(KeyCode::Right) {
                                        let mut rotation = OrthogonalBases::default().bases
                                            [state.ghost_rotation as usize];
                                        rotation *= Quat::from_axis_angle(Vec3::Y, PI / 2.);
                                        new_rotation = rotation.get_orthogonal_index();
                                    } else if keys.just_pressed(KeyCode::Down) {
                                        let mut rotation = OrthogonalBases::default().bases
                                            [state.ghost_rotation as usize];
                                        rotation *= Quat::from_axis_angle(Vec3::X, PI);
                                        new_rotation = rotation.get_orthogonal_index();
                                    }
                                }
                                crate::grid::CellType::Center => {
                                    if keys.just_pressed(KeyCode::Left) {
                                        let mut rotation = OrthogonalBases::default().bases
                                            [state.ghost_rotation as usize];
                                        rotation *= Quat::from_axis_angle(Vec3::X, PI / 2.);
                                        new_rotation = rotation.get_orthogonal_index();
                                    } else if keys.just_pressed(KeyCode::Right) {
                                        let mut rotation = OrthogonalBases::default().bases
                                            [state.ghost_rotation as usize];
                                        rotation *= Quat::from_axis_angle(Vec3::Z, PI / 2.);
                                        new_rotation = rotation.get_orthogonal_index();
                                    } else if keys.just_pressed(KeyCode::Down) {
                                        let mut rotation = OrthogonalBases::default().bases
                                            [state.ghost_rotation as usize];
                                        rotation *= Quat::from_axis_angle(Vec3::Y, PI / 2.);
                                        new_rotation = rotation.get_orthogonal_index();
                                    }
                                }
                            }

                            *ghost_transform = gridmap.get_cell_transform(
                                TargetCell {
                                    id: selected_id,
                                    face: new_face.clone(),
                                },
                                new_rotation,
                            );
                            state.ghost_face = new_face;
                            state.ghost_rotation = new_rotation;
                        }
                        None => {
                            warn!("Couldnt find cell properties.");
                        }
                    }
                }
                None => {}
            },
            Err(_) => {
                warn!("Couldnt find ghost transform.");
            }
        },
        None => {}
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

    state.selected = Some(Vec3Int {
        x: world_id.x,
        y: state.y_level,
        z: world_id.z,
    });
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum GhostTileLabel {
    Update,
}
#[derive(Component)]
pub struct GhostTileComponent;

pub(crate) fn update_ghost_cell(
    mut select_state: ResMut<GridmapConstructionState>,
    gridmap: Res<Gridmap>,
    mut events: EventReader<ConstructionCellSelectionChanged>,
    mut ghost_tile: Query<(&mut Transform, &mut Handle<Scene>), With<GhostTileComponent>>,
) {
    if !select_state.is_constructing {
        return;
    }

    for event in events.iter() {
        match select_state.selected {
            Some(selected_id) => match ghost_tile.get_mut(select_state.ghost_entity) {
                Ok((mut transform, mut scene)) => match &select_state.ghost_tile {
                    Some(ghost) => match gridmap.main_cell_properties.get(&ghost.tile_type) {
                        Some(properties) => {
                            *scene = properties.mesh_option.clone().unwrap();
                            let face;
                            if event.changed_tile_type {
                                select_state.ghost_rotation = 0;
                                match properties.cell_type {
                                    crate::grid::CellType::Wall => {
                                        face = CellFace::FrontWall;
                                    }
                                    crate::grid::CellType::Floor => {
                                        face = CellFace::Floor;
                                    }
                                    crate::grid::CellType::Center => {
                                        face = CellFace::Center;
                                    }
                                }
                                select_state.ghost_face = face.clone();
                            } else {
                                face = select_state.ghost_face.clone();
                            }
                            *transform = gridmap.get_cell_transform(
                                TargetCell {
                                    id: selected_id,
                                    face: face,
                                },
                                select_state.ghost_rotation,
                            );
                        }
                        None => {
                            warn!("Coudlnt find tile.");
                        }
                    },
                    None => {}
                },
                Err(_) => {
                    warn!("Couldnt query ghost tile.");
                }
            },
            None => {}
        }
    }
}

pub(crate) fn change_ghost_tile_request(
    mut net: EventReader<IncomingReliableServerMessage<GridmapServerMessage>>,
    mut events: EventWriter<ConstructionCellSelectionChanged>,
    mut select_state: ResMut<GridmapConstructionState>,
) {
    for message in net.iter() {
        match &message.message {
            GridmapServerMessage::GhostCellType(type_id) => {
                select_state.ghost_tile = Some(GhostTile {
                    tile_type: *type_id,
                });
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
        if state.ghost_tile.is_none() {
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

        net.send(OutgoingReliableClientMessage {
            message: GridmapClientMessage::ConstructCell(ConstructCell {
                cell: TargetCell {
                    id: cell_id,
                    face: state.ghost_face.clone(),
                },
                orientation: state.ghost_rotation,
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

        net.send(OutgoingReliableClientMessage {
            message: GridmapClientMessage::DeconstructCell(DeconstructCell {
                cell: TargetCell {
                    id: cell_id,
                    face: state.ghost_face.clone(),
                },
            }),
        });
    }
}
