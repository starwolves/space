use std::collections::HashMap;

use bevy::{
    ecs::system::{Query, Res, ResMut, Resource},
    prelude::{Component, Entity, Vec2},
};
use networking::stamp::TickRateStamp;
use pawn::pawn::FacingDirection;
use resources::math::Vec3Int;

/// Controller input component.
#[derive(Component, Clone)]

pub struct ControllerInput {
    pub movement_vector: Vec2,
    pub sprinting: bool,
    pub is_mouse_action_pressed: bool,
    pub auto_move_enabled: bool,
    pub auto_move_direction: Vec2,
    pub combat_targetted_entity: Option<Entity>,
    pub combat_targetted_cell: Option<Vec3Int>,
    pub alt_attack_mode: bool,
    pub combat_mode: bool,
    pub pending_direction: Option<FacingDirection>,
}
impl Default for ControllerInput {
    fn default() -> Self {
        Self {
            movement_vector: Vec2::ZERO,
            sprinting: false,
            is_mouse_action_pressed: false,
            auto_move_enabled: false,
            auto_move_direction: Vec2::ZERO,
            combat_targetted_entity: None,
            combat_targetted_cell: None,
            alt_attack_mode: false,
            combat_mode: false,
            pending_direction: None,
        }
    }
}
#[derive(Resource, Default, Clone)]
pub struct ControllerCache {
    pub cache: HashMap<u64, HashMap<Entity, ControllerInput>>,
}

pub(crate) fn cache_controller(
    query: Query<(Entity, &ControllerInput)>,
    stamp: Res<TickRateStamp>,
    mut cache: ResMut<ControllerCache>,
) {
    for (entity, controller) in query.iter() {
        match cache.cache.get_mut(&stamp.large) {
            Some(map) => {
                map.insert(entity, controller.clone());
            }
            None => {
                let mut map = HashMap::new();
                map.insert(entity, controller.clone());
                cache.cache.insert(stamp.large, map);
            }
        }
    }

    // Clean cache.
    let mut to_remove = vec![];
    for recorded_stamp in cache.cache.keys() {
        if stamp.large >= 256 && recorded_stamp < &(stamp.large - 256) {
            to_remove.push(*recorded_stamp);
        }
    }
    for i in to_remove {
        cache.cache.remove(&i);
    }
}
