use bevy::prelude::{warn, EventReader, Query, With};
use inventory::{inventory::Inventory, item_events::ThrownItem};

use controller::controller::ControllerInput;

use crate::humanoid::{CharacterAnimationState, Humanoid};

/// Face the direction of the thrown entity.
pub(crate) fn thrown_item_adjust_facingdirection(
    mut events: EventReader<ThrownItem>,
    mut inventory_holders: Query<(&Humanoid, &mut ControllerInput), With<Inventory>>,
) {
    for event in events.iter() {
        match inventory_holders.get_mut(event.inventory_entity) {
            Ok((humanoid, mut controller_input)) => match humanoid.current_lower_animation_state {
                CharacterAnimationState::Idle => {
                    if !humanoid.combat_mode {
                        controller_input.pending_direction = Some(event.direction.clone());
                    }
                }
                _ => (),
            },
            Err(_) => {
                warn!(
                    "Couldn't find inventory holder for {:?}",
                    event.inventory_entity
                );
            }
        }
    }
}
