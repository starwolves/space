use bevy::prelude::Entity;

pub struct InputTabDataEntity {
    pub handle : u32,
    pub player_entity: Entity,
    pub examine_entity_bits : u64,
}
