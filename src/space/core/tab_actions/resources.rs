use bevy_ecs::{
    entity::Entity,
    prelude::{FromWorld, World},
};

use crate::space::core::networking::resources::GridMapType;

pub struct QueuedTabActions {
    pub queue: Vec<QueuedTabAction>,
}

pub struct QueuedTabAction {
    pub tab_id: String,
    pub handle: u32,
    pub target_cell_option: Option<(GridMapType, i16, i16, i16)>,
    pub target_entity_option: Option<u64>,
    pub belonging_entity_option: Option<u64>,
    pub player_entity: Entity,
}

impl FromWorld for QueuedTabActions {
    fn from_world(_world: &mut World) -> Self {
        QueuedTabActions { queue: vec![] }
    }
}
