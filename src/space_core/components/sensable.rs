use bevy::prelude::{Entity, EventWriter, Res, Component};

use crate::space_core::{events::net::net_unload_entity::NetUnloadEntity, functions::entity_updates::unload_entity_for_player::unload_entity, resources::handle_to_entity::HandleToEntity};

#[derive(Component)]
pub struct Sensable{
    pub is_light : bool,
    pub is_audible : bool,
    pub sensed_by : Vec<Entity>,
    pub sensed_by_cached : Vec<Entity>,
    pub always_sensed : bool
}

impl Default for Sensable {
    fn default() -> Self {
        Self {
            is_audible : false,
            is_light:false,
            sensed_by_cached:vec![],
            sensed_by:vec![],
            always_sensed : false
        }
    }
}


impl Sensable {
    pub fn despawn(
        &mut self,
        entity : Entity,
        mut net_unload_entity : &mut EventWriter<NetUnloadEntity>,
        handle_to_entity : &Res<HandleToEntity>,
    ) {

        // Shouldn't be called from the same stage visible_checker.system() runs in.

        for sensed_by_entity in self.sensed_by.iter() {
            match handle_to_entity.inv_map.get(&sensed_by_entity) {
                Some(handle) => {
                    unload_entity(*handle, entity, &mut net_unload_entity, true);
                }
                None => {}
            }
        }
        for sensed_by_entity in self.sensed_by_cached.iter() {
            match handle_to_entity.inv_map.get(&sensed_by_entity) {
                Some(handle) => {
                    unload_entity(*handle, entity, &mut net_unload_entity, true);
                }
                None => {}
            }
        }

        self.sensed_by = vec![];
        self.sensed_by_cached = vec![];
    }
}
