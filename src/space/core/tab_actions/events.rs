use bevy_ecs::entity::Entity;

use crate::space::core::networking::resources::GridMapType;

pub struct InputTabAction {
    pub tab_id: String,
    pub player_entity: Entity,
    pub target_entity_option: Option<u64>,
    pub belonging_entity_option: Option<u64>,
    pub target_cell_option: Option<(GridMapType, i16, i16, i16)>,
}
