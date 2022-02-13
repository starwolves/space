use bevy::{prelude::{Added, Commands, Entity, EventWriter, Query, ResMut}};

use crate::space::{entities::human_male_pawn::spawn::HumanMalePawnBundle, core::{pawn::{components::{Spawning, PersistentPlayerData, ConnectedPlayer}, events::NetOnSpawning, resources::{HandleToEntity, UsedNames}}, entity::resources::{EntityDataResource, SpawnPawnData}, networking::resources::{ReliableServerMessage, ServerConfigMessage}}};

pub fn on_spawning(
    mut net_on_new_player_connection : EventWriter<NetOnSpawning>,
    query : Query<(Entity, &Spawning, &ConnectedPlayer, &PersistentPlayerData),Added<Spawning>>,
    mut commands : Commands,
    mut handle_to_entity : ResMut<HandleToEntity>,
    mut used_names : ResMut<UsedNames>,
    entity_data : ResMut<EntityDataResource>,
) {
    
    for (
        entity_id,
        spawning_component,
        connected_player_component,
        persistent_player_data_component,
    ) in query.iter() {

        let passed_inventory_setup = vec![
            ("jumpsuit".to_string(), "jumpsuitSecurity".to_string()),
            ("helmet".to_string(), "helmetSecurity".to_string()),
            ("holster".to_string(), "pistolL1".to_string()),
            ("left_hand".to_string(), "constructionTool".to_string()),
        ];

        let new_entity = HumanMalePawnBundle::spawn(
            spawning_component.transform,
            &mut commands,
            true,
            Some(SpawnPawnData{
                data: (
                    persistent_player_data_component,
                    Some(connected_player_component),
                    passed_inventory_setup,
                    false,
                    false,
                    None,
                    None,
                    Some(persistent_player_data_component.user_name.clone()),
                    &entity_data,
                )
            }),
            None,
        );


        let handle = *handle_to_entity.inv_map.get(&entity_id).unwrap();

        handle_to_entity.inv_map.remove(&entity_id);
        handle_to_entity.inv_map.insert(new_entity, handle);

        handle_to_entity.map.remove(&handle);
        handle_to_entity.map.insert(handle, new_entity);

        used_names.names.insert(persistent_player_data_component.character_name.clone(), new_entity);

        commands.entity(entity_id).despawn();
        
        net_on_new_player_connection.send(NetOnSpawning{
            handle: handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(new_entity.to_bits()))
        });


    }

}
