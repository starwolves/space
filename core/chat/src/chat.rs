use std::collections::HashMap;

use bevy::{
    math::Vec3,
    prelude::{error, warn, Component, Entity, EventReader, EventWriter, Query, Res, Transform},
};

use api::gridmap::world_to_cell_id;
use api::{
    chat::{
        escape_bb, BILLBOARD_DATA_SECURITY_END, BILLBOARD_DATA_SECURITY_START,
        BILLBOARD_SHOUT_FONT, BILLBOARD_SHOUT_ITALIC_FONT, FAR_BOLD_FONT, FAR_SHOUT_FONT,
        FURTHER_BOLD_FONT, FURTHER_SHOUT_FONT, JOB_CONTROL_WORD, JOB_SECURITY_WORD,
        NEARBY_BOLD_FONT, NEARBY_SHOUT_FONT, SHOUT_DATA_MACHINE_FAR_I_END,
        SHOUT_DATA_MACHINE_FAR_I_START, SHOUT_DATA_MACHINE_FURTHER_I_END,
        SHOUT_DATA_MACHINE_FURTHER_I_START, SHOUT_DATA_MACHINE_NEARBY_I_END,
        SHOUT_DATA_MACHINE_NEARBY_I_START, SHOUT_DATA_STANDARD_FAR_I_END,
        SHOUT_DATA_STANDARD_FAR_I_START, SHOUT_DATA_STANDARD_FURTHER_I_END,
        SHOUT_DATA_STANDARD_FURTHER_I_START, SHOUT_DATA_STANDARD_NEARBY_I_END,
        SHOUT_DATA_STANDARD_NEARBY_I_START, TALK_DATA_MACHINE_B_FAR_END,
        TALK_DATA_MACHINE_B_FAR_START, TALK_DATA_MACHINE_B_FURTHER_END,
        TALK_DATA_MACHINE_B_FURTHER_START, TALK_DATA_MACHINE_B_NEARBY_END,
        TALK_DATA_MACHINE_B_NEARBY_START, TALK_DATA_MACHINE_I_FAR_END,
        TALK_DATA_MACHINE_I_FAR_START, TALK_DATA_MACHINE_I_FURTHER_END,
        TALK_DATA_MACHINE_I_FURTHER_START, TALK_DATA_MACHINE_I_NEARBY_END,
        TALK_DATA_MACHINE_I_NEARBY_START, TALK_DATA_MACHINE_NORMAL_FAR_END,
        TALK_DATA_MACHINE_NORMAL_FAR_START, TALK_DATA_MACHINE_NORMAL_FURTHER_END,
        TALK_DATA_MACHINE_NORMAL_FURTHER_START, TALK_DATA_MACHINE_NORMAL_NEARBY_END,
        TALK_DATA_MACHINE_NORMAL_NEARBY_START, TALK_DATA_STANDARD_B_FAR_END,
        TALK_DATA_STANDARD_B_FAR_START, TALK_DATA_STANDARD_B_FURTHER_END,
        TALK_DATA_STANDARD_B_FURTHER_START, TALK_DATA_STANDARD_B_NEARBY_END,
        TALK_DATA_STANDARD_B_NEARBY_START, TALK_DATA_STANDARD_I_FAR_END,
        TALK_DATA_STANDARD_I_FAR_START, TALK_DATA_STANDARD_I_FURTHER_END,
        TALK_DATA_STANDARD_I_FURTHER_START, TALK_DATA_STANDARD_I_NEARBY_END,
        TALK_DATA_STANDARD_I_NEARBY_START, TALK_DATA_STANDARD_NORMAL_FAR_END,
        TALK_DATA_STANDARD_NORMAL_FAR_START, TALK_DATA_STANDARD_NORMAL_FURTHER_END,
        TALK_DATA_STANDARD_NORMAL_FURTHER_START, TALK_DATA_STANDARD_NORMAL_NEARBY_END,
        TALK_DATA_STANDARD_NORMAL_NEARBY_START, TALK_SPACE_COMMON_CHATPREFIX,
        TALK_SPACE_COMMON_MESSAGEBBEND, TALK_SPACE_COMMON_MESSAGEBBSTART,
        TALK_SPACE_COMMON_PREFIXBBEND, TALK_SPACE_COMMON_PREFIXBBSTART, TALK_SPACE_COMMON_WORD,
        TALK_SPACE_GLOBAL_CHATPREFIX, TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX,
        TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBEND, TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBSTART,
        TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBEND, TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBSTART,
        TALK_SPACE_PROXIMITY_MESSAGEBBEND, TALK_SPACE_PROXIMITY_MESSAGEBBSTART,
        TALK_SPACE_PROXIMITY_PREFIXBBEND, TALK_SPACE_PROXIMITY_PREFIXBBSTART,
        TALK_SPACE_SECURITY_CHATPREFIX, TALK_SPACE_SECURITY_MESSAGEBBEND,
        TALK_SPACE_SECURITY_MESSAGEBBSTART, TALK_SPACE_SECURITY_PREFIXBBEND,
        TALK_SPACE_SECURITY_PREFIXBBSTART, TALK_SPACE_SECURITY_WORD,
        TALK_SPACE_SPECIALOPS_CHATPREFIX, TALK_SPACE_SPECIALOPS_MESSAGEBBEND,
        TALK_SPACE_SPECIALOPS_MESSAGEBBSTART, TALK_SPACE_SPECIALOPS_PREFIXBBEND,
        TALK_SPACE_SPECIALOPS_PREFIXBBSTART, TALK_SPACE_SPECIALOPS_WORD, TALK_STYLE_MACHINE_ASKS,
        TALK_STYLE_MACHINE_EXCLAIMS, TALK_STYLE_MACHINE_SHOUTS, TALK_STYLE_MACHINE_STANDARD,
        TALK_STYLE_STANDARD_ASKS, TALK_STYLE_STANDARD_EXCLAIMS, TALK_STYLE_STANDARD_SHOUTS,
        TALK_STYLE_STANDARD_STANDARD, TALK_TYPE_MACHINE_NEARBY_END, TALK_TYPE_MACHINE_NEARBY_START,
        TALK_TYPE_STANDARD_NEARBY_END, TALK_TYPE_STANDARD_NEARBY_START,
    },
    data::{ConnectedPlayer, HandleToEntity},
    entity_updates::EntityUpdateData,
    player_controller::SoftPlayer,
};
use bevy::prelude::SystemLabel;
use networking::messages::PendingMessage;
use networking::messages::ReliableServerMessage;
use networking::messages::{
    EntityWorldType, InputChatMessage, NetSendEntityUpdates, PendingNetworkMessage,
};
use networking_macros::NetMessage;
use pawn::pawn::{Pawn, PersistentPlayerData, ShipJobsEnum};
use sensable::core::Sensable;
use senser::senser::{to_doryen_coordinates, Senser};
use sfx::{proximity_message::PlaySoundProximityMessageData, radio_sound::PlaySoundRadioMessage};
use voca_rs::*;

