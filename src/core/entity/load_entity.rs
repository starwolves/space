use std::collections::HashMap;

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

use bevy::prelude::{Entity, EventWriter, Transform};

use crate::core::networking::networking::{EntityUpdateData, ReliableServerMessage};

use super::{
    entity_data::EntityData,
    entity_updates::{personalise, EntityUpdates},
};

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

pub struct NetLoadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetUnloadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
