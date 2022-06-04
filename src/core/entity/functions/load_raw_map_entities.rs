use bevy_ecs::event::EventWriter;

use crate::core::entity::events::RawSpawnEvent;

use super::raw_entity::RawEntity;

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
