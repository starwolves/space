use std::collections::HashMap;

use bevy::{
    prelude::{warn, Changed, Component, Entity, EventWriter, Query, Res, ResMut, Transform},
    time::{FixedTimesteps, Time},
};
use networking::{
    messages::{
        EntityUpdateData, NetLoadEntity, NetUnloadEntity, ReliableServerMessage,
        UnreliableServerMessage,
    },
    plugin::RENET_UNRELIABLE_CHANNEL_ID,
};
use networking_macros::NetMessage;
use serde::Deserialize;

use bevy_renet::renet::RenetServer;
use bincode::serialize;
use networking::messages::PendingMessage;
use networking::messages::PendingNetworkMessage;
use server_instance::core::HandleToEntity;

use crate::{
    meta::{EntityDataProperties, EntityDataResource},
    sensable::Sensable,
};
/// Initialize meta-data for an entity as a function.
#[cfg(feature = "server")]
pub fn initialize_entity_data(
    entity_data: &mut ResMut<EntityDataResource>,
    entity_properties: EntityDataProperties,
) {
    entity_data
        .id_to_name
        .insert(entity_properties.id, entity_properties.name.clone());
    entity_data
        .name_to_id
        .insert(entity_properties.name.clone(), entity_properties.id);
    entity_data.data.push(entity_properties);
}

/// Broadcast transforms of entities to players for interpolation.
#[cfg(feature = "server")]
pub(crate) fn broadcast_position_updates(
    time: Res<Time>,
    fixed_timesteps: Res<FixedTimesteps>,

    mut net: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut query_update_transform_entities: Query<(
        Entity,
        &Sensable,
        &UpdateTransform,
        &Transform,
        &mut CachedBroadcastTransform,
    )>,
) {
    let current_time_stamp = time.time_since_startup().as_millis();

    let overstep_percentage = fixed_timesteps
        .get(INTERPOLATION_LABEL1)
        .unwrap()
        .overstep_percentage();
    if overstep_percentage > 5. {
        if current_time_stamp > 60000 {
            warn!("overstep_percentage: {}", overstep_percentage);
        }
    }

    for (
        entity,
        visible_component,
        _update_transform_component,
        static_transform_component,
        mut cached_transform_component,
    ) in query_update_transform_entities.iter_mut()
    {
        if cached_transform_component.transform == *static_transform_component {
            continue;
        }

        cached_transform_component.transform = *static_transform_component;

        let new_position = static_transform_component.translation;

        for sensed_by_entity in visible_component.sensed_by.iter() {
            let player_handle_option = handle_to_entity.inv_map.get(&sensed_by_entity);

            match player_handle_option {
                Some(handle) => {
                    net.send_message(
                        *handle,
                        RENET_UNRELIABLE_CHANNEL_ID,
                        serialize::<UnreliableServerMessage>(
                            &UnreliableServerMessage::PositionUpdate(
                                entity.to_bits(),
                                new_position,
                                current_time_stamp as u64,
                            ),
                        )
                        .unwrap(),
                    );
                }
                None => {
                    continue;
                }
            }
        }
    }
}
#[cfg(feature = "server")]
pub const INTERPOLATION_LABEL1: &str = "fixed_timestep_interpolation1";
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetShowcase {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
/// Component for entities that were included and spawned with the map itself.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct DefaultMapEntity;

/// Event about spawning entities from json.
#[cfg(feature = "server")]
pub struct RawSpawnEvent {
    pub raw_entity: RawEntity,
}
/// Load json entities.
#[cfg(feature = "server")]
pub fn load_raw_map_entities(
    raw_entities: &Vec<RawEntity>,
    spawn_raw_entity: &mut EventWriter<RawSpawnEvent>,
) {
    for raw_entity in raw_entities.iter() {
        spawn_raw_entity.send(RawSpawnEvent {
            raw_entity: raw_entity.clone(),
        });
    }
}

/// json entity.
#[derive(Deserialize, Clone)]
#[cfg(feature = "server")]
pub struct RawEntity {
    pub entity_type: String,
    pub transform: String,
    pub data: String,
}
/// Component reserved server entity.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Server;

/// Component with the cache of the latest broadcasted transforms for its entity.
#[derive(Component, Default)]
#[cfg(feature = "server")]
pub struct CachedBroadcastTransform {
    pub transform: Transform,
    pub is_active: bool,
}
/// Component with transform for sound effects.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct UpdateTransform;
/// The NodePath to the node to spawn entities in on the Godot clients.
#[cfg(feature = "server")]
pub const ENTITY_SPAWN_PARENT : &str = "ColorRect/background/VBoxContainer/HBoxContainer/3dviewportPopup/Control/TabContainer/3D Viewport/Control/ViewportContainer/Viewport/Spatial";

/// Check if entity updates for a player has changed.
#[cfg(feature = "server")]
pub fn entity_update_changed_detection(
    changed_parameters: &mut Vec<String>,
    entity_updates: &mut HashMap<String, EntityUpdateData>,
    set: EntityUpdateData,
    parameter: String,
) {
    let get = entity_updates.get(&parameter);
    let has_changed;
    match get {
        Some(value) => {
            has_changed = !entity_data_is_matching(value, &set);
        }
        None => {
            has_changed = true;
        }
    }

    if has_changed == true {
        entity_updates.insert(parameter.clone(), set);
        changed_parameters.push(parameter);
    }
}

/// The base entity component holding base entity data.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct EntityData {
    pub entity_class: String,
    pub entity_name: String,
    pub entity_group: EntityGroup,
}

