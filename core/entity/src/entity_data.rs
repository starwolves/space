use api::{
    data::{EntityDataProperties, EntityDataResource, HandleToEntity},
    network::{
        PendingMessage, PendingNetworkMessage, ReliableServerMessage, UnreliableServerMessage,
    },
    sensable::Sensable,
};
use bevy::{
    prelude::{warn, Component, Entity, EventWriter, Query, Res, ResMut, Transform},
    time::{FixedTimesteps, Time},
};
use networking::plugin::RENET_UNRELIABLE_CHANNEL_ID;
use serde::Deserialize;
pub const CONSTRUCTION_TOOL_ENTITY_NAME: &str = "constructionTool";
pub const HELMET_SECURITY_ENTITY_NAME: &str = "helmetSecurity";
/// Initialize meta-data for an entity as a function.
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

use bevy_renet::renet::RenetServer;
use bincode::serialize;

/// Broadcast transforms of entities to players for interpolation.
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
pub const INTERPOLATION_LABEL1: &str = "fixed_timestep_interpolation1";

pub struct NetShowcase {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetShowcase {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
/// Entities that were included and spawned with the map itself.
#[derive(Component)]
pub struct DefaultMapEntity;

/// Event about spawning entities from json.
pub struct RawSpawnEvent {
    pub raw_entity: RawEntity,
}
/// Load json entities.
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
pub struct RawEntity {
    pub entity_type: String,
    pub transform: String,
    pub data: String,
}
/// The reserved server entity.
#[derive(Component)]
pub struct Server;

/// The cache of the latest broadcasted transforms.
#[derive(Component, Default)]
pub struct CachedBroadcastTransform {
    pub transform: Transform,
    pub is_active: bool,
}
/// UpdateTransform for sound effects.
#[derive(Component)]
pub struct UpdateTransform;
/// The NodePath to the node to spawn entities in on the Godot clients.
pub const ENTITY_SPAWN_PARENT : &str = "ColorRect/background/VBoxContainer/HBoxContainer/3dviewportPopup/Control/TabContainer/3D Viewport/Control/ViewportContainer/Viewport/Spatial";
