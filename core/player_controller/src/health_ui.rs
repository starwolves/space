use std::collections::HashMap;

use api::entity_updates::EntityUpdateData;
use bevy::prelude::{Changed, Entity, EventWriter, Query, ResMut};
use health::core::{HealthComponent, HealthContainer};
use networking::messages::{EntityWorldType, NetHealthUpdate, ReliableServerMessage};
use server::core::ConnectedPlayer;

const UI_ALPHA: f32 = 146.;
const NONE_UI_RED: f32 = 102.;
const NONE_UI_GREEN: f32 = 165.;
const NONE_UI_BLUE: f32 = 255.;

const LIGHT_UI_RED: f32 = 186.;
const LIGHT_UI_GREEN: f32 = 255.;
const LIGHT_UI_BLUE: f32 = 0.;

const MODERATE_UI_RED: f32 = 255.;
const MODERATE_UI_GREEN: f32 = 175.;
const MODERATE_UI_BLUE: f32 = 0.;

const HEAVY_UI_RED: f32 = 255.;
const HEAVY_UI_GREEN: f32 = 0.;
const HEAVY_UI_BLUE: f32 = 60.;

/// Resource with all client health UI caches.
#[derive(Default)]
pub struct ClientHealthUICache {
    pub cache: HashMap<Entity, ClientHealthUI>,
}
/// Client health UI cache.
pub struct ClientHealthUI {
    pub head_damage: UIDamageType,
    pub torso_damage: UIDamageType,
    pub left_arm_damage: UIDamageType,
    pub right_arm_damage: UIDamageType,
    pub left_leg_damage: UIDamageType,
    pub right_leg_damage: UIDamageType,
}
/// All UI damage types.
pub enum UIDamageType {
    None,
    Light,
    Moderate,
    Heavy,
}

