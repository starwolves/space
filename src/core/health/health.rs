use std::collections::HashMap;

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

#[derive(Default)]
pub struct ClientHealthUICache {
    pub cache: HashMap<Entity, ClientHealthUI>,
}

pub struct ClientHealthUI {
    pub head_damage: UIDamageType,
    pub torso_damage: UIDamageType,
    pub left_arm_damage: UIDamageType,
    pub right_arm_damage: UIDamageType,
    pub left_leg_damage: UIDamageType,
    pub right_leg_damage: UIDamageType,
}

pub enum UIDamageType {
    None,
    Light,
    Moderate,
    Heavy,
}

pub fn health_ui_update(
    mut updated_player_health_entities: Query<(Entity, &ConnectedPlayer, &Health), Changed<Health>>,
    mut client_health_ui_cache: ResMut<ClientHealthUICache>,
    mut net_health_update: EventWriter<NetHealthUpdate>,
) {
    for (entity, connected_player_component, health_component) in
        updated_player_health_entities.iter_mut()
    {
        match &health_component.health_container {
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

use bevy::prelude::{Changed, Component, Entity, EventWriter, Query, Res, ResMut};
use rand::prelude::SliceRandom;

use crate::core::{
    chat::net::NetChatMessage,
    combat::attack::HitSoundSurface,
    connected_player::{connection::ConnectedPlayer, plugin::HandleToEntity},
    gridmap::gridmap::{to_doryen_coordinates, Vec3Int},
    networking::networking::{EntityUpdateData, EntityWorldType, ReliableServerMessage},
    senser::visible_checker::Senser,
};

use super::net::NetHealthUpdate;

#[derive(Component)]
pub struct Health {
    pub health_container: HealthContainer,
    pub health_flags: HashMap<u32, HealthFlag>,
    pub hit_sound_surface: HitSoundSurface,
    pub is_combat_obstacle: bool,
    pub is_laser_obstacle: bool,
    pub is_reach_obstacle: bool,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health_container: HealthContainer::Entity(EntityContainer::default()),
            health_flags: HashMap::new(),
            hit_sound_surface: HitSoundSurface::Soft,
            is_combat_obstacle: false,
            is_laser_obstacle: true,
            is_reach_obstacle: false,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum HealthFlag {
    ArmourPlated,
    HeadBruteDefence(f32),
    TorsoBruteDefence(f32),
}

pub enum HealthContainer {
    Humanoid(HumanoidHealth),
    Entity(EntityContainer),
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum DamageFlag {
    SoftDamage, //Ie fists.
    WeakLethalLaser,
    Stun(f32),
    Floor(f32),
}

#[derive(Debug, Default)]
pub struct HumanoidHealth {
    pub head_brute: f32,
    pub head_burn: f32,
    pub head_toxin: f32,

    pub torso_brute: f32,
    pub torso_burn: f32,
    pub torso_toxin: f32,

    pub left_arm_brute: f32,
    pub left_arm_burn: f32,
    pub left_arm_toxin: f32,

    pub right_arm_brute: f32,
    pub right_arm_burn: f32,
    pub right_arm_toxin: f32,

    pub right_leg_brute: f32,
    pub right_leg_burn: f32,
    pub right_leg_toxin: f32,

    pub left_leg_brute: f32,
    pub left_leg_burn: f32,
    pub left_leg_toxin: f32,
}

#[derive(Default)]
pub struct EntityContainer {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}

#[derive(Clone)]
pub struct DamageModel {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
    pub damage_flags: HashMap<u32, DamageFlag>,
}

impl Default for DamageModel {
    fn default() -> Self {
        Self {
            brute: 0.,
            burn: 0.,
            toxin: 0.,
            damage_flags: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
pub enum HitResult {
    HitSoft,
    Blocked,
    Missed,
}

pub fn calculate_damage(
    health_flags: &HashMap<u32, HealthFlag>,
    damage_flags: &HashMap<u32, DamageFlag>,

    brute: &f32,
    burn: &f32,
    toxin: &f32,
) -> (f32, f32, f32, HitResult) {
    let mut output_brute = brute.clone();
    let mut output_burn = burn.clone();
    let output_toxin = toxin.clone();

    let mut hit_result = HitResult::HitSoft;

    let mut damager_flags = vec![];

    for damage_flag in damage_flags.values() {
        damager_flags.push(damage_flag);
    }

    let mut structure_health_flags = vec![];

    for stucture_health_flag in health_flags.values() {
        structure_health_flags.push(stucture_health_flag);
    }

    let is_armour_plated = structure_health_flags.contains(&&HealthFlag::ArmourPlated);

    if damager_flags.contains(&&DamageFlag::SoftDamage) && is_armour_plated {
        output_brute = 0.;
        hit_result = HitResult::Blocked;
    } else if damager_flags.contains(&&DamageFlag::WeakLethalLaser) && is_armour_plated {
        output_burn *= 0.05;
        hit_result = HitResult::Blocked;
    }

    (output_brute, output_burn, output_toxin, hit_result)
}

pub enum DamageType {
    Melee,
    Projectile,
}

impl Health {
    pub fn apply_damage(
        &mut self,
        body_part: &str,
        damage_model: &DamageModel,
        net_new_chat_message_event: &mut EventWriter<NetChatMessage>,
        handle_to_entity: &Res<HandleToEntity>,
        attacker_cell_id: &Vec3Int,
        attacked_cell_id: &Vec3Int,
        sensers: &Query<(Entity, &Senser)>,
        attacker_name: &str,
        entity_name: &str,
        _damage_type: &DamageType,
        weapon_name: &str,
        weapon_a_name: &str,
        offense_words: &Vec<String>,
        trigger_words: &Vec<String>,
    ) -> HitResult {
        let (brute_damage, burn_damage, toxin_damage, hit_result) = calculate_damage(
            &self.health_flags,
            &damage_model.damage_flags,
            &damage_model.brute,
            &damage_model.burn,
            &damage_model.toxin,
        );

        let attacker_cell_id_doryen = to_doryen_coordinates(attacker_cell_id.x, attacker_cell_id.z);
        let attacked_cell_id_doryen = to_doryen_coordinates(attacked_cell_id.x, attacked_cell_id.z);

        match &mut self.health_container {
            HealthContainer::Humanoid(humanoid_health) => {
                for (entity, senser) in sensers.iter() {
                    let mut message = "".to_string();

                    let strike_word = offense_words.choose(&mut rand::thread_rng()).unwrap();

                    let attacker_is_visible;

                    if senser.fov.is_in_fov(
                        attacker_cell_id_doryen.0 as usize,
                        attacker_cell_id_doryen.1 as usize,
                    ) {
                        attacker_is_visible = true;
                    } else {
                        attacker_is_visible = false;
                    }

                    let attacked_is_visible;

                    if senser.fov.is_in_fov(
                        attacked_cell_id_doryen.0 as usize,
                        attacked_cell_id_doryen.1 as usize,
                    ) {
                        attacked_is_visible = true;
                    } else {
                        attacked_is_visible = false;
                    }

                    let mut send_message = false;

                    if attacker_is_visible && attacked_is_visible {
                        send_message = true;
                        if body_part == "head" {
                            humanoid_health.head_brute += brute_damage;
                            humanoid_health.head_burn += burn_damage;
                            humanoid_health.head_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the head with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "torso" {
                            humanoid_health.torso_brute += brute_damage;
                            humanoid_health.torso_burn += burn_damage;
                            humanoid_health.torso_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the torso with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "right_arm" {
                            humanoid_health.right_arm_brute += brute_damage;
                            humanoid_health.right_arm_burn += burn_damage;
                            humanoid_health.right_arm_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the right arm with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "left_arm" {
                            humanoid_health.left_arm_brute += brute_damage;
                            humanoid_health.left_arm_burn += burn_damage;
                            humanoid_health.left_arm_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the left arm with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "right_leg" {
                            humanoid_health.right_leg_brute += brute_damage;
                            humanoid_health.right_leg_burn += burn_damage;
                            humanoid_health.right_leg_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the right leg with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "left_leg" {
                            humanoid_health.left_leg_brute += brute_damage;
                            humanoid_health.left_leg_burn += burn_damage;
                            humanoid_health.left_leg_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the left leg with "
                                + weapon_a_name
                                + "![/color]";
                        }
                    } else if attacker_is_visible && !attacked_is_visible {
                        send_message = true;
                        let trigger_word = trigger_words.choose(&mut rand::thread_rng()).unwrap();
                        message = "[color=#ff003c]".to_string()
                            + attacker_name
                            + " has "
                            + trigger_word
                            + " his "
                            + weapon_name
                            + "![/color]";
                    } else if !attacker_is_visible && attacked_is_visible {
                        send_message = true;
                        if body_part == "head" {
                            humanoid_health.head_brute += brute_damage;
                            humanoid_health.head_burn += burn_damage;
                            humanoid_health.head_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the head with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "torso" {
                            humanoid_health.torso_brute += brute_damage;
                            humanoid_health.torso_burn += burn_damage;
                            humanoid_health.torso_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the torso with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "right_arm" {
                            humanoid_health.right_arm_brute += brute_damage;
                            humanoid_health.right_arm_burn += burn_damage;
                            humanoid_health.right_arm_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the right arm with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "left_arm" {
                            humanoid_health.left_arm_brute += brute_damage;
                            humanoid_health.left_arm_burn += burn_damage;
                            humanoid_health.left_arm_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the left arm with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "right_leg" {
                            humanoid_health.right_leg_brute += brute_damage;
                            humanoid_health.right_leg_burn += burn_damage;
                            humanoid_health.right_leg_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the right leg with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "left_leg" {
                            humanoid_health.left_leg_brute += brute_damage;
                            humanoid_health.left_leg_burn += burn_damage;
                            humanoid_health.left_leg_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the left leg with "
                                + weapon_a_name
                                + "![/color]";
                        }
                    }

                    if send_message {
                        match handle_to_entity.inv_map.get(&entity) {
                            Some(handle) => {
                                net_new_chat_message_event.send(NetChatMessage {
                                    handle: *handle,
                                    message: ReliableServerMessage::ChatMessage(message.clone()),
                                });
                            }
                            None => {}
                        }
                    }
                }
            }
            HealthContainer::Entity(item) => {
                item.brute += brute_damage;
                item.burn += burn_damage;
                item.toxin += toxin_damage;

                for (entity, senser) in sensers.iter() {
                    let mut message = "".to_string();

                    let strike_word = offense_words.choose(&mut rand::thread_rng()).unwrap();

                    let attacker_is_visible;

                    if senser.fov.is_in_fov(
                        attacker_cell_id_doryen.0 as usize,
                        attacker_cell_id_doryen.1 as usize,
                    ) {
                        attacker_is_visible = true;
                    } else {
                        attacker_is_visible = false;
                    }

                    let attacked_is_visible;

                    if senser.fov.is_in_fov(
                        attacked_cell_id_doryen.0 as usize,
                        attacked_cell_id_doryen.1 as usize,
                    ) {
                        attacked_is_visible = true;
                    } else {
                        attacked_is_visible = false;
                    }

                    let mut should_send = false;

                    if attacker_is_visible && attacked_is_visible {
                        message = "[color=#ff003c]".to_string()
                            + attacker_name
                            + " has "
                            + strike_word
                            + " "
                            + entity_name
                            + " with "
                            + weapon_a_name
                            + "![/color]";
                        should_send = true;
                    } else if attacker_is_visible && !attacked_is_visible {
                        let trigger_word = trigger_words.choose(&mut rand::thread_rng()).unwrap();
                        message = "[color=#ff003c]".to_string()
                            + attacker_name
                            + " has "
                            + trigger_word
                            + " his "
                            + weapon_a_name
                            + "![/color]";
                        should_send = true;
                    } else if !attacker_is_visible && attacked_is_visible {
                        message = "[color=#ff003c]".to_string()
                            + entity_name
                            + " has been "
                            + strike_word
                            + " with "
                            + weapon_a_name
                            + "![/color]";
                        should_send = true;
                    }

                    if should_send {
                        match handle_to_entity.inv_map.get(&entity) {
                            Some(handle) => {
                                net_new_chat_message_event.send(NetChatMessage {
                                    handle: *handle,
                                    message: ReliableServerMessage::ChatMessage(message.clone()),
                                });
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        hit_result
    }
}
