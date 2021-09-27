use std::collections::HashMap;

use bevy::prelude::{Changed, Entity, EventWriter, Local, Query};

use crate::space_core::{components::{connected_player::ConnectedPlayer, health::Health}, events::net::net_health_update::NetHealthUpdate, resources::network_messages::ReliableServerMessage};



#[derive(Default)]
pub struct ClientHealthUICache {

    cache : HashMap<Entity, ClientHealthUI>

}

struct ClientHealthUI {

    head_damage : DamageType,
    torso_damage : DamageType,
    left_arm : DamageType,
    right_arm : DamageType,
    left_leg : DamageType,
    right_leg : DamageType,

}

enum DamageType {
    None,
    Light,
    Moderate,
    Heavy,
}

pub fn health_ui_update(
    mut updated_player_health_entities: Query<(Entity, &ConnectedPlayer, &Health), Changed<Health>>,
    mut client_health_ui_cache : Local<ClientHealthUICache>,
    mut net_health_update : EventWriter<NetHealthUpdate>,
) {

    for (
        entity,
        connected_player_component,
        health_component
    ) in updated_player_health_entities.iter_mut() {

        match &health_component.health_container {
            crate::space_core::components::health::HealthContainer::Humanoid(humanoid_health) => {

                let total_head_damage = humanoid_health.head_brute+humanoid_health.head_burn+humanoid_health.head_toxin;
                let total_torso_damage = humanoid_health.torso_brute+humanoid_health.torso_burn+humanoid_health.torso_toxin;
                let total_left_arm_damage = humanoid_health.left_arm_brute+humanoid_health.left_arm_burn+humanoid_health.left_arm_toxin;
                let total_right_arm_damage = humanoid_health.right_arm_brute+humanoid_health.right_arm_burn+humanoid_health.right_arm_toxin;
                let total_left_leg_damage = humanoid_health.left_leg_brute+humanoid_health.left_leg_burn+humanoid_health.left_leg_toxin;
                let total_right_leg_damage = humanoid_health.right_leg_brute+humanoid_health.right_leg_burn+humanoid_health.right_leg_toxin;

                let mut client_health_ui_option = None;

                match client_health_ui_cache.cache.get_mut(&entity) {
                    Some(cached_ui) => {
                        client_health_ui_option = Some(cached_ui);
                    },
                    None => {},
                }

                if matches!(client_health_ui_option, None) {
                    client_health_ui_cache.cache.insert(entity, ClientHealthUI {
                        head_damage: DamageType::None,
                        torso_damage: DamageType::None,
                        left_arm: DamageType::None,
                        right_arm: DamageType::None,
                        left_leg: DamageType::None,
                        right_leg: DamageType::None,
                    });
                    client_health_ui_option = Some(client_health_ui_cache.cache.get_mut(&entity).unwrap());
                }

                let client_health_ui = client_health_ui_option.unwrap();









                if total_head_damage > 75. {

                    if !matches!(client_health_ui.head_damage, DamageType::Heavy) {

                        let mut entity_updates_map = HashMap::new();
                        entity_updates_map.insert(".".to_string(), HashMap::new());
                        
                        // Insert entity_updates_map update with relative path to head textureRect and set color parameter on it, don't think we have this parameter added yet.

                        net_health_update.send(NetHealthUpdate {
                            handle: connected_player_component.handle,
                            message: ReliableServerMessage::EntityUpdate(0, entity_updates_map, false, "healthUI".to_string())
                        });

                    }

                } else if total_head_damage > 50. {

                } else if total_head_damage > 25. {

                } else {
                    
                }










            },
        }

        

    }

}
