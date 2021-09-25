use std::collections::HashMap;

use bevy::prelude::{Changed, Entity, Local, Query};

use crate::space_core::components::{health::Health};



#[derive(Default)]
pub struct ClientHealthUICache {

    cache : HashMap<Entity, ClientHealthUI>

}

struct ClientHealthUI {



}

pub fn health_update(
    mut updated_player_health_entities: Query<&Health, Changed<Health>>,
    mut client_health_ui_cache : Local<ClientHealthUICache>,
) {

    for health_component in updated_player_health_entities.iter_mut() {

        match &health_component.health_container {
            crate::space_core::components::health::HealthContainer::Humanoid(humanoid_health) => {

                let total_head_damage = humanoid_health.head_brute+humanoid_health.head_burn+humanoid_health.head_toxin;
                let total_torso_damage = humanoid_health.torso_brute+humanoid_health.torso_burn+humanoid_health.torso_toxin;
                let total_left_arm_damage = humanoid_health.left_arm_brute+humanoid_health.left_arm_burn+humanoid_health.left_arm_toxin;
                let total_right_arm_damage = humanoid_health.right_arm_brute+humanoid_health.right_arm_burn+humanoid_health.right_arm_toxin;
                let total_left_leg_damage = humanoid_health.left_leg_brute+humanoid_health.left_leg_burn+humanoid_health.left_leg_toxin;
                let total_right_leg_damage = humanoid_health.right_leg_brute+humanoid_health.right_leg_burn+humanoid_health.right_leg_toxin;

                if total_head_damage > 75. {

                } else if total_head_damage > 50. {

                } else if total_head_damage > 25. {

                } else {
                    
                }



            },
        }

        

    }

}