#[cfg(feature = "server")]
impl Default for EntityData {
    fn default() -> Self {
        Self {
            entity_class: "".to_string(),
            entity_name: "".to_string(),
            entity_group: EntityGroup::None,
        }
    }
}

#[derive(Copy, Clone)]
#[cfg(feature = "server")]
pub enum EntityGroup {
    None,
    AirLock,
    CounterWindowSensor,
    Pawn,
}

/// Entity update component containing Godot node related updates for clients for visual changes.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct EntityUpdates {
    pub updates: HashMap<String, HashMap<String, EntityUpdateData>>,
    pub updates_difference: Vec<HashMap<String, HashMap<String, EntityUpdateData>>>,
    pub changed_parameters: Vec<String>,
    pub excluded_handles: HashMap<String, Vec<u64>>,
}

#[cfg(feature = "server")]
impl Default for EntityUpdates {
    fn default() -> Self {
        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());
        Self {
            updates: entity_updates_map,
            changed_parameters: vec![],
            excluded_handles: HashMap::new(),
            updates_difference: vec![],
        }
    }
}

/// Match entity data as a function.
#[cfg(feature = "server")]
pub fn entity_data_is_matching(data1: &EntityUpdateData, data2: &EntityUpdateData) -> bool {
    let mut is_not_matching = true;

    match data1 {
        EntityUpdateData::Int(old_value) => match data2 {
            EntityUpdateData::Int(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::UInt8(old_value) => match data2 {
            EntityUpdateData::UInt8(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::String(old_value) => match data2 {
            EntityUpdateData::String(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::StringVec(old_value) => match data2 {
            EntityUpdateData::StringVec(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Float(old_value) => match data2 {
            EntityUpdateData::Float(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Transform(old_value, old_value1, old_value2) => match data2 {
            EntityUpdateData::Transform(new_value, new_value1, new_value2) => {
                is_not_matching = *new_value != *old_value
                    || *old_value1 != *new_value1
                    || *old_value2 != *new_value2;
            }
            _ => {}
        },
        EntityUpdateData::Color(r, g, b, a) => match data2 {
            EntityUpdateData::Color(r_n, g_n, b_n, a_n) => {
                is_not_matching = r != r_n && g != g_n && b != b_n && a != a_n;
            }
            _ => {}
        },
        EntityUpdateData::Bool(old_value) => match data2 {
            EntityUpdateData::Bool(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Vec3(old_value) => match data2 {
            EntityUpdateData::Vec3(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::AttachedItem(old_value0, old_value1, old_value2, old_value3) => {
            match data2 {
                EntityUpdateData::AttachedItem(new_value0, new_value1, new_value2, new_value3) => {
                    is_not_matching = *new_value0 != *old_value0
                        || *new_value1 != *old_value1
                        || *new_value2 != *old_value2
                        || *new_value3 != *old_value3;
                }
                _ => {}
            }
        }
        EntityUpdateData::WornItem(
            old_value0,
            old_value1,
            old_value2,
            old_value3,
            old_value4,
            old_value5,
        ) => match data2 {
            EntityUpdateData::WornItem(
                new_value0,
                new_value1,
                new_value2,
                new_value3,
                new_value4,
                new_value5,
            ) => {
                is_not_matching = *new_value0 != *old_value0
                    || *new_value1 != *old_value1
                    || *new_value2 != *old_value2
                    || *new_value3 != *old_value3
                    || *new_value4 != *old_value4
                    || *new_value5 != *old_value5;
            }
            _ => {}
        },
        EntityUpdateData::WornItemNotAttached(old_value0, old_value1, old_value2) => match data2 {
            EntityUpdateData::WornItemNotAttached(new_value0, new_value1, new_value2) => {
                is_not_matching = *new_value0 != *old_value0
                    || *new_value1 != *old_value1
                    || *new_value2 != *old_value2;
            }
            _ => {}
        },
        EntityUpdateData::Vec2(old_value0) => match data2 {
            EntityUpdateData::Vec2(new_value0) => is_not_matching = *new_value0 != *old_value0,
            _ => {}
        },
    }

    !is_not_matching
}

/// Personalise entity update set.
#[cfg(feature = "server")]
pub fn personalise(
    updates_data: &mut HashMap<String, HashMap<String, EntityUpdateData>>,
    player_handle: u64,
    entity_updates_component: &EntityUpdates,
) {
    let mut to_be_removed_parameters = vec![];

    for key_value in entity_updates_component.excluded_handles.clone() {
        if updates_data.contains_key(&key_value.0) && key_value.1.contains(&player_handle) {
            to_be_removed_parameters.push(key_value.0);
        }
    }

    for parameter in to_be_removed_parameters {
        updates_data.remove(&parameter);
    }
}

/// Get difference between this frame and last's frame entity updates per player.
#[cfg(feature = "server")]
pub fn get_entity_update_difference(
    old_data: HashMap<String, HashMap<String, EntityUpdateData>>,
    new_data: &HashMap<String, HashMap<String, EntityUpdateData>>,
) -> HashMap<String, HashMap<String, EntityUpdateData>> {
    let mut difference_data = HashMap::new();

    for (new_node_path, new_data_entity_updates) in new_data {
        match old_data.get(new_node_path) {
            Some(old_data_entity_updates) => {
                for (new_entity_update_type, new_entity_update_data) in new_data_entity_updates {
                    match old_data_entity_updates.get(new_entity_update_type) {
                        Some(old_entity_update_data) => {
                            if !entity_data_is_matching(
                                new_entity_update_data,
                                old_entity_update_data,
                            ) {
                                if !difference_data.contains_key(&new_node_path.to_string()) {
                                    difference_data
                                        .insert(new_node_path.to_string(), HashMap::new());
                                }
                                let difference_data_entity_updates =
                                    difference_data.get_mut(&new_node_path.to_string()).unwrap();
                                difference_data_entity_updates.insert(
                                    new_entity_update_type.clone(),
                                    new_entity_update_data.clone(),
                                );
                            }
                        }
                        None => {
                            if !difference_data.contains_key(&new_node_path.to_string()) {
                                difference_data.insert(new_node_path.to_string(), HashMap::new());
                            }
                            let difference_data_entity_updates =
                                difference_data.get_mut(&new_node_path.to_string()).unwrap();
                            difference_data_entity_updates.insert(
                                new_entity_update_type.clone(),
                                new_entity_update_data.clone(),
                            );
                        }
                    }
                }
            }
            None => {
                difference_data.insert(new_node_path.to_string(), new_data_entity_updates.clone());
            }
        }
    }

    difference_data
}

/// Load an entity in for the client as a function.
#[cfg(feature = "server")]
pub fn load_entity(
    entity_updates: &HashMap<String, HashMap<String, EntityUpdateData>>,
    entity_transform: Transform,
    interpolated_transform: bool,
    net_load_entity: &mut EventWriter<NetLoadEntity>,
    player_handle: u64,
    entity_data: &EntityData,
    entity_updates_component: &EntityUpdates,
    entity_id: Entity,
    load_entirely: bool,
) {
    let mut hash_map;

    if load_entirely {
        hash_map = entity_updates.clone();

        personalise(&mut hash_map, player_handle, entity_updates_component);

        let transform_entity_update = EntityUpdateData::Transform(
            entity_transform.translation,
            entity_transform.rotation,
            entity_transform.scale,
        );

        match interpolated_transform {
            true => {
                let mut transform_hash_map = HashMap::new();
                transform_hash_map.insert("transform".to_string(), transform_entity_update);

                hash_map.insert("rawTransform".to_string(), transform_hash_map);
            }
            false => {
                let root_map_option = hash_map.get_mut(&".".to_string());

                match root_map_option {
                    Some(root_map) => {
                        root_map.insert("transform".to_string(), transform_entity_update);
                    }
                    None => {
                        let mut transform_hash_map = HashMap::new();
                        transform_hash_map.insert("transform".to_string(), transform_entity_update);

                        hash_map.insert(".".to_string(), transform_hash_map);
                    }
                }
            }
        }
    } else {
        hash_map = HashMap::new();
    }

    net_load_entity.send(NetLoadEntity {
        handle: player_handle,
        message: ReliableServerMessage::LoadEntity(
            entity_data.entity_class.clone(),
            entity_data.entity_name.clone(),
            hash_map,
            entity_id.to_bits(),
            load_entirely,
            "main".to_string(),
            "".to_string(),
            false,
        ),
    });
}

/// Unload an entity in for the client as a function.
#[cfg(feature = "server")]
pub fn unload_entity(
    player_handle: u64,
    entity_id: Entity,
    net_unload_entity: &mut EventWriter<NetUnloadEntity>,
    unload_entirely: bool,
) {
    net_unload_entity.send(NetUnloadEntity {
        handle: player_handle,
        message: ReliableServerMessage::UnloadEntity(entity_id.to_bits(), unload_entirely),
    });
}

/// World mode component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct WorldMode {
    pub mode: WorldModes,
}

/// All world modes.
#[derive(Debug)]
#[cfg(feature = "server")]
pub enum WorldModes {
    Static,
    Kinematic,
    Physics,
    Held,
    Worn,
}

/// Physics entity change world mode for Godot client.
#[cfg(feature = "server")]
pub(crate) fn world_mode_update(
    mut updated_entities: Query<(&WorldMode, &mut EntityUpdates), Changed<WorldMode>>,
) {
    for (world_mode_component, mut entity_updates_component) in updated_entities.iter_mut() {
        let old_entity_updates = entity_updates_component.updates.clone();

        let world_mode;

        match world_mode_component.mode {
            WorldModes::Static => {
                world_mode = "static";
            }
            WorldModes::Kinematic => {
                world_mode = "kinematic";
            }
            WorldModes::Physics => {
                world_mode = "physics";
            }
            WorldModes::Worn => {
                world_mode = "worn";
            }
            WorldModes::Held => {
                world_mode = "held";
            }
        };

        let entity_updates = entity_updates_component
            .updates
            .get_mut(&".".to_string())
            .unwrap();

        entity_updates.insert(
            "world_mode".to_string(),
            EntityUpdateData::String(world_mode.to_string()),
        );

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}
