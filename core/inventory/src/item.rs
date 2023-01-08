use std::collections::HashMap;

use crate::inventory::SlotType;
use bevy::prelude::{Component, Entity, Transform};

/// Humanoid animations for combat.

pub enum CombatStandardAnimation {
    StandardStance,
    PistolStance,
}

/// Inventory item component.
#[derive(Component)]

pub struct InventoryItem {
    /// Entity that is holding this entity.
    pub in_inventory_of_entity: Option<Entity>,
    /// What transform this entity has per attachment slot.
    pub attachment_transforms: HashMap<String, Transform>,
    /// Mainly the set rotation for when an entity gets dropped.
    pub drop_transform: Transform,
    /// The slot type this item attaches to.
    pub slot_type: SlotType,
    /// Items that are worn by attachment to a slot have this set to true.
    pub is_attached_when_worn: bool,
    /// How far the entity will be thrown.
    pub throw_force_factor: f32,
    /// The to be played animation when in combat mode whilst holding this item.
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
            throw_force_factor: 1.,
        }
    }
}
