use api::{
    combat::{
        get_default_fists_words, get_default_trigger_fists_words, get_default_trigger_melee_words,
        get_default_trigger_weapon_words, MeleeCombat, ProjectileCombat,
    },
    data::HandleToEntity,
    examinable::Examinable,
    gridmap::{to_doryen_coordinates, world_to_cell_id, GridmapData, GridmapMain},
    health::{HealthComponent, HealthContainer},
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    senser::Senser,
};
use bevy::prelude::{warn, Component, Entity, EventReader, EventWriter, Query, Res, Transform};
use chat::chat::EntityProximityMessage;
use inventory_item::item::InventoryItem;
use rand::prelude::SliceRandom;

use crate::{
    active_attacks::ActiveAttacks, attack::QueryCombatHitResult, melee_queries::MeleeBlank,
    projectile_queries::ProjectileBlank,
};

pub struct NetHitQueryChat {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetHitQueryChat {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Chat hooks for entities that got hit by something.
pub fn attacked_by_chat<T: Component>(
    mut query_hit_results: EventReader<QueryCombatHitResult>,
    sensers: Query<(Entity, &Senser)>,
    attackers: Query<(&Transform, &Examinable)>,
    cached_attacks: Res<ActiveAttacks>,
    handle_to_entity: Res<HandleToEntity>,
    inventory_items_query: Query<(
        &InventoryItem,
        &Examinable,
        &MeleeCombat,
        Option<&ProjectileCombat>,
    )>,
    mut net: EventWriter<NetHitQueryChat>,
    target_entities: Query<(&HealthComponent, &Examinable, &Transform)>,
    attacker_criteria: Query<&T>,
) {
    for query_hit_result in query_hit_results.iter() {
        let attack_cache;

        match cached_attacks.map.get(&query_hit_result.incremented_id) {
            Some(cache) => {
                attack_cache = cache;
            }
            None => {
                warn!(
                    "Cell attack text couldnt find attack cache! {}",
                    query_hit_result.incremented_id
                );
                continue;
            }
        }

        match attacker_criteria.get(attack_cache.attack.attacker) {
            Ok(_) => {}
            Err(_) => {
                continue;
            }
        }

        let melee;
        match attack_cache.melee {
            Some(n) => {
                melee = n;
            }
            None => {
                warn!("attack cache meelee not yet setup.");
                continue;
            }
        }

        let attacker_transform;
        let attacker_examinable_component;

        match attackers.get(attack_cache.attack.attacker) {
            Ok((t, e)) => {
                attacker_transform = t;
                attacker_examinable_component = e;
            }
            Err(_rr) => {
                warn!("Cell attack text couldnt find attacker entity.");
                continue;
            }
        }

        let mut weapon_name = "his fists".to_string();
        let mut weapon_a_name = "his fists".to_string();

        let offense_words;
        let trigger_words;

        match attack_cache.attack.weapon_option {
            Some(weapon_entity) => match inventory_items_query.get(weapon_entity) {
                Ok((
                    _inventory_item_component,
                    examinable_component,
                    melee_combat_component,
                    projectile_combat_component,
                )) => {
                    weapon_a_name = examinable_component.name.get_a_name().clone();
                    weapon_name = examinable_component.name.get_name().to_owned();
                    match melee {
                        true => {
                            offense_words = melee_combat_component.combat_melee_text_set.clone();
                            trigger_words = melee_combat_component.trigger_melee_text_set.clone();
                        }
                        false => {
                            offense_words = projectile_combat_component
                                .unwrap()
                                .combat_projectile_text_set
                                .clone();
                            trigger_words = projectile_combat_component
                                .unwrap()
                                .trigger_projectile_text_set
                                .clone();
                        }
                    }
                }
                Err(_rr) => {
                    warn!("Attack cell text couldnt find weapon entity!");
                    continue;
                }
            },
            None => {
                offense_words = get_default_fists_words();
                trigger_words = get_default_trigger_fists_words();
            }
        }

        let attacker_cell_id = world_to_cell_id(attacker_transform.translation);

        let attacker_cell_id_doryen = to_doryen_coordinates(attacker_cell_id.x, attacker_cell_id.z);

        for attacked_entity_hit in query_hit_result.entities_hits.iter() {
            let health_component;
            let examinable_component;
            let target_transform;

            match target_entities.get(attacked_entity_hit.entity) {
                Ok((h, e, t)) => {
                    health_component = h;
                    examinable_component = e;
                    target_transform = t;
                }
                Err(_rr) => {
                    warn!("Couldnt find health component of hit entity.");
                    continue;
                }
            }

            let attacked_gridmap_coords = world_to_cell_id(target_transform.translation);
            let attacked_cell_id_doryen =
                to_doryen_coordinates(attacked_gridmap_coords.x, attacked_gridmap_coords.z);

            match &health_component.health.health_container {
                HealthContainer::Humanoid(_) => {
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
                            if attack_cache.attack.targetted_limb == "head" {
                                message = "[color=#ff003c]".to_string()
                                    + attacker_examinable_component.name.get_name()
                                    + " has "
                                    + strike_word
                                    + " "
                                    + &examinable_component.name.get_name()
                                    + " in the head with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "torso" {
                                message = "[color=#ff003c]".to_string()
                                    + attacker_examinable_component.name.get_name()
                                    + " has "
                                    + strike_word
                                    + " "
                                    + examinable_component.name.get_name()
                                    + " in the torso with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "right_arm" {
                                message = "[color=#ff003c]".to_string()
                                    + attacker_examinable_component.name.get_name()
                                    + " has "
                                    + strike_word
                                    + " "
                                    + examinable_component.name.get_name()
                                    + " in the right arm with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "left_arm" {
                                message = "[color=#ff003c]".to_string()
                                    + attacker_examinable_component.name.get_name()
                                    + " has "
                                    + strike_word
                                    + " "
                                    + examinable_component.name.get_name()
                                    + " in the left arm with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "right_leg" {
                                message = "[color=#ff003c]".to_string()
                                    + attacker_examinable_component.name.get_name()
                                    + " has "
                                    + strike_word
                                    + " "
                                    + examinable_component.name.get_name()
                                    + " in the right leg with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "left_leg" {
                                message = "[color=#ff003c]".to_string()
                                    + attacker_examinable_component.name.get_name()
                                    + " has "
                                    + strike_word
                                    + " "
                                    + examinable_component.name.get_name()
                                    + " in the left leg with "
                                    + &weapon_a_name
                                    + "![/color]";
                            }
                        } else if attacker_is_visible && !attacked_is_visible {
                            send_message = true;
                            let trigger_word =
                                trigger_words.choose(&mut rand::thread_rng()).unwrap();
                            message = "[color=#ff003c]".to_string()
                                + attacker_examinable_component.name.get_name()
                                + " has "
                                + trigger_word
                                + " his "
                                + &weapon_name
                                + "![/color]";
                        } else if !attacker_is_visible && attacked_is_visible {
                            send_message = true;
                            if attack_cache.attack.targetted_limb == "head" {
                                message = "[color=#ff003c]".to_string()
                                    + examinable_component.name.get_name()
                                    + " has been "
                                    + strike_word
                                    + " in the head with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "torso" {
                                message = "[color=#ff003c]".to_string()
                                    + examinable_component.name.get_name()
                                    + " has been "
                                    + strike_word
                                    + " in the torso with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "right_arm" {
                                message = "[color=#ff003c]".to_string()
                                    + examinable_component.name.get_name()
                                    + " has been "
                                    + strike_word
                                    + " in the right arm with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "left_arm" {
                                message = "[color=#ff003c]".to_string()
                                    + examinable_component.name.get_name()
                                    + " has been "
                                    + strike_word
                                    + " in the left arm with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "right_leg" {
                                message = "[color=#ff003c]".to_string()
                                    + examinable_component.name.get_name()
                                    + " has been "
                                    + strike_word
                                    + " in the right leg with "
                                    + &weapon_a_name
                                    + "![/color]";
                            } else if attack_cache.attack.targetted_limb == "left_leg" {
                                message = "[color=#ff003c]".to_string()
                                    + examinable_component.name.get_name()
                                    + " has been "
                                    + strike_word
                                    + " in the left leg with "
                                    + &weapon_a_name
                                    + "![/color]";
                            }
                        }

                        if send_message {
                            match handle_to_entity.inv_map.get(&entity) {
                                Some(handle) => {
                                    net.send(NetHitQueryChat {
                                        handle: *handle,
                                        message: ReliableServerMessage::ChatMessage(
                                            message.clone(),
                                        ),
                                    });
                                }
                                None => {}
                            }
                        }
                    }
                }
                HealthContainer::Entity(_) => {
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
                                + attacker_examinable_component.name.get_name()
                                + " has "
                                + strike_word
                                + " "
                                + &examinable_component.name.get_a_name()
                                + " with "
                                + &weapon_a_name
                                + "![/color]";
                            should_send = true;
                        } else if attacker_is_visible && !attacked_is_visible {
                            let trigger_word =
                                trigger_words.choose(&mut rand::thread_rng()).unwrap();
                            message = "[color=#ff003c]".to_string()
                                + attacker_examinable_component.name.get_name()
                                + " has "
                                + trigger_word
                                + " his "
                                + &weapon_a_name
                                + "![/color]";
                            should_send = true;
                        } else if !attacker_is_visible && attacked_is_visible {
                            message = "[color=#ff003c]".to_string()
                                + examinable_component.name.get_name()
                                + " has been "
                                + strike_word
                                + " with "
                                + &weapon_a_name
                                + "![/color]";
                            should_send = true;
                        }

                        if should_send {
                            match handle_to_entity.inv_map.get(&entity) {
                                Some(handle) => {
                                    net.send(NetHitQueryChat {
                                        handle: *handle,
                                        message: ReliableServerMessage::ChatMessage(
                                            message.clone(),
                                        ),
                                    });
                                }
                                None => {}
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

/// Chat hooks for when ship cells are hit.
pub fn hit_query_chat_cells(
    mut query_hit_results: EventReader<QueryCombatHitResult>,
    sensers: Query<(Entity, &Senser)>,
    attackers: Query<(&Transform, &Examinable)>,
    cached_attacks: Res<ActiveAttacks>,
    handle_to_entity: Res<HandleToEntity>,
    inventory_items_query: Query<(
        &InventoryItem,
        &Examinable,
        &MeleeCombat,
        Option<&ProjectileCombat>,
    )>,
    gridmap_main: Res<GridmapMain>,
    gridmap_data: Res<GridmapData>,
    mut net: EventWriter<NetHitQueryChat>,
) {
    for query_hit_result in query_hit_results.iter() {
        let attack_cache;

        match cached_attacks.map.get(&query_hit_result.incremented_id) {
            Some(cache) => {
                attack_cache = cache;
            }
            None => {
                warn!(
                    "Cell attack text couldnt find attack cache! {}",
                    query_hit_result.incremented_id
                );
                continue;
            }
        }

        let melee;
        match attack_cache.melee {
            Some(n) => {
                melee = n;
            }
            None => {
                warn!("attack cache meelee not yet setup.");
                continue;
            }
        }

        let attacker_transform;
        let attacker_examinable_component;

        match attackers.get(attack_cache.attack.attacker) {
            Ok((t, e)) => {
                attacker_transform = t;
                attacker_examinable_component = e;
            }
            Err(_rr) => {
                warn!("Cell attack text couldnt find attacker entity.");
                continue;
            }
        }

        let mut weapon_name = "his fists".to_string();
        let mut weapon_a_name = "his fists".to_string();

        let offense_words;
        let trigger_words;

        match attack_cache.attack.weapon_option {
            Some(weapon_entity) => match inventory_items_query.get(weapon_entity) {
                Ok((
                    _inventory_item_component,
                    examinable_component,
                    melee_combat_component,
                    projectile_combat_component,
                )) => {
                    weapon_a_name = examinable_component.name.get_a_name().clone();
                    weapon_name = examinable_component.name.get_name().to_owned();
                    match melee {
                        true => {
                            offense_words = melee_combat_component.combat_melee_text_set.clone();
                            trigger_words = melee_combat_component.trigger_melee_text_set.clone();
                        }
                        false => {
                            offense_words = projectile_combat_component
                                .unwrap()
                                .combat_projectile_text_set
                                .clone();
                            trigger_words = projectile_combat_component
                                .unwrap()
                                .trigger_projectile_text_set
                                .clone();
                        }
                    }
                }
                Err(_rr) => {
                    warn!("Attack cell text couldnt find weapon entity!");
                    continue;
                }
            },
            None => {
                offense_words = get_default_fists_words();
                trigger_words = get_default_trigger_fists_words();
            }
        }

        let attacker_cell_id = world_to_cell_id(attacker_transform.translation);

        let attacker_cell_id_doryen = to_doryen_coordinates(attacker_cell_id.x, attacker_cell_id.z);

        for attacked_cell_id in query_hit_result.cell_hits.iter() {
            let attacked_cell_id_doryen =
                to_doryen_coordinates(attacked_cell_id.cell.x, attacked_cell_id.cell.z);

            let cell_data;

            match gridmap_main.grid_data.get(&attacked_cell_id.cell) {
                Some(c) => {
                    cell_data = c;
                }
                None => {
                    warn!("Cell attack hit no cell!");
                    continue;
                }
            }

            let cell_name = gridmap_data
                .main_text_names
                .get(&cell_data.item)
                .unwrap()
                .clone();

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
                        + attacker_examinable_component.name.get_name()
                        + " has "
                        + strike_word
                        + " "
                        + &cell_name.get_a_name()
                        + " with "
                        + &weapon_a_name
                        + "![/color]";
                    should_send = true;
                } else if attacker_is_visible && !attacked_is_visible {
                    let trigger_word = trigger_words.choose(&mut rand::thread_rng()).unwrap();
                    message = "[color=#ff003c]".to_string()
                        + attacker_examinable_component.name.get_name()
                        + " has "
                        + trigger_word
                        + " his "
                        + &weapon_name
                        + "![/color]";
                    should_send = true;
                } else if !attacker_is_visible && attacked_is_visible {
                    message = "[color=#ff003c]".to_string()
                        + &cell_name.get_a_name()
                        + " has been "
                        + strike_word
                        + " with "
                        + &weapon_a_name
                        + "![/color]";
                    should_send = true;
                }

                if should_send {
                    match handle_to_entity.inv_map.get(&entity) {
                        Some(handle) => {
                            net.send(NetHitQueryChat {
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
}

/// Chat hooks for blanks, when nothing was hit.
pub(crate) fn blanks_chat(
    mut projectile_blanks: EventReader<ProjectileBlank>,
    active_attacks: Res<ActiveAttacks>,
    attackers: Query<&Examinable>,
    inventory_items_query: Query<(&InventoryItem, &MeleeCombat, &Examinable)>,
    mut melee_blanks: EventReader<MeleeBlank>,
    mut entity_proximity_messages: EventWriter<EntityProximityMessage>,
) {
    for melee_blank in melee_blanks.iter() {
        let active_attack;

        match active_attacks.map.get(&melee_blank.incremented_id) {
            Some(a) => {
                active_attack = a;
            }
            None => {
                warn!("Couldnt find active attack for melee blank.");
                continue;
            }
        }

        let attacker_examinable;

        match attackers.get(active_attack.attack.attacker) {
            Ok(e) => {
                attacker_examinable = e;
            }
            Err(_rr) => {
                warn!("Couldnt find attacker components for melee blank.");
                continue;
            }
        }

        let mut weapon_a_name = "his fists".to_string();

        match active_attack.attack.weapon_option {
            Some(weapon_entity) => match inventory_items_query.get(weapon_entity) {
                Ok((_i, _m, e)) => {
                    weapon_a_name = e.name.get_a_name();
                }
                Err(_rr) => {
                    warn!("Couldnt find weapon entity for blank projectile!");
                    continue;
                }
            },
            None => {}
        }

        let ra = get_default_trigger_melee_words();
        let trigger_word = ra.choose(&mut rand::thread_rng()).unwrap();

        entity_proximity_messages.send(EntityProximityMessage {
            entities: vec![active_attack.attack.attacker],
            message: "[color=#ff003c]".to_string()
                + attacker_examinable.name.get_name()
                + " has "
                + trigger_word
                + " "
                + &weapon_a_name
                + "![/color]",
        });
    }

    for projectile_blank in projectile_blanks.iter() {
        let active_attack;

        match active_attacks.map.get(&projectile_blank.incremented_id) {
            Some(a) => {
                active_attack = a;
            }
            None => {
                warn!("Couldnt find belonging active attack of projectile blank.");
                continue;
            }
        }

        let attacker_examinable;

        match attackers.get(active_attack.attack.attacker) {
            Ok(examinable) => {
                attacker_examinable = examinable;
            }
            Err(_rr) => {
                warn!("Couldnt find attacker components of projectile blank.");
                continue;
            }
        }

        let weapon_entity;

        match active_attack.attack.weapon_option {
            Some(e) => {
                weapon_entity = e;
            }
            None => {
                warn!("Projectile blank but had no weapon.");
                continue;
            }
        }

        let weapon_examinable;
        let _weapon_inventory_item;

        match inventory_items_query.get(weapon_entity) {
            Ok((i, _m, e)) => {
                weapon_examinable = e;
                _weapon_inventory_item = i;
            }
            Err(_rr) => {
                warn!("Couldnt find weapon entity for blank projectile!");
                continue;
            }
        }

        let ra = get_default_trigger_weapon_words();
        let trigger_word = ra.choose(&mut rand::thread_rng()).unwrap();

        entity_proximity_messages.send(EntityProximityMessage {
            entities: vec![active_attack.attack.attacker],
            message: "[color=#ff003c]".to_string()
                + attacker_examinable.name.get_name()
                + " has "
                + trigger_word
                + " his "
                + &weapon_examinable.name.get_name()
                + "![/color]",
        });
    }
}