/// Radio component for entities that can hear or speak through radios.
#[derive(Component)]
pub struct Radio {
    pub listen_access: Vec<RadioChannel>,
    pub speak_access: Vec<RadioChannel>,
}

/// All available chat channels.
#[derive(PartialEq, Debug, Clone)]
pub enum RadioChannel {
    Proximity,
    ProximityEmote,
    Global,
    Common,
    Security,
    SpecialOps,
}
#[derive(NetMessage)]
pub struct NetChatMessage {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
/// Handle chat message input.
pub(crate) fn chat_message_input_event(
    mut chat_message_input_events: EventReader<InputChatMessage>,
    handle_to_entity: Res<HandleToEntity>,
    player_pawns: Query<(&Pawn, &Transform, &Sensable)>,
    radio_pawns: Query<(Entity, &Radio, &Transform, &PersistentPlayerData)>,
    soft_player_query: Query<&SoftPlayer>,
    mut net_new_chat_message_event: EventWriter<NetChatMessage>,
    mut net_send_entity_updates: EventWriter<NetSendEntityUpdates>,
    global_listeners: Query<(&ConnectedPlayer, &PersistentPlayerData)>,
) {
    for chat_message_input_event in chat_message_input_events.iter() {
        let player_pawn_entity;
        player_pawn_entity = chat_message_input_event.entity;

        let player_components_result = player_pawns.get(player_pawn_entity);

        match player_components_result {
            Ok(player_components) => {
                let player_position;

                let translation = player_components.1.translation;
                player_position = Vec3::new(translation.x, translation.y, translation.z);

                new_chat_message(
                    &mut net_new_chat_message_event,
                    &handle_to_entity,
                    &player_components.2.sensed_by,
                    &player_components.2.sensed_by_cached,
                    player_position,
                    player_components.0.name.clone(),
                    player_components.0.job,
                    chat_message_input_event.message.clone(),
                    Communicator::Standard,
                    false,
                    &radio_pawns,
                    &global_listeners,
                    Some(&player_pawn_entity),
                    Some(&mut net_send_entity_updates),
                    &MessagingPlayerState::Alive,
                );
            }
            Err(_) => {
                // Soft connected chat

                let persistent_player_data_component;

                //Safety check.
                match soft_player_query.get(player_pawn_entity) {
                    Ok(_) => {}
                    Err(_rr) => {
                        continue;
                    }
                }

                match global_listeners.get(player_pawn_entity) {
                    Ok((_connected, persistent_data)) => {
                        persistent_player_data_component = persistent_data;
                    }
                    Err(_rr) => {
                        warn!("Couldnt find components for SoftConnected player with assumed global message.");
                        continue;
                    }
                }

                new_chat_message(
                    &mut net_new_chat_message_event,
                    &handle_to_entity,
                    &vec![],
                    &vec![],
                    Vec3::ZERO,
                    persistent_player_data_component.user_name.clone(),
                    ShipJobsEnum::Security,
                    chat_message_input_event.message.clone(),
                    Communicator::Standard,
                    false,
                    &radio_pawns,
                    &global_listeners,
                    Some(&player_pawn_entity),
                    Some(&mut net_send_entity_updates),
                    &MessagingPlayerState::SoftConnected,
                );
            }
        }
    }
}

/// Chat distance. Impacts font size.
enum Distance {
    Nearby,
    Further,
    Far,
}

/// Chat talk style variant.
enum TalkStyleVariant {
    Standard,
    Shouts,
    Exclaims,
    Asks,
}

/// The kind of communicator.
pub enum Communicator {
    Standard,
    Machine,
}

/// Check if a message has a shouting intend as a function.
fn is_shouting(message: &str) -> bool {
    message.ends_with("!!!")
        || message.ends_with("!!?")
        || message.ends_with("!?!")
        || message.ends_with("?!!")
        || message.ends_with("??!")
        || message.ends_with("?!?")
        || message.ends_with("??!")
        || message.ends_with("!??")
        || message.ends_with("???")
}

/// Check if a message has a questioning intend as a function.
fn is_asking(message: &str) -> bool {
    message.ends_with("?") || message.ends_with("??") || message.ends_with("?!")
}

/// Sets radio channel list for clients in setup UI to only show global chat availability as a function.
pub fn get_talk_spaces_setupui() -> Vec<(String, String)> {
    vec![(
        "Global".to_string(),
        TALK_SPACE_GLOBAL_CHATPREFIX.to_string(),
    )]
}

/// Process chat prefixes which act as flags as a function.
fn get_talk_space(message: String) -> (RadioChannel, String, bool, bool) {
    let radio_channel;
    let content;
    let mut exclusive_proximity = false;
    let mut is_emote = false;

    if message.starts_with(TALK_SPACE_GLOBAL_CHATPREFIX) {
        radio_channel = RadioChannel::Global;
        content = message.split(TALK_SPACE_GLOBAL_CHATPREFIX).collect();
    } else if message.starts_with(TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX) {
        radio_channel = RadioChannel::ProximityEmote;
        content = message
            .split(TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX)
            .collect();
        exclusive_proximity = true;
        is_emote = true;
    } else if message.starts_with(TALK_SPACE_COMMON_CHATPREFIX) {
        radio_channel = RadioChannel::Common;
        content = message.split(TALK_SPACE_COMMON_CHATPREFIX).collect();
    } else if message.starts_with(TALK_SPACE_SECURITY_CHATPREFIX) {
        radio_channel = RadioChannel::Security;
        content = message.split(TALK_SPACE_SECURITY_CHATPREFIX).collect();
    } else if message.starts_with(TALK_SPACE_SPECIALOPS_CHATPREFIX) {
        radio_channel = RadioChannel::SpecialOps;
        content = message.split(TALK_SPACE_SPECIALOPS_CHATPREFIX).collect();
    } else {
        radio_channel = RadioChannel::Proximity;
        content = message.to_owned();
        exclusive_proximity = true;
    }

    (radio_channel, content, exclusive_proximity, is_emote)
}

/// Manage global messages.
pub(crate) fn new_global_message(
    persistent_player_data_component: &PersistentPlayerData,
    global_listeners: &Query<(&ConnectedPlayer, &PersistentPlayerData)>,
    net_new_chat_message_event: &mut EventWriter<NetChatMessage>,
    send_message: String,
) {
    let message = persistent_player_data_component.user_name.clone()
        + "[b][color=#322bff](Global)[/color][/b]: "
        + &send_message;

    for (connected_player_component, _persistent) in global_listeners.iter() {
        if connected_player_component.connected == false {
            continue;
        }

        net_new_chat_message_event.send(NetChatMessage {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ChatMessage(message.clone()),
        });
    }
}

/// Parts of the chat and radio channels can and can't they access depend on it.
pub enum MessagingPlayerState {
    SoftConnected,
    Alive,
}

/// Function. It is huge, not-modular and just overall not nice. This will get modularized and rewritten for the Bevy client when it is ready.
pub fn new_chat_message(
    net_new_chat_message_event: &mut EventWriter<NetChatMessage>,
    handle_to_entity: &Res<HandleToEntity>,
    sensed_by: &Vec<Entity>,
    sensed_by_distance: &Vec<Entity>,
    position: Vec3,
    name: String,
    job: ShipJobsEnum,
    mut raw_message: String,
    communicator: Communicator,
    exclusive_radio: bool,
    radio_pawns: &Query<(Entity, &Radio, &Transform, &PersistentPlayerData)>,
    global_listeners: &Query<(&ConnectedPlayer, &PersistentPlayerData)>,
    messenger_entity_option: Option<&Entity>,
    mut net_send_entity_updates_option: Option<&mut EventWriter<NetSendEntityUpdates>>,
    messaging_player_state: &MessagingPlayerState,
) {
    if raw_message.len() > 500 {
        raw_message = raw_message[..500].to_string();
    }

    raw_message = escape_bb(raw_message, false, false);

    let mut radio_channel;
    let mut message;
    let mut exclusive_proximity;
    let mut is_emote;

    let result = get_talk_space(raw_message.clone());
    radio_channel = result.0;
    message = result.1;
    exclusive_proximity = result.2;
    is_emote = result.3;

    message = escape_bb(message, false, false);

    let mut prev_was_proximity;

    if matches!(radio_channel, RadioChannel::Proximity) {
        prev_was_proximity = true;
    } else {
        prev_was_proximity = false;
    }

    let mut proximity_emote_included;

    if matches!(radio_channel, RadioChannel::ProximityEmote) {
        proximity_emote_included = true;
    } else {
        proximity_emote_included = false;
    }

    let mut radio_command_included;
    let mut included_radio_channel;

    if !matches!(radio_channel, RadioChannel::Proximity)
        && !matches!(radio_channel, RadioChannel::ProximityEmote)
    {
        radio_command_included = true;
        included_radio_channel = Some(radio_channel.clone());
    } else {
        radio_command_included = false;
        included_radio_channel = None;
    }

    while !prev_was_proximity {
        let result = get_talk_space(message.clone());

        if matches!(result.0, RadioChannel::ProximityEmote) {
            proximity_emote_included = true;
        }

        if !matches!(radio_channel, RadioChannel::Proximity)
            && !matches!(radio_channel, RadioChannel::ProximityEmote)
        {
            radio_command_included = true;
            included_radio_channel = Some(radio_channel.clone());
        }

        if matches!(result.0, RadioChannel::Proximity) {
            prev_was_proximity = true;
        } else {
            prev_was_proximity = false;
            radio_channel = result.0;
            message = result.1;
            exclusive_proximity = result.2;
            is_emote = result.3;

            message = escape_bb(message, false, false);
        }
    }

    if matches!(messaging_player_state, &MessagingPlayerState::SoftConnected) {
        radio_channel = RadioChannel::Global;
    }

    // Emote over Radio channel for the memes.
    if !matches!(radio_channel, RadioChannel::ProximityEmote) {
        if proximity_emote_included {
            is_emote = true;
            exclusive_proximity = false;
        } else {
            message = case::capitalize(&message, false);
        }
    } else {
        if radio_command_included {
            is_emote = true;
            exclusive_proximity = false;
            radio_channel = included_radio_channel.unwrap();

            if matches!(radio_channel, RadioChannel::Global) {
                message = case::capitalize(&message, false);
            }
        }
    }

    if matches!(radio_channel, RadioChannel::Global) {
        match global_listeners.get(*messenger_entity_option.unwrap()) {
            Ok((_connected, persistent_player_data_component)) => {
                new_global_message(
                    persistent_player_data_component,
                    global_listeners,
                    net_new_chat_message_event,
                    message,
                );
            }
            Err(_rr) => {
                warn!("Couldnt find components of global messenger.");
            }
        }

        return;
    }

    if exclusive_radio == true {
        exclusive_proximity = false;
    }

    if message.len() == 0 {
        return;
    }

    let mut talk_style_variation = TalkStyleVariant::Standard;

    if is_emote == false {
        if is_shouting(&message) {
            talk_style_variation = TalkStyleVariant::Shouts;
        } else if message.ends_with("!") {
            talk_style_variation = TalkStyleVariant::Exclaims;
        } else if is_asking(&message) {
            talk_style_variation = TalkStyleVariant::Asks;
        }
    }

    let mut radio_message: String = "".to_string();

    if exclusive_proximity == false {
        // Radio chat message.

        let talk_space_prefix_bb_start;
        let talk_space_prefix_bb_end;
        let talk_space_message_bb_start;
        let talk_space_message_bb_end;
        let mut talk_space_word = "";
        match radio_channel {
            RadioChannel::Proximity => {
                talk_space_prefix_bb_start = TALK_SPACE_PROXIMITY_PREFIXBBSTART;
                talk_space_prefix_bb_end = TALK_SPACE_PROXIMITY_PREFIXBBEND;
                talk_space_message_bb_start = TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
                talk_space_message_bb_end = TALK_SPACE_PROXIMITY_MESSAGEBBEND;
            }
            RadioChannel::ProximityEmote => {
                talk_space_prefix_bb_start = TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBSTART;
                talk_space_prefix_bb_end = TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBEND;
                talk_space_message_bb_start = TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBSTART;
                talk_space_message_bb_end = TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBEND;
            }
            RadioChannel::Common => {
                talk_space_prefix_bb_start = TALK_SPACE_COMMON_PREFIXBBSTART;
                talk_space_word = TALK_SPACE_COMMON_WORD;
                talk_space_prefix_bb_end = TALK_SPACE_COMMON_PREFIXBBEND;
                talk_space_message_bb_start = TALK_SPACE_COMMON_MESSAGEBBSTART;
                talk_space_message_bb_end = TALK_SPACE_COMMON_MESSAGEBBEND;
            }
            RadioChannel::Security => {
                talk_space_prefix_bb_start = TALK_SPACE_SECURITY_PREFIXBBSTART;
                talk_space_word = TALK_SPACE_SECURITY_WORD;
                talk_space_prefix_bb_end = TALK_SPACE_SECURITY_PREFIXBBEND;
                talk_space_message_bb_start = TALK_SPACE_SECURITY_MESSAGEBBSTART;
                talk_space_message_bb_end = TALK_SPACE_SECURITY_MESSAGEBBEND;
            }
            RadioChannel::SpecialOps => {
                talk_space_prefix_bb_start = TALK_SPACE_SPECIALOPS_PREFIXBBSTART;
                talk_space_word = TALK_SPACE_SPECIALOPS_WORD;
                talk_space_prefix_bb_end = TALK_SPACE_SPECIALOPS_PREFIXBBEND;
                talk_space_message_bb_start = TALK_SPACE_SPECIALOPS_MESSAGEBBSTART;
                talk_space_message_bb_end = TALK_SPACE_SPECIALOPS_MESSAGEBBEND;
            }
            RadioChannel::Global => {
                warn!("Processing global chat while we shouldn't?");
                return;
            }
        }

        let talk_font_nearby_start;
        let talk_font_nearby_end;

        let talk_font_nearby_start_1;
        let talk_font_nearby_end_1;
        let talk_style_variation_word;
        match communicator {
            Communicator::Standard => {
                talk_font_nearby_start = TALK_DATA_STANDARD_B_NEARBY_START;
                talk_font_nearby_end = TALK_DATA_STANDARD_B_NEARBY_END;

                talk_font_nearby_start_1 = TALK_TYPE_STANDARD_NEARBY_START;
                talk_font_nearby_end_1 = TALK_TYPE_STANDARD_NEARBY_END;

                match talk_style_variation {
                    TalkStyleVariant::Standard => {
                        talk_style_variation_word = TALK_STYLE_STANDARD_STANDARD;
                    }
                    TalkStyleVariant::Shouts => {
                        talk_style_variation_word = TALK_STYLE_STANDARD_SHOUTS;
                    }
                    TalkStyleVariant::Exclaims => {
                        talk_style_variation_word = TALK_STYLE_STANDARD_EXCLAIMS;
                    }
                    TalkStyleVariant::Asks => {
                        talk_style_variation_word = TALK_STYLE_STANDARD_ASKS;
                    }
                }
            }
            Communicator::Machine => {
                talk_font_nearby_start = TALK_DATA_MACHINE_B_NEARBY_START;
                talk_font_nearby_end = TALK_DATA_MACHINE_B_NEARBY_END;

                talk_font_nearby_start_1 = TALK_TYPE_MACHINE_NEARBY_START;
                talk_font_nearby_end_1 = TALK_TYPE_MACHINE_NEARBY_END;
                match talk_style_variation {
                    TalkStyleVariant::Standard => {
                        talk_style_variation_word = TALK_STYLE_MACHINE_STANDARD;
                    }
                    TalkStyleVariant::Shouts => {
                        talk_style_variation_word = TALK_STYLE_MACHINE_SHOUTS;
                    }
                    TalkStyleVariant::Exclaims => {
                        talk_style_variation_word = TALK_STYLE_MACHINE_EXCLAIMS;
                    }
                    TalkStyleVariant::Asks => {
                        talk_style_variation_word = TALK_STYLE_MACHINE_ASKS;
                    }
                }
            }
        }

        let rank_word;
        match job {
            ShipJobsEnum::Security => {
                rank_word = JOB_SECURITY_WORD;
            }
            ShipJobsEnum::Control => {
                rank_word = JOB_CONTROL_WORD;
            }
        }

        if is_emote {
            radio_message = radio_message + talk_space_prefix_bb_start;
            radio_message = radio_message + talk_font_nearby_start;
            radio_message =
                radio_message + &name + " [" + talk_space_word + "][" + rank_word + "] ";
            radio_message = radio_message + talk_font_nearby_end + talk_space_prefix_bb_end;
            radio_message = radio_message + talk_space_message_bb_start;

            radio_message =
                radio_message + talk_font_nearby_start_1 + &message + talk_font_nearby_end_1;
            radio_message = radio_message + talk_space_message_bb_end;
        } else {
            radio_message = radio_message + talk_space_prefix_bb_start;
            radio_message = radio_message + talk_font_nearby_start;
            radio_message =
                radio_message + &name + " [" + talk_space_word + "][" + rank_word + "] ";
            radio_message = radio_message + talk_font_nearby_end + talk_space_prefix_bb_end;
            radio_message = radio_message + talk_space_message_bb_start;

            radio_message = radio_message + talk_style_variation_word + ",\n";

            if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
                radio_message = radio_message
                    + talk_font_nearby_start_1
                    + "[font="
                    + NEARBY_SHOUT_FONT
                    + "]\""
                    + &message
                    + "\"[/font]"
                    + talk_font_nearby_end_1;
            } else {
                radio_message = radio_message
                    + talk_font_nearby_start_1
                    + "\""
                    + &message
                    + "\""
                    + talk_font_nearby_end_1;
            }

            radio_message = radio_message + talk_space_message_bb_end;
        }
    }

