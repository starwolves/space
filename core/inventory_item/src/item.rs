use std::collections::HashMap;

use api::{combat::CombatStandardAnimation, inventory::SlotType, tab_actions::TabAction};
use bevy::prelude::{Component, Entity, Transform};

#[derive(Component)]
pub struct InventoryItem {
    pub in_inventory_of_entity: Option<Entity>,
    pub attachment_transforms: HashMap<String, Transform>,
    pub drop_transform: Transform,
    pub slot_type: SlotType,
    pub is_attached_when_worn: bool,
    pub active_slot_tab_actions: Vec<TabAction>,
    pub throw_force_factor: f32,
    pub combat_standard_animation: CombatStandardAnimation,
}
impl Default for InventoryItem {
    fn default() -> Self {
        Self {
            in_inventory_of_entity: None,
            attachment_transforms: HashMap::default(),
            drop_transform: Transform::default(),
            slot_type: SlotType::Generic,
            is_attached_when_worn: true,
            combat_standard_animation: CombatStandardAnimation::StandardStance,
            /*combat_attack_animation: CombatAttackAnimation::OneHandedMeleePunch,
            combat_melee_text_set: InventoryItem::get_default_strike_words(),
            combat_projectile_text_set: None,
            trigger_melee_text_set: InventoryItem::get_default_trigger_melee_words(),
            trigger_projectile_text_set: None,*/
            active_slot_tab_actions: vec![],
            throw_force_factor: 1.,
        }
    }
}
