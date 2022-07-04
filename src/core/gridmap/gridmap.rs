pub fn gridmap_updates(
    mut gridmap_main: ResMut<GridmapMain>,
    mut gridmap_details1: ResMut<GridmapDetails1>,
    sensers: Query<(Entity, &Senser, &ConnectedPlayer)>,
    mut net_gridmap_updates: EventWriter<NetGridmapUpdates>,
) {
    for (cell_id, cell_update) in gridmap_main.updates.iter_mut() {
        let cell_coords = to_doryen_coordinates(cell_id.x, cell_id.z);
        for (senser_entity, senser_component, connected_player_component) in sensers.iter() {
            if connected_player_component.connected
                && !cell_update.entities_received.contains(&senser_entity)
                && senser_component.fov.is_in_fov(cell_coords.0, cell_coords.1)
            {
                cell_update.entities_received.push(senser_entity);
                if cell_update.cell_data.item != -1 {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::AddCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            cell_update.cell_data.item,
                            cell_update.cell_data.orientation,
                            GridMapType::Main,
                        ),
                    });
                } else {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::RemoveCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            GridMapType::Main,
                        ),
                    });
                }
            }
        }
    }

    for (cell_id, cell_update) in gridmap_details1.updates.iter_mut() {
        let cell_coords = to_doryen_coordinates(cell_id.x, cell_id.z);

        for (senser_entity, senser_component, connected_player_component) in sensers.iter() {
            if connected_player_component.connected
                && !cell_update.entities_received.contains(&senser_entity)
                && senser_component.fov.is_in_fov(cell_coords.0, cell_coords.1)
            {
                cell_update.entities_received.push(senser_entity);
                if cell_update.cell_data.item != -1 {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::AddCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            cell_update.cell_data.item,
                            cell_update.cell_data.orientation,
                            GridMapType::Details1,
                        ),
                    });
                } else {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::RemoveCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            GridMapType::Details1,
                        ),
                    });
                }
            }
        }
    }
}

pub fn get_cell_name(ship_cell: &CellData, gridmap_data: &Res<GridmapData>) -> String {
    gridmap_data
        .main_text_names
        .get(&ship_cell.item)
        .unwrap()
        .get_a_name()
}

const CELL_SIZE: f32 = 2.;
const Y_CENTER_OFFSET: f32 = 1.;

pub fn cell_id_to_world(cell_id: Vec3Int) -> Vec3 {
    let mut world_position: Vec3 = Vec3::ZERO;

    world_position.x = (cell_id.x as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z as f32 * CELL_SIZE) + Y_CENTER_OFFSET;

    world_position
}

pub fn world_to_cell_id(position: Vec3) -> Vec3Int {
    let map_pos = position / CELL_SIZE;

    Vec3Int {
        x: map_pos.x.floor() as i16,
        y: map_pos.y.floor() as i16,
        z: map_pos.z.floor() as i16,
    }
}

use bevy::{
    hierarchy::Children,
    math::{Quat, Vec3},
    prelude::{
        warn, Commands, Component, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform,
        With,
    },
};
use const_format::concatcp;
use rand::Rng;

pub const EXAMINATION_EMPTY: &str = "You cannot see what is there.";
pub const END_ASTRIX: &str = concatcp!("\n", ASTRIX, "[/font]");

pub fn examine_ship_cell(
    ship_cell: &CellData,
    gridmap_type: &GridMapType,
    gridmap_data: &Res<GridmapData>,
) -> String {
    let examine_text: &str;
    let mut message = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n";
    message = message
        + "[font="
        + FURTHER_ITALIC_FONT
        + "]"
        + "You examine the "
        + &gridmap_data
            .main_text_names
            .get(&ship_cell.item)
            .unwrap()
            .get_name()
        + ".[/font]\n";

    if ship_cell.item != -1 {
        match gridmap_type {
            GridMapType::Main => {
                examine_text = gridmap_data
                    .main_text_examine_desc
                    .get(&ship_cell.item)
                    .unwrap();
            }
            GridMapType::Details1 => {
                examine_text = gridmap_data
                    .details1_text_examine_desc
                    .get(&ship_cell.item)
                    .unwrap();
            }
        }
    } else {
        examine_text = EXAMINATION_EMPTY;
    }

    message = message + examine_text;

    if ship_cell.health.brute < 25. && ship_cell.health.burn < 25. && ship_cell.health.toxin < 25. {
        message = message
            + "[font="
            + FURTHER_ITALIC_FONT
            + "][color="
            + HEALTHY_COLOR
            + "]\nIt is in perfect shape.[/color][/font]";
    } else {
        if ship_cell.health.brute > 75. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt is heavily damaged.[/color][/font]";
        } else if ship_cell.health.brute > 50. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt is damaged.[/color][/font]";
        } else if ship_cell.health.brute > 25. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt is slightly damaged.[/color][/font]";
        }

        if ship_cell.health.burn > 75. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt suffers from heavy burn damage.[/color][/font]";
        } else if ship_cell.health.burn > 50. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt suffers burn damage.[/color][/font]";
        } else if ship_cell.health.burn > 25. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt has slight burn damage.[/color][/font]";
        }
    }

    message
}