/// Manage sending UI health updates to Godot client.
pub(crate) fn health_ui_update(
    mut updated_player_health_entities: Query<
        (Entity, &ConnectedPlayer, &HealthComponent),
        Changed<HealthComponent>,
    >,
    mut client_health_ui_cache: ResMut<ClientHealthUICache>,
    mut net_health_update: EventWriter<NetHealthUpdate>,
) {
    for (entity, connected_player_component, health_component) in
        updated_player_health_entities.iter_mut()
    {
        match &health_component.health.health_container {
            HealthContainer::Humanoid(humanoid_health) => {
                let total_head_damage = humanoid_health.head_brute
                    + humanoid_health.head_burn
                    + humanoid_health.head_toxin;
                let total_torso_damage = humanoid_health.torso_brute
                    + humanoid_health.torso_burn
                    + humanoid_health.torso_toxin;
                let total_left_arm_damage = humanoid_health.left_arm_brute
                    + humanoid_health.left_arm_burn
                    + humanoid_health.left_arm_toxin;
                let total_right_arm_damage = humanoid_health.right_arm_brute
                    + humanoid_health.right_arm_burn
                    + humanoid_health.right_arm_toxin;
                let total_left_leg_damage = humanoid_health.left_leg_brute
                    + humanoid_health.left_leg_burn
                    + humanoid_health.left_leg_toxin;
                let total_right_leg_damage = humanoid_health.right_leg_brute
                    + humanoid_health.right_leg_burn
                    + humanoid_health.right_leg_toxin;

                let mut client_health_ui_option = None;

                match client_health_ui_cache.cache.get_mut(&entity) {
                    Some(cached_ui) => {
                        client_health_ui_option = Some(cached_ui);
                    }
                    None => {}
                }

                if matches!(client_health_ui_option, None) {
                    client_health_ui_cache.cache.insert(
                        entity,
                        ClientHealthUI {
                            head_damage: UIDamageType::None,
                            torso_damage: UIDamageType::None,
                            left_arm_damage: UIDamageType::None,
                            right_arm_damage: UIDamageType::None,
                            left_leg_damage: UIDamageType::None,
                            right_leg_damage: UIDamageType::None,
                        },
                    );
                    client_health_ui_option =
                        Some(client_health_ui_cache.cache.get_mut(&entity).unwrap());
                }

                let client_health_ui = client_health_ui_option.unwrap();

                let mut entity_updates_map = HashMap::new();
                entity_updates_map.insert(".".to_string(), HashMap::new());

                let mut new_update = false;

                // Head
                if total_head_damage > 75. {
                    if !matches!(client_health_ui.head_damage, UIDamageType::Heavy) {
                        client_health_ui.head_damage = UIDamageType::Heavy;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                HEAVY_UI_RED / 255.,
                                HEAVY_UI_GREEN / 255.,
                                HEAVY_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("head".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_head_damage > 50. {
                    if !matches!(client_health_ui.head_damage, UIDamageType::Moderate) {
                        client_health_ui.head_damage = UIDamageType::Moderate;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                MODERATE_UI_RED / 255.,
                                MODERATE_UI_GREEN / 255.,
                                MODERATE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("head".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_head_damage > 25. {
                    if !matches!(client_health_ui.head_damage, UIDamageType::Light) {
                        client_health_ui.head_damage = UIDamageType::Light;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                LIGHT_UI_RED / 255.,
                                LIGHT_UI_GREEN / 255.,
                                LIGHT_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("head".to_string(), head_data);
                        new_update = true;
                    }
                } else {
                    if !matches!(client_health_ui.head_damage, UIDamageType::None) {
                        client_health_ui.head_damage = UIDamageType::None;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                NONE_UI_RED / 255.,
                                NONE_UI_GREEN / 255.,
                                NONE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("head".to_string(), head_data);
                        new_update = true;
                    }
                }

                // Torso
                if total_torso_damage > 75. {
                    if !matches!(client_health_ui.torso_damage, UIDamageType::Heavy) {
                        client_health_ui.torso_damage = UIDamageType::Heavy;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                HEAVY_UI_RED / 255.,
                                HEAVY_UI_GREEN / 255.,
                                HEAVY_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("torso".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_torso_damage > 50. {
                    if !matches!(client_health_ui.torso_damage, UIDamageType::Moderate) {
                        client_health_ui.torso_damage = UIDamageType::Moderate;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                MODERATE_UI_RED / 255.,
                                MODERATE_UI_GREEN / 255.,
                                MODERATE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("torso".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_torso_damage > 25. {
                    if !matches!(client_health_ui.torso_damage, UIDamageType::Light) {
                        client_health_ui.torso_damage = UIDamageType::Light;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                LIGHT_UI_RED / 255.,
                                LIGHT_UI_GREEN / 255.,
                                LIGHT_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("torso".to_string(), head_data);
                        new_update = true;
                    }
                } else {
                    if !matches!(client_health_ui.torso_damage, UIDamageType::None) {
                        client_health_ui.torso_damage = UIDamageType::None;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                NONE_UI_RED / 255.,
                                NONE_UI_GREEN / 255.,
                                NONE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("torso".to_string(), head_data);
                        new_update = true;
                    }
                }

                // LeftArm
                if total_left_arm_damage > 75. {
                    if !matches!(client_health_ui.left_arm_damage, UIDamageType::Heavy) {
                        client_health_ui.left_arm_damage = UIDamageType::Heavy;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                HEAVY_UI_RED / 255.,
                                HEAVY_UI_GREEN / 255.,
                                HEAVY_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("leftArm".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_left_arm_damage > 50. {
                    if !matches!(client_health_ui.left_arm_damage, UIDamageType::Moderate) {
                        client_health_ui.left_arm_damage = UIDamageType::Moderate;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                MODERATE_UI_RED / 255.,
                                MODERATE_UI_GREEN / 255.,
                                MODERATE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("leftArm".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_left_arm_damage > 25. {
                    if !matches!(client_health_ui.left_arm_damage, UIDamageType::Light) {
                        client_health_ui.left_arm_damage = UIDamageType::Light;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                LIGHT_UI_RED / 255.,
                                LIGHT_UI_GREEN / 255.,
                                LIGHT_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("leftArm".to_string(), head_data);
                        new_update = true;
                    }
                } else {
                    if !matches!(client_health_ui.left_arm_damage, UIDamageType::None) {
                        client_health_ui.left_arm_damage = UIDamageType::None;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                NONE_UI_RED / 255.,
                                NONE_UI_GREEN / 255.,
                                NONE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("leftArm".to_string(), head_data);
                        new_update = true;
                    }
                }

                // RightArm
                if total_right_arm_damage > 75. {
                    if !matches!(client_health_ui.right_arm_damage, UIDamageType::Heavy) {
                        client_health_ui.right_arm_damage = UIDamageType::Heavy;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                HEAVY_UI_RED / 255.,
                                HEAVY_UI_GREEN / 255.,
                                HEAVY_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("rightArm".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_right_arm_damage > 50. {
                    if !matches!(client_health_ui.right_arm_damage, UIDamageType::Moderate) {
                        client_health_ui.right_arm_damage = UIDamageType::Moderate;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                MODERATE_UI_RED / 255.,
                                MODERATE_UI_GREEN / 255.,
                                MODERATE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("rightArm".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_right_arm_damage > 25. {
                    if !matches!(client_health_ui.right_arm_damage, UIDamageType::Light) {
                        client_health_ui.right_arm_damage = UIDamageType::Light;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                LIGHT_UI_RED / 255.,
                                LIGHT_UI_GREEN / 255.,
                                LIGHT_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("rightArm".to_string(), head_data);
                        new_update = true;
                    }
                } else {
                    if !matches!(client_health_ui.right_arm_damage, UIDamageType::None) {
                        client_health_ui.right_arm_damage = UIDamageType::None;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                NONE_UI_RED / 255.,
                                NONE_UI_GREEN / 255.,
                                NONE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("rightArm".to_string(), head_data);
                        new_update = true;
                    }
                }

                // LeftLeg
                if total_left_leg_damage > 75. {
                    if !matches!(client_health_ui.left_leg_damage, UIDamageType::Heavy) {
                        client_health_ui.left_leg_damage = UIDamageType::Heavy;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                HEAVY_UI_RED / 255.,
                                HEAVY_UI_GREEN / 255.,
                                HEAVY_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("leftLeg".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_left_leg_damage > 50. {
                    if !matches!(client_health_ui.left_leg_damage, UIDamageType::Moderate) {
                        client_health_ui.left_leg_damage = UIDamageType::Moderate;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                MODERATE_UI_RED / 255.,
                                MODERATE_UI_GREEN / 255.,
                                MODERATE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("leftLeg".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_left_leg_damage > 25. {
                    if !matches!(client_health_ui.left_leg_damage, UIDamageType::Light) {
                        client_health_ui.left_leg_damage = UIDamageType::Light;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                LIGHT_UI_RED / 255.,
                                LIGHT_UI_GREEN / 255.,
                                LIGHT_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("leftLeg".to_string(), head_data);
                        new_update = true;
                    }
                } else {
                    if !matches!(client_health_ui.left_leg_damage, UIDamageType::None) {
                        client_health_ui.left_leg_damage = UIDamageType::None;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                NONE_UI_RED / 255.,
                                NONE_UI_GREEN / 255.,
                                NONE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("leftLeg".to_string(), head_data);
                        new_update = true;
                    }
                }

                // RightLeg
                if total_right_leg_damage > 75. {
                    if !matches!(client_health_ui.right_leg_damage, UIDamageType::Heavy) {
                        client_health_ui.right_leg_damage = UIDamageType::Heavy;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                HEAVY_UI_RED / 255.,
                                HEAVY_UI_GREEN / 255.,
                                HEAVY_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("rightLeg".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_right_leg_damage > 50. {
                    if !matches!(client_health_ui.right_leg_damage, UIDamageType::Moderate) {
                        client_health_ui.right_leg_damage = UIDamageType::Moderate;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                MODERATE_UI_RED / 255.,
                                MODERATE_UI_GREEN / 255.,
                                MODERATE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("rightLeg".to_string(), head_data);
                        new_update = true;
                    }
                } else if total_right_leg_damage > 25. {
                    if !matches!(client_health_ui.right_leg_damage, UIDamageType::Light) {
                        client_health_ui.right_leg_damage = UIDamageType::Light;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                LIGHT_UI_RED / 255.,
                                LIGHT_UI_GREEN / 255.,
                                LIGHT_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("rightLeg".to_string(), head_data);
                        new_update = true;
                    }
                } else {
                    if !matches!(client_health_ui.right_leg_damage, UIDamageType::None) {
                        client_health_ui.right_leg_damage = UIDamageType::None;
                        let mut head_data = HashMap::new();
                        head_data.insert(
                            "control_color".to_string(),
                            EntityUpdateData::Color(
                                NONE_UI_RED / 255.,
                                NONE_UI_GREEN / 255.,
                                NONE_UI_BLUE / 255.,
                                UI_ALPHA / 255.,
                            ),
                        );
                        entity_updates_map.insert("rightLeg".to_string(), head_data);
                        new_update = true;
                    }
                }

                if new_update && connected_player_component.connected {
                    net_health_update.send(NetHealthUpdate {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::EntityUpdate(
                            entity.to_bits(),
                            entity_updates_map,
                            false,
                            EntityWorldType::HealthUI,
                        ),
                    });
                }
            }
            _ => (),
        }
    }
}
