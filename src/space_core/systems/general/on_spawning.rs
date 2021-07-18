use std::collections::HashMap;

use bevy::{prelude::{Added, Commands, Entity, EventWriter, Query, ResMut}};

use crate::space_core::{bundles::human_male_pawn::HumanMalePawnBundle, components::{connected_player::ConnectedPlayer, persistent_player_data::PersistentPlayerData, spawning::Spawning}, events::net::net_on_spawning::NetOnSpawning, resources::handle_to_entity::HandleToEntity, structs::network_messages::{ReliableServerMessage, ServerConfigMessage}};




pub fn on_spawning(
    mut net_on_new_player_connection : EventWriter<NetOnSpawning>,
    query : Query<(Entity, &Spawning, &ConnectedPlayer, &PersistentPlayerData),Added<Spawning>>,
    mut commands : Commands,
    mut handle_to_entity : ResMut<HandleToEntity>,
    
) {
    
    for (
        entity_id,
        spawning_component,
        connected_player_component,
        persistent_player_data_component,
    ) in query.iter() {

        let mut passed_inventory_setup = HashMap::new();

        passed_inventory_setup.insert("jumpsuit".to_string(), "jumpsuitSecurity".to_string());
        passed_inventory_setup.insert("helmet".to_string(), "helmetSecurity".to_string());

        let new_entity = HumanMalePawnBundle::spawn(
            spawning_component.transform,
            &mut commands,
            persistent_player_data_component,
            connected_player_component,
            passed_inventory_setup,
        );


        let handle = *handle_to_entity.inv_map.get(&entity_id.id()).unwrap();

        handle_to_entity.inv_map.remove(&entity_id.id());
        handle_to_entity.inv_map.insert(new_entity.id(), handle);

        handle_to_entity.map.remove(&handle);
        handle_to_entity.map.insert(handle, new_entity);

        commands.entity(entity_id).despawn();
        
        net_on_new_player_connection.send(NetOnSpawning{
            handle: handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::EntityId(new_entity.to_bits()))
        });


    }

    
    
}