    // Build proximity message.
    // For 3 different distances.

    let mut proximity_message_nearby = "".to_string();
    let mut proximity_message_further = "".to_string();
    let mut proximity_message_far = "".to_string();

    if exclusive_radio == false {
        proximity_message_nearby = proximity_message_nearby
            + "[font="
            + NEARBY_BOLD_FONT
            + "]"
            + TALK_SPACE_PROXIMITY_PREFIXBBSTART;
        proximity_message_further = proximity_message_further
            + "[font="
            + FURTHER_BOLD_FONT
            + "]"
            + TALK_SPACE_PROXIMITY_PREFIXBBSTART;
        proximity_message_far = proximity_message_far
            + "[font="
            + FAR_BOLD_FONT
            + "]"
            + TALK_SPACE_PROXIMITY_PREFIXBBSTART;

        let nearby_talk_data_b_end;
        let further_talk_data_b_end;
        let far_talk_data_b_end;

        match communicator {
            Communicator::Standard => {
                proximity_message_nearby =
                    proximity_message_nearby + TALK_DATA_STANDARD_B_NEARBY_START;
                proximity_message_further =
                    proximity_message_further + TALK_DATA_STANDARD_B_FURTHER_START;
                proximity_message_far = proximity_message_far + TALK_DATA_STANDARD_B_FAR_START;

                nearby_talk_data_b_end = TALK_DATA_STANDARD_B_NEARBY_END;
                further_talk_data_b_end = TALK_DATA_STANDARD_B_FURTHER_END;
                far_talk_data_b_end = TALK_DATA_STANDARD_B_FAR_END;
            }
            Communicator::Machine => {
                proximity_message_nearby =
                    proximity_message_nearby + TALK_DATA_MACHINE_B_NEARBY_START;
                proximity_message_further =
                    proximity_message_further + TALK_DATA_MACHINE_B_FURTHER_START;
                proximity_message_far = proximity_message_far + TALK_DATA_MACHINE_B_FAR_START;

                nearby_talk_data_b_end = TALK_DATA_MACHINE_B_NEARBY_END;
                further_talk_data_b_end = TALK_DATA_MACHINE_B_FURTHER_END;
                far_talk_data_b_end = TALK_DATA_MACHINE_B_FAR_END;
            }
        }

        proximity_message_nearby = proximity_message_nearby + &name + nearby_talk_data_b_end + " ";
        proximity_message_further =
            proximity_message_further + &name + further_talk_data_b_end + " ";
        proximity_message_far = proximity_message_far + &name + far_talk_data_b_end + " ";

        let rank_word;
        match job {
            ShipJobsEnum::Security => {
                rank_word = JOB_SECURITY_WORD;
            }
            ShipJobsEnum::Control => {
                rank_word = JOB_CONTROL_WORD;
            }
        }

        if is_emote == false {
            proximity_message_nearby = proximity_message_nearby + "[" + rank_word + "]";
            proximity_message_further = proximity_message_further + "[" + rank_word + "]";
            proximity_message_far = proximity_message_far + "[" + rank_word + "]";
        }

        proximity_message_nearby = proximity_message_nearby
            + TALK_SPACE_PROXIMITY_PREFIXBBEND
            + "[/font]"
            + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
        proximity_message_further = proximity_message_further
            + TALK_SPACE_PROXIMITY_PREFIXBBEND
            + "[/font]"
            + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
        proximity_message_far = proximity_message_far
            + TALK_SPACE_PROXIMITY_PREFIXBBEND
            + "[/font]"
            + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;

        if is_emote == false {
            let talk_style_variation_word;
            match communicator {
                Communicator::Standard => {
                    proximity_message_nearby =
                        proximity_message_nearby + TALK_DATA_STANDARD_NORMAL_NEARBY_START;
                    proximity_message_further =
                        proximity_message_further + TALK_DATA_STANDARD_NORMAL_FURTHER_START;
                    proximity_message_far =
                        proximity_message_far + TALK_DATA_STANDARD_NORMAL_FAR_START;

                    match talk_style_variation {
                        TalkStyleVariant::Standard => {
                            talk_style_variation_word = TALK_STYLE_STANDARD_STANDARD;
                        }
                        TalkStyleVariant::Shouts => {
                            talk_style_variation_word = TALK_STYLE_STANDARD_SHOUTS;
                        }
                        TalkStyleVariant::Exclaims => {
                            talk_style_variation_word = TALK_STYLE_STANDARD_EXCLAIMS;
                        }
                        TalkStyleVariant::Asks => {
                            talk_style_variation_word = TALK_STYLE_STANDARD_ASKS;
                        }
                    }
                }
                Communicator::Machine => {
                    proximity_message_nearby =
                        proximity_message_nearby + TALK_DATA_MACHINE_NORMAL_NEARBY_START;
                    proximity_message_further =
                        proximity_message_further + TALK_DATA_MACHINE_NORMAL_FURTHER_START;
                    proximity_message_far =
                        proximity_message_far + TALK_DATA_MACHINE_NORMAL_FAR_START;

                    match talk_style_variation {
                        TalkStyleVariant::Standard => {
                            talk_style_variation_word = TALK_STYLE_MACHINE_STANDARD;
                        }
                        TalkStyleVariant::Shouts => {
                            talk_style_variation_word = TALK_STYLE_MACHINE_SHOUTS;
                        }
                        TalkStyleVariant::Exclaims => {
                            talk_style_variation_word = TALK_STYLE_MACHINE_EXCLAIMS;
                        }
                        TalkStyleVariant::Asks => {
                            talk_style_variation_word = TALK_STYLE_MACHINE_ASKS;
                        }
                    }
                }
            }

            proximity_message_nearby = proximity_message_nearby + talk_style_variation_word + ",\n";
            proximity_message_further =
                proximity_message_further + talk_style_variation_word + ",\n";
            proximity_message_far = proximity_message_far + talk_style_variation_word + ",\n";

            let nearby_talk_data_i_start;
            let further_talk_data_i_start;
            let far_talk_data_i_start;

            let nearby_talk_data_i_end;
            let further_talk_data_i_end;
            let far_talk_data_i_end;

            let nearby_talk_data_normal_start;
            let further_talk_data_normal_start;
            let far_talk_data_normal_start;

            let nearby_talk_data_normal_end;
            let further_talk_data_normal_end;
            let far_talk_data_normal_end;

            let nearby_talk_data_b_start;
            let further_talk_data_b_start;
            let far_talk_data_b_start;

            let nearby_shout_data_i_start;
            let further_shout_data_i_start;
            let far_shout_data_i_start;

            let nearby_shout_data_i_end;
            let further_shout_data_i_end;
            let far_shout_data_i_end;

            let nearby_talk_data_start;
            let further_talk_data_start;
            let far_talk_data_start;

            let nearby_talk_data_end;
            let further_talk_data_end;
            let far_talk_data_end;

            match communicator {
                Communicator::Standard => {
                    proximity_message_nearby =
                        proximity_message_nearby + TALK_DATA_STANDARD_NORMAL_NEARBY_END;
                    proximity_message_further =
                        proximity_message_further + TALK_DATA_STANDARD_NORMAL_FURTHER_END;
                    proximity_message_far =
                        proximity_message_far + TALK_DATA_STANDARD_NORMAL_FAR_END;

                    nearby_talk_data_i_start = TALK_DATA_STANDARD_I_NEARBY_START;
                    further_talk_data_i_start = TALK_DATA_STANDARD_I_FURTHER_START;
                    far_talk_data_i_start = TALK_DATA_STANDARD_I_FAR_START;

                    nearby_talk_data_i_end = TALK_DATA_STANDARD_I_NEARBY_END;
                    further_talk_data_i_end = TALK_DATA_STANDARD_I_FURTHER_END;
                    far_talk_data_i_end = TALK_DATA_STANDARD_I_FAR_END;

                    nearby_talk_data_normal_start = TALK_DATA_STANDARD_NORMAL_NEARBY_START;
                    further_talk_data_normal_start = TALK_DATA_STANDARD_NORMAL_FURTHER_START;
                    far_talk_data_normal_start = TALK_DATA_STANDARD_NORMAL_FAR_START;

                    nearby_talk_data_normal_end = TALK_DATA_STANDARD_NORMAL_NEARBY_END;
                    further_talk_data_normal_end = TALK_DATA_STANDARD_NORMAL_FURTHER_END;
                    far_talk_data_normal_end = TALK_DATA_STANDARD_NORMAL_FAR_END;

                    nearby_talk_data_b_start = TALK_DATA_STANDARD_B_NEARBY_START;
                    further_talk_data_b_start = TALK_DATA_STANDARD_B_FURTHER_START;
                    far_talk_data_b_start = TALK_DATA_STANDARD_B_FAR_START;

                    nearby_shout_data_i_start = SHOUT_DATA_STANDARD_NEARBY_I_START;
                    further_shout_data_i_start = SHOUT_DATA_STANDARD_FURTHER_I_START;
                    far_shout_data_i_start = SHOUT_DATA_STANDARD_FAR_I_START;

                    nearby_shout_data_i_end = SHOUT_DATA_STANDARD_NEARBY_I_END;
                    further_shout_data_i_end = SHOUT_DATA_STANDARD_FURTHER_I_END;
                    far_shout_data_i_end = SHOUT_DATA_STANDARD_FAR_I_END;

                    nearby_talk_data_start = TALK_TYPE_STANDARD_NEARBY_START;
                    further_talk_data_start = TALK_TYPE_STANDARD_NEARBY_START;
                    far_talk_data_start = TALK_TYPE_STANDARD_NEARBY_START;

                    nearby_talk_data_end = TALK_TYPE_STANDARD_NEARBY_END;
                    further_talk_data_end = TALK_TYPE_STANDARD_NEARBY_END;
                    far_talk_data_end = TALK_TYPE_STANDARD_NEARBY_END;
                }
                Communicator::Machine => {
                    proximity_message_nearby =
                        proximity_message_nearby + TALK_DATA_MACHINE_NORMAL_NEARBY_END;
                    proximity_message_further =
                        proximity_message_further + TALK_DATA_MACHINE_NORMAL_FURTHER_END;
                    proximity_message_far =
                        proximity_message_far + TALK_DATA_MACHINE_NORMAL_FAR_END;

                    nearby_talk_data_i_start = TALK_DATA_MACHINE_I_NEARBY_START;
                    further_talk_data_i_start = TALK_DATA_MACHINE_I_FURTHER_START;
                    far_talk_data_i_start = TALK_DATA_MACHINE_I_FAR_START;

                    nearby_talk_data_i_end = TALK_DATA_MACHINE_I_NEARBY_END;
                    further_talk_data_i_end = TALK_DATA_MACHINE_I_FURTHER_END;
                    far_talk_data_i_end = TALK_DATA_MACHINE_I_FAR_END;

                    nearby_talk_data_normal_start = TALK_DATA_MACHINE_NORMAL_NEARBY_START;
                    further_talk_data_normal_start = TALK_DATA_MACHINE_NORMAL_FURTHER_START;
                    far_talk_data_normal_start = TALK_DATA_MACHINE_NORMAL_FAR_START;

                    nearby_talk_data_normal_end = TALK_DATA_MACHINE_NORMAL_NEARBY_END;
                    further_talk_data_normal_end = TALK_DATA_MACHINE_NORMAL_FURTHER_END;
                    far_talk_data_normal_end = TALK_DATA_MACHINE_NORMAL_FAR_END;

                    nearby_talk_data_b_start = TALK_DATA_MACHINE_B_NEARBY_START;
                    further_talk_data_b_start = TALK_DATA_MACHINE_B_FURTHER_START;
                    far_talk_data_b_start = TALK_DATA_MACHINE_B_FAR_START;

                    nearby_shout_data_i_start = SHOUT_DATA_MACHINE_NEARBY_I_START;
                    further_shout_data_i_start = SHOUT_DATA_MACHINE_FURTHER_I_START;
                    far_shout_data_i_start = SHOUT_DATA_MACHINE_FAR_I_START;

                    nearby_shout_data_i_end = SHOUT_DATA_MACHINE_NEARBY_I_END;
                    further_shout_data_i_end = SHOUT_DATA_MACHINE_FURTHER_I_END;
                    far_shout_data_i_end = SHOUT_DATA_MACHINE_FAR_I_END;

                    nearby_talk_data_start = TALK_TYPE_MACHINE_NEARBY_START;
                    further_talk_data_start = TALK_TYPE_MACHINE_NEARBY_START;
                    far_talk_data_start = TALK_TYPE_MACHINE_NEARBY_START;

                    nearby_talk_data_end = TALK_TYPE_MACHINE_NEARBY_END;
                    further_talk_data_end = TALK_TYPE_MACHINE_NEARBY_END;
                    far_talk_data_end = TALK_TYPE_MACHINE_NEARBY_END;
                }
            }

            if exclusive_proximity == false {
                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_i_start;
                proximity_message_further = proximity_message_further + further_talk_data_i_start;
                proximity_message_far = proximity_message_far + far_talk_data_i_start;
            } else {
                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_normal_start;
                proximity_message_further =
                    proximity_message_further + further_talk_data_normal_start;
                proximity_message_far = proximity_message_far + far_talk_data_normal_start;
            }

            if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
                if exclusive_proximity == false {
                    proximity_message_nearby = proximity_message_nearby
                        + nearby_shout_data_i_start
                        + &message
                        + nearby_shout_data_i_end;
                    proximity_message_further = proximity_message_further
                        + further_shout_data_i_start
                        + &message
                        + further_shout_data_i_end;
                    proximity_message_far = proximity_message_far
                        + far_shout_data_i_start
                        + &message
                        + far_shout_data_i_end;
                } else {
                    proximity_message_nearby = proximity_message_nearby
                        + nearby_talk_data_b_start
                        + "[font="
                        + NEARBY_SHOUT_FONT
                        + "]\""
                        + &message
                        + "\"[/font]"
                        + nearby_talk_data_b_end;
                    proximity_message_further = proximity_message_further
                        + further_talk_data_b_start
                        + "[font="
                        + FURTHER_SHOUT_FONT
                        + "]\""
                        + &message
                        + "\"[/font]"
                        + further_talk_data_b_end;
                    proximity_message_far = proximity_message_far
                        + far_talk_data_b_start
                        + "[font="
                        + FAR_SHOUT_FONT
                        + "]\""
                        + &message
                        + "\"[/font]"
                        + far_talk_data_b_end;
                }
            } else {
                proximity_message_nearby = proximity_message_nearby
                    + nearby_talk_data_start
                    + "\""
                    + &message
                    + "\""
                    + nearby_talk_data_end;
                proximity_message_further = proximity_message_further
                    + further_talk_data_start
                    + "\""
                    + &message
                    + "\""
                    + further_talk_data_end;
                proximity_message_far = proximity_message_far
                    + far_talk_data_start
                    + "\""
                    + &message
                    + "\""
                    + far_talk_data_end;
            }

            if exclusive_proximity == false {
                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_i_end;
                proximity_message_further = proximity_message_further + further_talk_data_i_end;
                proximity_message_far = proximity_message_far + far_talk_data_i_end;
            } else {
                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_normal_end;
                proximity_message_further =
                    proximity_message_further + further_talk_data_normal_end;
                proximity_message_far = proximity_message_far + far_talk_data_normal_end;
            }
        } else {
            proximity_message_nearby = proximity_message_nearby + &message;
            proximity_message_further = proximity_message_further + &message;
            proximity_message_far = proximity_message_far + &message;
        }