pub fn get_empty_cell_message() -> String {
    "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n" + EXAMINATION_EMPTY
}

pub fn get_space_message() -> String {
    let mut rng = rand::thread_rng();
    let random_pick: i32 = rng.gen_range(0..3);

    let mut msg = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n";
    msg = msg + "[font=" + FURTHER_ITALIC_FONT + "]" + "You examine the empty space.[/font]\n";

    if random_pick == 0 {
        msg = msg + "You are starstruck by the sight of space.";
    } else if random_pick == 1 {
        msg = msg + "That certainly looks like space.";
    } else {
        msg = msg + "Space.";
    }

    msg.to_string()
}

use crate::core::{
    atmospherics::{
        difussion::{get_atmos_index, AtmosphericsResource, EffectType},
        effects::VACUUM_ATMOSEFFECT,
    },
    chat::{
        message::{
            ASTRIX, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR,
        },
        net::NetChatMessage,
    },
    combat::attack::HitSoundSurface,
    connected_player::{connection::ConnectedPlayer, plugin::HandleToEntity},
    entity::entity_data::string_transform_to_transform,
    examinable::examinable::RichName,
    health::health::{calculate_damage, DamageModel, DamageType, HealthFlag, HitResult},
    networking::networking::{GridMapType, ReliableServerMessage},
    senser::visible_checker::Senser,
};

#[derive(Component)]
pub struct Cell {
    pub id: Vec3Int,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            id: Vec3Int { x: 0, y: 0, z: 0 },
        }
    }
}

pub fn to_doryen_coordinates(x: i16, y: i16) -> (usize, usize) {
    let mut n_x = x + FOV_MAP_WIDTH as i16 / 2;
    let mut n_y = y + FOV_MAP_WIDTH as i16 / 2;

    if doryen_coordinates_out_of_range(n_x as usize, n_y as usize) {
        n_x = 0;
        n_y = 0;
    }

    (n_x as usize, n_y as usize)
}

