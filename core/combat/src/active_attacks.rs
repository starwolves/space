use std::collections::HashMap;

use bevy::prelude::{EventReader, ResMut};

use crate::attack::{Attack, QueryCombatHitResult};

/// Resource storing current incremented attack id.
#[derive(Default)]
pub struct ActiveAttackIncrement {
    /// Attack id.
    incremented_id: u64,
}

impl ActiveAttackIncrement {
    /// Get a unique attack ID and increment the resource so the next call will also be unique.
    pub fn get_id_inc(&mut self) -> u64 {
        let return_val = self.incremented_id.clone();
        self.incremented_id += 1;
        return_val
    }
}

/// Resource with an active attack cache.
#[derive(Default)]
pub struct ActiveAttacks {
    pub map: HashMap<u64, ActiveAttack>,
}

/// An data struct for the [ActiveAttacks] cache.
pub struct ActiveAttack {
    pub attack: Attack,
    /// Physics hit result.
    pub hit_result: Option<QueryCombatHitResult>,
    /// Match content of option to find out if it is a melee attack or not.
    pub melee: Option<bool>,
}

/// Cache attacks with [ActiveAttacks].
pub fn cache_attacks(
    mut attack_events: EventReader<Attack>,
    mut cached_attacks: ResMut<ActiveAttacks>,
) {
    cached_attacks.map.clear();
    for attack in attack_events.iter() {
        cached_attacks.map.insert(
            attack.incremented_id,
            ActiveAttack {
                attack: attack.clone(),
                hit_result: None,
                melee: None,
            },
        );
    }
}