        proximity_message_nearby = proximity_message_nearby + TALK_SPACE_PROXIMITY_MESSAGEBBEND;
        proximity_message_further = proximity_message_further + TALK_SPACE_PROXIMITY_MESSAGEBBEND;
        proximity_message_far = proximity_message_far + TALK_SPACE_PROXIMITY_MESSAGEBBEND;
    }

    // Todo...
    // Create & send proximity billboard message.

    let mut billboard_message = "".to_string();

    if exclusive_radio == false {
        billboard_message = billboard_message + BILLBOARD_DATA_SECURITY_START;

        let nearby_talk_data_i_start;
        let nearby_talk_data_i_end;
        let nearby_talk_data_b_start;
        let nearby_talk_data_b_end;

        match communicator {
            Communicator::Standard => {
                nearby_talk_data_i_start = TALK_DATA_STANDARD_I_NEARBY_START;
                nearby_talk_data_i_end = TALK_DATA_STANDARD_I_NEARBY_END;

                nearby_talk_data_b_start = TALK_DATA_STANDARD_B_NEARBY_START;
                nearby_talk_data_b_end = TALK_DATA_STANDARD_B_NEARBY_END;
            }
            Communicator::Machine => {
                nearby_talk_data_i_start = TALK_DATA_MACHINE_I_NEARBY_START;
                nearby_talk_data_i_end = TALK_DATA_STANDARD_I_NEARBY_END;

                nearby_talk_data_b_start = TALK_DATA_STANDARD_B_NEARBY_START;
                nearby_talk_data_b_end = TALK_DATA_STANDARD_B_NEARBY_END;
            }
        }

        if exclusive_proximity == false {
            billboard_message = billboard_message + nearby_talk_data_i_start;
        }

        if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
            billboard_message = billboard_message + nearby_talk_data_b_start;

            if exclusive_proximity == false {
                billboard_message = billboard_message
                    + "[font="
                    + BILLBOARD_SHOUT_FONT
                    + "]"
                    + &message
                    + "[/font]";
            } else {
                billboard_message = billboard_message
                    + "[font="
                    + BILLBOARD_SHOUT_ITALIC_FONT
                    + "]"
                    + &message
                    + "[/font]";
            }

            billboard_message = billboard_message + nearby_talk_data_b_end;
        } else {
            if is_emote {
                billboard_message = billboard_message + nearby_talk_data_i_start;
                billboard_message = billboard_message + "*" + &message + "*";
                billboard_message = billboard_message + nearby_talk_data_i_end;
            } else {
                billboard_message = billboard_message + &message;
            }
        }

        if exclusive_proximity == false {
            billboard_message = billboard_message + nearby_talk_data_i_end;
        }

        billboard_message = billboard_message + BILLBOARD_DATA_SECURITY_END;
    }

    // Send radio message to all radio_pawns who can listen to that channel.

    let mut handles_direct_proximity: Vec<u64> = vec![];
    let mut handles_radio: Vec<u64> = vec![];

    if exclusive_proximity == false {
        let mut has_radio_permission = false;

        match messenger_entity_option {
            Some(entity) => {
                let messenger_components_result = radio_pawns.get(*entity);

                match messenger_components_result {
                    Ok((
                        _entity,
                        radio_component,
                        _rigid_body_handle_component,
                        _persistent_player_data_component,
                    )) => {
                        if radio_component.speak_access.contains(&radio_channel) {
                            has_radio_permission = true;
                        }
                    }
                    Err(_rr) => {
                        return;
                    }
                }
            }
            None => {
                has_radio_permission = true;
            }
        }

        if has_radio_permission {
            for (
                entity,
                radio_component,
                _rigid_body_handle_component,
                _persistent_player_data_component,
            ) in radio_pawns.iter()
            {
                if radio_component.listen_access.contains(&radio_channel) {
                    let listener_handle_result = handle_to_entity.inv_map.get(&entity);
                    match listener_handle_result {
                        Some(listener_handle) => {
                            net_new_chat_message_event.send(NetChatMessage {
                                handle: *listener_handle,
                                message: ReliableServerMessage::ChatMessage(radio_message.clone()),
                            });

                            handles_radio.push(*listener_handle);
                        }
                        None => {}
                    }
                }
            }
        }
    }

    if exclusive_radio == false {
        // Proximity messages to listeners based on distance and shouting.
        let mut sensed_by_list;

        if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
            sensed_by_list = vec![];
            for entity in sensed_by {
                sensed_by_list.push(*entity);
            }
            for entity in sensed_by_distance {
                sensed_by_list.push(*entity);
            }
        } else {
            sensed_by_list = sensed_by.to_vec();
        }

        // Build billboard entity_update
        let mut billboard_entity_update = HashMap::new();

        let mut parameters_entity_update = HashMap::new();

        parameters_entity_update.insert(
            "billboardMessage".to_string(),
            EntityUpdateData::String(billboard_message.clone()),
        );

        billboard_entity_update.insert(
            "Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText".to_string(),
            parameters_entity_update,
        );

        for entity in sensed_by_list {
            let sensed_by_entity_components_result = radio_pawns.get(entity);

            match sensed_by_entity_components_result {
                Ok((
                    entity,
                    _radio_component,
                    rigid_body_position,
                    _persistent_player_data_component,
                )) => {
                    let listener_handle_result = handle_to_entity.inv_map.get(&entity);

                    match listener_handle_result {
                        Some(listener_handle) => {
                            let listener_rigid_body_transform = rigid_body_position.translation;
                            let listener_position = Vec3::new(
                                listener_rigid_body_transform.x,
                                listener_rigid_body_transform.y,
                                listener_rigid_body_transform.z,
                            );

                            let listener_distance = position.distance(listener_position);

                            let distance;

                            if listener_distance > 24. {
                                distance = Distance::Far;
                            } else if listener_distance > 14. {
                                distance = Distance::Further;
                            } else {
                                distance = Distance::Nearby;
                            }

                            match distance {
                                Distance::Nearby => {
                                    net_new_chat_message_event.send(NetChatMessage {
                                        handle: *listener_handle,
                                        message: ReliableServerMessage::ChatMessage(
                                            proximity_message_nearby.clone(),
                                        ),
                                    });
                                }
                                Distance::Further => {
                                    net_new_chat_message_event.send(NetChatMessage {
                                        handle: *listener_handle,
                                        message: ReliableServerMessage::ChatMessage(
                                            proximity_message_further.clone(),
                                        ),
                                    });
                                }
                                Distance::Far => {
                                    net_new_chat_message_event.send(NetChatMessage {
                                        handle: *listener_handle,
                                        message: ReliableServerMessage::ChatMessage(
                                            proximity_message_far.clone(),
                                        ),
                                    });
                                }
                            }

                            match net_send_entity_updates_option {
                                Some(ref mut net_send_entity_updates) => {
                                    match messenger_entity_option {
                                        Some(messenger_entity) => {
                                            net_send_entity_updates.send(NetSendEntityUpdates {
                                                handle: *listener_handle,
                                                message: ReliableServerMessage::EntityUpdate(
                                                    messenger_entity.to_bits(),
                                                    billboard_entity_update.clone(),
                                                    false,
                                                    EntityWorldType::Main,
                                                ),
                                            });
                                            handles_direct_proximity.push(*listener_handle);
                                        }
                                        None => {
                                            error!("Cannot send proximity message without providing messeger_entity.");
                                        }
                                    }
                                }
                                None => {
                                    error!("Cannot send proximity message without providing EventWriter<NetSendEntityUpdates>.");
                                }
                            }
                        }
                        None => {}
                    }
                }
                Err(_rr) => {}
            }
        }
    }

    for player_handle in handles_direct_proximity.iter() {
        net_new_chat_message_event.send(NetChatMessage {
            handle: *player_handle,
            message: PlaySoundProximityMessageData::get_message(position),
        });
    }

    for player_handle in handles_radio.iter() {
        if !handles_direct_proximity.contains(player_handle) {
            net_new_chat_message_event.send(NetChatMessage {
                handle: *player_handle,
                message: PlaySoundRadioMessage::get_message(),
            });
        }
    }
}

