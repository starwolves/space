use std::collections::HashMap;

use bevy::prelude::{Entity, EventWriter, Transform};

use crate::space::core::{
    entity::{
        components::{EntityData, EntityUpdates},
        events::NetLoadEntity,
    },
    networking::resources::{EntityUpdateData, ReliableServerMessage},
};

use super::entity_updates_personalise;

pub fn load_entity(
    entity_updates: &HashMap<String, HashMap<String, EntityUpdateData>>,
    entity_transform: Transform,
    interpolated_transform: bool,
    net_load_entity: &mut EventWriter<NetLoadEntity>,
    player_handle: u32,
    entity_data: &EntityData,
    entity_updates_component: &EntityUpdates,
    entity_id: Entity,
    load_entirely: bool,
) {
    let mut hash_map;

    if load_entirely {
        hash_map = entity_updates.clone();

        entity_updates_personalise::personalise(
            &mut hash_map,
            player_handle,
            entity_updates_component,
        );

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
            entity_data.entity_type.clone(),
            hash_map,
            entity_id.to_bits(),
            load_entirely,
            "main".to_string(),
            "".to_string(),
            false,
        ),
    });
}
