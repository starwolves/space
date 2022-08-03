use api::humanoid::MELEE_FISTS_REACH;
use bevy::prelude::{EventReader, EventWriter};
use combat::{attack::Attack, melee_queries::MeleeDirectQuery};

pub fn hands_attack_handler(
    mut attacks: EventReader<Attack>,
    mut melee_attack: EventWriter<MeleeDirectQuery>,
) {
    for attack in attacks.iter() {
        if attack.weapon_option.is_none() {
            melee_attack.send(MeleeDirectQuery {
                attacker_entity: attack.attacker,
                targetted_entity: attack.targetted_entity,
                targetted_cell: attack.targetted_cell,
                angle: attack.angle,
                range: MELEE_FISTS_REACH,
                exclude_physics: vec![],
                barehanded: true,
                incremented_id: attack.incremented_id,
            });
        }
    }
}