/// Requested proximity message event.
pub struct EntityProximityMessage {
    pub entities: Vec<Entity>,
    pub message: String,
}
#[derive(NetMessage)]
pub(crate) struct NetProximityMessage {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
/// Requested entity proximity messages systems ordering label.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum EntityProximityMessages {
    Send,
}

/// Send entity proximity messages to receivers.
pub(crate) fn send_entity_proximity_messages(
    mut entity_proximity_messages: EventReader<EntityProximityMessage>,
    sensers: Query<(Entity, &Senser)>,
    positions: Query<&Transform>,
    handle_to_entity: Res<HandleToEntity>,
    mut net: EventWriter<NetProximityMessage>,
) {
    for entity_proximity_message in entity_proximity_messages.iter() {
        for proximity_entity in entity_proximity_message.entities.iter() {
            let entity_transform;

            match positions.get(*proximity_entity) {
                Ok(t) => {
                    entity_transform = t;
                }
                Err(_rr) => {
                    warn!("Couldnt find transform of entity");
                    continue;
                }
            }

            let entity_gridmap_coords = world_to_cell_id(entity_transform.translation);
            let entity_cell_id_doryen =
                to_doryen_coordinates(entity_gridmap_coords.x, entity_gridmap_coords.z);

            for (entity, senser_component) in sensers.iter() {
                if senser_component.fov.is_in_fov(
                    entity_cell_id_doryen.0 as usize,
                    entity_cell_id_doryen.1 as usize,
                ) {
                    match handle_to_entity.inv_map.get(&entity) {
                        Some(handle) => {
                            net.send(NetProximityMessage {
                                handle: *handle,
                                message: ReliableServerMessage::ChatMessage(
                                    entity_proximity_message.message.clone(),
                                ),
                            });
                        }
                        None => {}
                    }
                }
            }
        }
    }
}
