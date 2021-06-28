use bevy::prelude::{Entity, EventWriter, Res};

use crate::space_core::{events::net::net_unload_entity::NetUnloadEntity, functions::unload_entity_for_player::unload_entity, resources::handle_to_entity::HandleToEntity};

pub struct Sensable{
    pub is_light : bool,
    pub is_audible : bool,
    pub sensed_by : Vec<Entity>,
    pub sensed_by_cached : Vec<Entity>,
    pub always_sensed : bool
}


impl Sensable {
    pub fn despawn(
        &self,
        entity : Entity,
        mut net_unload_entity : &mut EventWriter<NetUnloadEntity>,
        handle_to_entity : &Res<HandleToEntity>,
    ) {

        for sensed_by_entity in self.sensed_by.iter() {

            match handle_to_entity.inv_map.get(&sensed_by_entity.id()) {
                Some(handle) => {
                    unload_entity(*handle, entity.id(), &mut net_unload_entity, true);
                }
                None => {}
            }

            

        }

    }
}
