use bevy::prelude::{Component, Entity, Vec2};
use math::grid::Vec3Int;
use pawn::pawn::FacingDirection;

/// Controller input component.
#[derive(Component)]

pub struct ControllerInput {
    pub movement_vector: Vec2,
    pub sprinting: bool,
    pub is_mouse_action_pressed: bool,
    pub targetted_limb: String,
    pub auto_move_enabled: bool,
    pub auto_move_direction: Vec2,
    pub combat_targetted_entity: Option<Entity>,
    pub combat_targetted_cell: Option<Vec3Int>,
    pub alt_attack_mode: bool,
    pub pending_direction: Option<FacingDirection>,
}
impl Default for ControllerInput {
    fn default() -> Self {
        Self {
            movement_vector: Vec2::ZERO,
            sprinting: false,
            is_mouse_action_pressed: false,
            targetted_limb: "torso".to_string(),
            auto_move_enabled: false,
            auto_move_direction: Vec2::ZERO,
            combat_targetted_entity: None,
            combat_targetted_cell: None,
            alt_attack_mode: false,
            pending_direction: None,
        }
    }
}
