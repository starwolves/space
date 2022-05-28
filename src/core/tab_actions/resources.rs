use bevy_ecs::entity::Entity;

use crate::core::networking::resources::GridMapType;

#[derive(Default)]
pub struct QueuedTabActions {
    pub queue: Vec<QueuedTabAction>,
}

pub struct QueuedTabAction {
    pub tab_id: String,
    pub handle_option: Option<u32>,
    pub target_cell_option: Option<(GridMapType, i16, i16, i16)>,
    pub target_entity_option: Option<u64>,
    pub belonging_entity_option: Option<u64>,
    pub player_entity: Entity,
}