pub fn doryen_coordinates_out_of_range(x: usize, y: usize) -> bool {
    x > FOV_MAP_WIDTH || y > FOV_MAP_WIDTH
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Default)]
pub struct Vec2Int {
    pub x: i16,
    pub y: i16,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec3Int {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

// Turning up these values drastically increases fov calculation time.
// The largest maps we can support with f32 accuracy is a 2000x2000 tiled map.
// FOV calculation time will take 10x-15x slower, up to 2-3ms for just a single player calculation.
// For bigger maps than 500x500 gridmaps we need a new and better FOV algorithm.
// Dividible by 2.
pub const FOV_MAP_WIDTH: usize = 500;

#[derive(Deserialize)]
pub struct SpawnPointRaw {
    pub point_type: String,
    pub transform: String,
}

pub struct SpawnPoint {
    pub point_type: String,
    pub transform: Transform,
}

#[derive(Default)]
pub struct SpawnPoints {
    pub list: Vec<SpawnPoint>,
    pub i: usize,
}

impl SpawnPoint {
    pub fn new(raw: &SpawnPointRaw) -> SpawnPoint {
        let mut this_transform = string_transform_to_transform(&raw.transform);

        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;

        SpawnPoint {
            point_type: raw.point_type.clone(),
            transform: this_transform,
        }
    }
}

use bevy_rapier3d::prelude::RigidBody;
use doryen_fov::FovAlgorithm;

pub struct RemoveCell {
    pub handle_option: Option<u64>,
    pub gridmap_type: GridMapType,
    pub id: Vec3Int,
    pub cell_data: CellData,
}

pub fn remove_cell(
    mut deconstruct_cell_events: EventReader<RemoveCell>,
    mut gridmap_main: ResMut<GridmapMain>,
    mut gridmap_details1: ResMut<GridmapDetails1>,
    mut fov_map: ResMut<DoryenMap>,
    mut commands: Commands,
    mut sensers: Query<(&mut Senser, &ConnectedPlayer)>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
    rigid_bodies: Query<&Children, With<RigidBody>>,
) {
    for event in deconstruct_cell_events.iter() {
        match event.gridmap_type {
            GridMapType::Main => {
                let coords = to_doryen_coordinates(event.id.x, event.id.z);

                let mut atmospherics = atmospherics_resource
                    .atmospherics
                    .get_mut(get_atmos_index(Vec2Int {
                        x: event.id.x,
                        y: event.id.z,
                    }))
                    .unwrap();

                if event.id.y == 0 {
                    // Wall
                    let cell_entity = gridmap_main
                        .grid_data
                        .get(&event.id)
                        .unwrap()
                        .entity
                        .unwrap();

                    match rigid_bodies.get(cell_entity) {
                        Ok(children) => {
                            for child in children.iter() {
                                commands.entity(*child).despawn();
                            }
                        }
                        Err(_rr) => {
                            warn!("Couldnt find rigidbody beloning to cell!");
                        }
                    }

                    commands.entity(cell_entity).despawn();
                    fov_map.map.set_transparent(coords.0, coords.1, true);
                    atmospherics.blocked = false;
                    atmospherics.forces_push_up = false;
                } else {
                    let mut upper_id = event.id.clone();
                    upper_id.y = 0;

                    // Add vacuum flag to atmos.
                    match gridmap_main.grid_data.get(&upper_id) {
                        Some(_) => {}
                        None => {
                            atmospherics
                                .effects
                                .insert(EffectType::Floorless, VACUUM_ATMOSEFFECT);
                        }
                    }
                }

                match gridmap_details1.data.get(&event.id) {
                    Some(_cell_data) => {
                        let mut local_copy = event.cell_data.clone();
                        local_copy.item = -1;

                        gridmap_details1.updates.insert(
                            event.id,
                            CellUpdate {
                                entities_received: vec![],
                                cell_data: local_copy,
                            },
                        );
                    }
                    None => {}
                }

                for (mut senser_component, _connected_player_component) in sensers.iter_mut() {
                    if senser_component.fov.is_in_fov(coords.0, coords.1) {
                        senser_component.fov.clear_fov();
                        let coords = to_doryen_coordinates(
                            senser_component.cell_id.x,
                            senser_component.cell_id.y,
                        );
                        senser_component.fov.compute_fov(
                            &mut fov_map.map,
                            coords.0,
                            coords.1,
                            FOV_DISTANCE,
                            true,
                        );

                        gridmap_main.updates.insert(
                            event.id,
                            CellUpdate {
                                entities_received: vec![],
                                cell_data: event.cell_data.clone(),
                            },
                        );
                    }
                }

                gridmap_main.grid_data.remove(&event.id);
            }
            GridMapType::Details1 => {
                gridmap_details1.updates.insert(
                    event.id,
                    CellUpdate {
                        entities_received: vec![],
                        cell_data: CellData {
                            item: -1,
                            orientation: 0,
                            health: StructureHealth::default(),
                            entity: None,
                        },
                    },
                );
            }
        }
    }
}

use std::collections::HashMap;

use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

use super::{
    fov::{DoryenMap, FOV_DISTANCE},
    net::NetGridmapUpdates,
    plugin::MainCellProperties,
};

#[derive(Default)]
pub struct GridmapData {
    pub non_fov_blocking_cells_list: Vec<i64>,
    pub non_combat_obstacle_cells_list: Vec<i64>,
    pub non_laser_obstacle_cells_list: Vec<i64>,
    pub placeable_items_cells_list: Vec<i64>,
    pub ordered_main_names: Vec<String>,
    pub ordered_details1_names: Vec<String>,
    pub main_name_id_map: HashMap<String, i64>,
    pub main_id_name_map: HashMap<i64, String>,
    pub details1_name_id_map: HashMap<String, i64>,
    pub details1_id_name_map: HashMap<i64, String>,
    pub main_text_names: HashMap<i64, RichName>,
    pub details1_text_names: HashMap<i64, RichName>,
    pub main_text_examine_desc: HashMap<i64, String>,
    pub details1_text_examine_desc: HashMap<i64, String>,
    pub blackcell_id: i64,
    pub blackcell_blocking_id: i64,
    pub main_cell_properties: HashMap<i64, MainCellProperties>,
}

#[derive(Default)]
pub struct GridmapDetails1 {
    pub data: HashMap<Vec3Int, CellData>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
}

#[derive(Default)]
pub struct GridmapMain {
    pub grid_data: HashMap<Vec3Int, CellData>,
    pub entity_data: HashMap<Vec3Int, EntityGridData>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
}

pub struct CellUpdate {
    pub entities_received: Vec<Entity>,
    pub cell_data: CellData,
}

pub struct EntityGridData {
    pub entity: Entity,
    pub entity_name: String,
}

#[derive(Deserialize)]
pub struct CellDataWID {
    pub id: String,
    pub item: String,
    pub orientation: i64,
}

#[derive(Clone)]
pub struct CellData {
    pub item: i64,
    pub orientation: i64,
    pub health: StructureHealth,
    pub entity: Option<Entity>,
}

#[derive(Clone)]
pub struct StructureHealth {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
    pub health_flags: HashMap<u32, HealthFlag>,
    pub hit_sound_surface: HitSoundSurface,
}

impl Default for StructureHealth {
    fn default() -> Self {
        Self {
            brute: 0.,
            burn: 0.,
            toxin: 0.,
            health_flags: HashMap::new(),
            hit_sound_surface: HitSoundSurface::Metaloid,
        }
    }
}

impl StructureHealth {
    pub fn apply_damage(
        &mut self,
        _body_part: &str,
        damage_model: &DamageModel,
        net_new_chat_message_event: &mut EventWriter<NetChatMessage>,
        handle_to_entity: &Res<HandleToEntity>,
        attacker_cell_id: &Vec3Int,
        attacked_cell_id: &Vec3Int,
        sensers: &Query<(Entity, &Senser)>,
        attacker_name: &str,
        cell_name: &str,
        _damage_type: &DamageType,
        weapon_name: &str,
        weapon_a_name: &str,
        offense_words: &Vec<String>,
        trigger_words: &Vec<String>,
    ) -> HitResult {
        let (brute_damage, burn_damage, toxin_damage, hit_result) = calculate_damage(
            &self.health_flags,
            &damage_model.damage_flags,
            &damage_model.brute,
            &damage_model.burn,
            &damage_model.toxin,
        );

        let attacker_cell_id_doryen = to_doryen_coordinates(attacker_cell_id.x, attacker_cell_id.z);
        let attacked_cell_id_doryen = to_doryen_coordinates(attacked_cell_id.x, attacked_cell_id.z);

        self.brute += brute_damage;
        self.burn += burn_damage;
        self.toxin += toxin_damage;

        for (entity, senser) in sensers.iter() {
            let mut message = "".to_string();

            let strike_word = offense_words.choose(&mut rand::thread_rng()).unwrap();

            let attacker_is_visible;

            if senser.fov.is_in_fov(
                attacker_cell_id_doryen.0 as usize,
                attacker_cell_id_doryen.1 as usize,
            ) {
                attacker_is_visible = true;
            } else {
                attacker_is_visible = false;
            }

            let attacked_is_visible;

            if senser.fov.is_in_fov(
                attacked_cell_id_doryen.0 as usize,
                attacked_cell_id_doryen.1 as usize,
            ) {
                attacked_is_visible = true;
            } else {
                attacked_is_visible = false;
            }

            let mut should_send = false;

            if attacker_is_visible && attacked_is_visible {
                message = "[color=#ff003c]".to_string()
                    + attacker_name
                    + " has "
                    + strike_word
                    + " "
                    + cell_name
                    + " with "
                    + weapon_a_name
                    + "![/color]";
                should_send = true;
            } else if attacker_is_visible && !attacked_is_visible {
                let trigger_word = trigger_words.choose(&mut rand::thread_rng()).unwrap();
                message = "[color=#ff003c]".to_string()
                    + attacker_name
                    + " has "
                    + trigger_word
                    + " his "
                    + weapon_name
                    + "![/color]";
                should_send = true;
            } else if !attacker_is_visible && attacked_is_visible {
                message = "[color=#ff003c]".to_string()
                    + cell_name
                    + " has been "
                    + strike_word
                    + " with "
                    + weapon_a_name
                    + "![/color]";
                should_send = true;
            }

            if should_send {
                match handle_to_entity.inv_map.get(&entity) {
                    Some(handle) => {
                        net_new_chat_message_event.send(NetChatMessage {
                            handle: *handle,
                            message: ReliableServerMessage::ChatMessage(message.clone()),
                        });
                    }
                    None => {}
                }
            }
        }

        hit_result
    }
}
