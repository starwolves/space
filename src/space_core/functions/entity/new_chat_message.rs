
use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Entity, EventWriter, Query, Res, error, warn}};
use bevy_rapier3d::{prelude::RigidBodyPosition};
use const_format::concatcp;

use crate::space_core::{bundles::{play_sound_proximity_message::PlaySoundProximityMessage, play_sound_radio_message::PlaySoundRadioMessage}, components::{connected_player::ConnectedPlayer, pawn::SpaceJobsEnum, persistent_player_data::PersistentPlayerData, radio::{Radio, RadioChannel}}, events::net::{net_chat_message::NetChatMessage, net_send_entity_updates::NetSendEntityUpdates}, resources::{handle_to_entity::HandleToEntity, network_messages::{EntityUpdateData, ReliableServerMessage}}};

const BILLBOARD_SHOUT_FONT : &str = "res://assets/fonts/RobotoFamily/RobotoCondensed/RobotoCondensed-BoldShoutDyna.tres";
const BILLBOARD_SHOUT_ITALIC_FONT : &str = "res://assets/fonts/RobotoFamily/RobotoCondensed/RobotoCondensed-BoldShoutItalicDyna.tres";

const NEARBY_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularBoldDyna.tres";
const _NEARBY_ITALIC_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumItDyna.tres";
const _NEARBY_NORMAL_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumDyna.tres";
pub const NEARBY_SHOUT_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldDyna.tres";
const _NEARBY_MACHINE_MEDIUM_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightMediumDyna.tres";
const _NEARBY_MACHINE_ITALIC_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicDyna.tres";
const _NEARBY_MACHINE_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightBoldDyna.tres";
const _NEARBY_MACHINE_ITALIC_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicBoldDyna.tres";

const FURTHER_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularBoldDynaSmall.tres";
pub const FURTHER_ITALIC_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumItDynaSmall.tres";
pub const FURTHER_NORMAL_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumDynaSmall.tres";
const FURTHER_SHOUT_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldDynaSmall.tres";
const FURTHER_MACHINE_MEDIUM_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightMediumDynaSmall.tres";
const FURTHER_MACHINE_ITALIC_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicDynaSmall.tres";
const FURTHER_MACHINE_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightBoldDynaSmall.tres";
const _FURTHER_MACHINE_ITALIC_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicBoldDynaSmall.tres";


const FAR_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularBoldDynaSmaller.tres";
const FAR_ITALIC_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumItDynaSmaller.tres";
const FAR_NORMAL_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatRegularMediumDynaSmaller.tres";
const FAR_SHOUT_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldDynaSmaller.tres";
const FAR_MACHINE_MEDIUM_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightMediumDynaSmaller.tres";
const FAR_MACHINE_ITALIC_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicDynaSmaller.tres";
const FAR_MACHINE_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightBoldDynaSmaller.tres";
const _FAR_MACHINE_ITALIC_BOLD_FONT : &str = "res://assets/fonts/SourceCodePro/SourceCodePro-ChatLightItalicBoldDynaSmaller.tres";


pub const ASTRIX : &str = "[color=#bdbdbd]*******[/color]";


const TALK_STYLE_STANDARD_STANDARD : &str = "says";
const TALK_STYLE_STANDARD_EXCLAIMS : &str = "exclaims";
const TALK_STYLE_STANDARD_SHOUTS : &str = "shouts";
const TALK_STYLE_STANDARD_ASKS : &str = "asks";

const TALK_STYLE_MACHINE_STANDARD : &str = "states";
const TALK_STYLE_MACHINE_EXCLAIMS : &str = "announces";
const TALK_STYLE_MACHINE_SHOUTS : &str = "shouts";
const TALK_STYLE_MACHINE_ASKS : &str = "queries";


const TALK_DATA_STANDARD_I_NEARBY_START : &str = "[i]";
const TALK_DATA_STANDARD_I_NEARBY_END : &str = "[/i]";

const TALK_DATA_STANDARD_I_FURTHER_START : &str = concatcp!("[font=",FURTHER_ITALIC_FONT,"]");
const TALK_DATA_STANDARD_I_FURTHER_END : &str = "[/font]";

const TALK_DATA_STANDARD_I_FAR_START : &str = concatcp!("[font=",FAR_ITALIC_FONT,"]");
const TALK_DATA_STANDARD_I_FAR_END : &str = "[/font]";


const TALK_DATA_STANDARD_B_NEARBY_START : &str = "[b]";
const TALK_DATA_STANDARD_B_NEARBY_END : &str = "[/b]";

const TALK_DATA_STANDARD_B_FURTHER_START : &str = concatcp!("[font=",FURTHER_BOLD_FONT,"]");
const TALK_DATA_STANDARD_B_FURTHER_END : &str = "[/font]";

const TALK_DATA_STANDARD_B_FAR_START : &str = concatcp!("[font=",FAR_BOLD_FONT,"]");
const TALK_DATA_STANDARD_B_FAR_END : &str = "[/font]";


const TALK_DATA_STANDARD_NORMAL_NEARBY_START : &str = "";
const TALK_DATA_STANDARD_NORMAL_NEARBY_END : &str = "";

const TALK_DATA_STANDARD_NORMAL_FURTHER_START : &str = concatcp!("[font=",FURTHER_NORMAL_FONT,"]");
const TALK_DATA_STANDARD_NORMAL_FURTHER_END : &str = "[/font]";

const TALK_DATA_STANDARD_NORMAL_FAR_START : &str = concatcp!("[font=",FAR_NORMAL_FONT,"]");
const TALK_DATA_STANDARD_NORMAL_FAR_END : &str = "[/font]";





const TALK_DATA_MACHINE_I_NEARBY_START : &str = "[i]";
const TALK_DATA_MACHINE_I_NEARBY_END : &str = "[/i]";

const TALK_DATA_MACHINE_I_FURTHER_START : &str = concatcp!("[font=",FURTHER_MACHINE_ITALIC_FONT,"]");
const TALK_DATA_MACHINE_I_FURTHER_END : &str = "[/font]";

const TALK_DATA_MACHINE_I_FAR_START : &str = concatcp!("[font=",FAR_MACHINE_ITALIC_FONT,"]");
const TALK_DATA_MACHINE_I_FAR_END : &str = "[/font]";


const TALK_DATA_MACHINE_B_NEARBY_START : &str = "[b]";
const TALK_DATA_MACHINE_B_NEARBY_END : &str = "[/b]";

const TALK_DATA_MACHINE_B_FURTHER_START : &str = concatcp!("[font=",FURTHER_MACHINE_BOLD_FONT,"]");
const TALK_DATA_MACHINE_B_FURTHER_END : &str = "[/font]";

const TALK_DATA_MACHINE_B_FAR_START : &str = concatcp!("[font=",FAR_MACHINE_BOLD_FONT,"]");
const TALK_DATA_MACHINE_B_FAR_END : &str = "[/font]";


const TALK_DATA_MACHINE_NORMAL_NEARBY_START : &str = "";
const TALK_DATA_MACHINE_NORMAL_NEARBY_END : &str = "";

const TALK_DATA_MACHINE_NORMAL_FURTHER_START : &str = concatcp!("[font=",FURTHER_MACHINE_MEDIUM_FONT,"]");
const TALK_DATA_MACHINE_NORMAL_FURTHER_END : &str = "[/font]";

const TALK_DATA_MACHINE_NORMAL_FAR_START : &str = concatcp!("[font=",FAR_MACHINE_MEDIUM_FONT,"]");
const TALK_DATA_MACHINE_NORMAL_FAR_END : &str = "[/font]";




const SHOUT_DATA_STANDARD_NEARBY_I_START : &str = "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDyna.tres]";
const SHOUT_DATA_STANDARD_NEARBY_I_END : &str = "[/font]";

const SHOUT_DATA_STANDARD_FURTHER_I_START : &str = "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDynaSmall.tres]";
const SHOUT_DATA_STANDARD_FURTHER_I_END : &str = "[/font]";

const SHOUT_DATA_STANDARD_FAR_I_START : &str = "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDynaSmaller.tres]";
const SHOUT_DATA_STANDARD_FAR_I_END : &str = "[/font]";


const SHOUT_DATA_MACHINE_NEARBY_I_START : &str = "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDyna.tres]";
const SHOUT_DATA_MACHINE_NEARBY_I_END : &str = "[/font]";

const SHOUT_DATA_MACHINE_FURTHER_I_START : &str = "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDynaSmall.tres]";
const SHOUT_DATA_MACHINE_FURTHER_I_END : &str = "[/font]";

const SHOUT_DATA_MACHINE_FAR_I_START : &str = "[font=res://assets/fonts/SourceCodePro/SourceCodePro-ChatShoutBoldItalicDynaSmaller.tres]";
const SHOUT_DATA_MACHINE_FAR_I_END : &str = "[/font]";



const TALK_TYPE_STANDARD_NEARBY_START : &str = "";
const TALK_TYPE_STANDARD_NEARBY_END : &str = "";
const _TALK_TYPE_STANDARD_FURTHER_START : &str = "";
const _TALK_TYPE_STANDARD_FURTHER_END : &str = "";
const _TALK_TYPE_STANDARD_FAR_START : &str = "";
const _TALK_TYPE_STANDARD_FAR_END : &str = "";

const TALK_TYPE_MACHINE_NEARBY_START : &str = "[i]";
const TALK_TYPE_MACHINE_NEARBY_END : &str = "[/i]";
const _TALK_TYPE_MACHINE_FURTHER_START : &str = "[i]";
const _TALK_TYPE_MACHINE_FURTHER_END : &str = "[/i]";
const _TALK_TYPE_MACHINE_FAR_START : &str = "[i]";
const _TALK_TYPE_MACHINE_FAR_END : &str = "[/i]";


const TALK_SPACE_OOC_CHATPREFIX : &str = "/ooc"; 

const TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX : &str = "/me";
const TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBSTART : &str = "[color=#dbdbdb]";
const TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBEND : &str = "[/color]";
const TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBSTART : &str = "[color=#e6e6e6]";
const TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBEND : &str = "[/color]";


const _TALK_SPACE_PROXIMITY_CHATPREFIX : &str = "";
const TALK_SPACE_PROXIMITY_PREFIXBBSTART : &str = "[color=#dbdbdb]";
const TALK_SPACE_PROXIMITY_PREFIXBBEND : &str = "[/color]";
const TALK_SPACE_PROXIMITY_MESSAGEBBSTART : &str = "[color=#e6e6e6]";
const TALK_SPACE_PROXIMITY_MESSAGEBBEND : &str = "[/color]";


const TALK_SPACE_COMMON_CHATPREFIX : &str = ";";
const TALK_SPACE_COMMON_PREFIXBBSTART : &str = "[color=#6ce07a]";
const TALK_SPACE_COMMON_PREFIXBBEND : &str = "[/color]";
const TALK_SPACE_COMMON_MESSAGEBBSTART : &str = "[color=#68de77]";
const TALK_SPACE_COMMON_MESSAGEBBEND : &str = "[/color]";


const TALK_SPACE_SECURITY_CHATPREFIX : &str = ":s";
const TALK_SPACE_SECURITY_PREFIXBBSTART : &str = "[color=#f24141]";
const TALK_SPACE_SECURITY_PREFIXBBEND : &str = "[/color]";
const TALK_SPACE_SECURITY_MESSAGEBBSTART : &str = "[color=#fc3d3d]";
const TALK_SPACE_SECURITY_MESSAGEBBEND : &str = "[/color]";


const TALK_SPACE_SPECIALOPS_CHATPREFIX : &str = ".";
const TALK_SPACE_SPECIALOPS_PREFIXBBSTART : &str = "[color=#f24141]";
const TALK_SPACE_SPECIALOPS_PREFIXBBEND : &str = "[/color]";
const TALK_SPACE_SPECIALOPS_MESSAGEBBSTART : &str = "[color=#fc3d3d]";
const TALK_SPACE_SPECIALOPS_MESSAGEBBEND : &str = "[/color]";



const BILLBOARD_DATA_SECURITY_START : &str = "[center][color=#ff7070]";
const BILLBOARD_DATA_SECURITY_END : &str = "[/color][/center]";

const _BILLBOARD_DATA_SPECIALOPS_START : &str = "[center][color=#ff7070]";
const _BILLBOARD_DATA_SPECIALOPS_END : &str = "[/color][/center]";

const TALK_SPACE_COMMON_WORD : &str = "Common";
const TALK_SPACE_SECURITY_WORD : &str = "Security";
const TALK_SPACE_SPECIALOPS_WORD : &str = "Spec-op";

const JOB_SECURITY_WORD : &str = "Security";
const JOB_CONTROL_WORD : &str = "Control";


enum Distance {
    Nearby,
    Further,
    Far
}

enum TalkStyleVariant {
    Standard,
    Shouts,
    Exclaims,
    Asks,
}

pub enum Communicator {
    Standard,
    Machine
}



fn is_shouting(message : &str) -> bool {
    message.ends_with("!!!") ||
    message.ends_with("!!?") || 
    message.ends_with("!?!") ||
    message.ends_with("?!!") ||
    message.ends_with("??!") ||
    message.ends_with("?!?") ||
    message.ends_with("??!") ||
    message.ends_with("!??") ||
    message.ends_with("???")
}

fn is_asking(message : &str) -> bool {
    message.ends_with("?") ||
    message.ends_with("??") ||
    message.ends_with("?!")
}


fn get_talk_space(message : String) -> (RadioChannel, String, bool, bool) {

    let radio_channel;
    let content;
    let mut exclusive_proximity = false;
    let mut is_emote = false;

    if message.starts_with(TALK_SPACE_OOC_CHATPREFIX) {

        radio_channel = RadioChannel::OOC;
        content = message.split(TALK_SPACE_OOC_CHATPREFIX).collect();

    } else if message.starts_with(TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX) {

        radio_channel = RadioChannel::ProximityEmote;
        content = message.split(TALK_SPACE_PROXIMITY_EMOTE_CHATPREFIX).collect();
        exclusive_proximity=true;
        is_emote=true;

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
        exclusive_proximity=true;

    }

    (radio_channel, content, exclusive_proximity, is_emote)

}

pub fn escape_bb(string : String, partially : bool, escape_special_chars : bool) -> String {

    let mut new_string = string.escape_default().to_string();

    new_string = new_string.replace("[", "(");
    new_string = new_string.replace("]", ")");

    if partially {
        if string == "b" || string == "i" || string == "u" ||
        string == "s" ||  string == "code" || string == "center" ||
        string == "right" || string == "fill" || string == "indent" ||
        string == "url" || string == "image" || string == "cell"  ||
        string.contains("url=") || string.contains("img=") ||
        string.contains("font=") || string.contains("color=") || 
        string.contains("table=") {

            new_string="".to_string();

        }
    }

    if escape_special_chars {

        new_string = new_string.replace("`", "").replace( "~", "").replace( "!", "").replace( "@", "").replace( "#", "").replace( "$", "").replace( "%", "")
        .replace( "^", "").replace( "&", "").replace( "*", "").replace( "(", "").replace( ")", "").replace( "-", "").replace( "+", "")
        .replace( "_", "").replace( "{", "").replace( "}", "").replace( "\\", "").replace( "|", "");

    }

    new_string.trim().to_string()

}

pub fn new_ooc_message(
    persistent_player_data_component : &PersistentPlayerData,
    ooc_listeners : &Query<(&ConnectedPlayer, &PersistentPlayerData)>,
    net_new_chat_message_event : &mut EventWriter<NetChatMessage>,
    send_message : String,
) {
    
    let message = persistent_player_data_component.ooc_name.clone() + ": " + &send_message;

    for (connected_player_component , _persistent) in ooc_listeners.iter() {

        if connected_player_component.connected == false {
            continue;
        }

        net_new_chat_message_event.send(NetChatMessage{
            handle: connected_player_component.handle,
            message: ReliableServerMessage::ChatMessage(message.clone()),
        });

    }

}

pub fn new_chat_message(
    net_new_chat_message_event : &mut EventWriter<NetChatMessage>,
    handle_to_entity : &Res<HandleToEntity>,
    sensed_by : &Vec<Entity>,
    sensed_by_distance : &Vec<Entity>,
    position : Vec3,
    name : String,
    job : SpaceJobsEnum,
    mut raw_message : String,
    communicator : Communicator,
    exclusive_radio : bool,
    radio_pawns : &Query<(Entity, &Radio, &RigidBodyPosition, &PersistentPlayerData)>,
    ooc_listeners : &Query<(&ConnectedPlayer, &PersistentPlayerData)>,
    messenger_entity_option : Option<&Entity>,
    mut net_send_entity_updates_option: Option<&mut EventWriter<NetSendEntityUpdates>>,
) {

    raw_message = escape_bb(raw_message, false, false);

    let (
        mut radio_channel
        ,mut message,
         mut exclusive_proximity,
          is_emote
    ) = get_talk_space(raw_message);

    message = escape_bb(message, false, false);

    if matches!(radio_channel, RadioChannel::OOC) {

        match radio_pawns.get(*messenger_entity_option.unwrap()) {
            Ok((_entity, _radio_component, _rigid_body_handle_component, persistent_player_data_component)) => {

                new_ooc_message(
                    persistent_player_data_component,
                    ooc_listeners,
                    net_new_chat_message_event,
                    message,
                );

            },
            Err(_rr) => {
                warn!("Couldnt find components of OOC messenger.");
            },
        }

        
        return;
    }

    if is_emote {

        let result = get_talk_space(message);

        radio_channel = result.0;
        message = result.1;
        exclusive_proximity = result.2;
    

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


    let mut radio_message : String = "".to_string();

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
                talk_space_message_bb_start=TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
                talk_space_message_bb_end=TALK_SPACE_PROXIMITY_MESSAGEBBEND;
            },
            RadioChannel::ProximityEmote => {
                talk_space_prefix_bb_start = TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBSTART;
                talk_space_prefix_bb_end = TALK_SPACE_PROXIMITY_EMOTE_PREFIXBBEND;
                talk_space_message_bb_start=TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBSTART;
                talk_space_message_bb_end=TALK_SPACE_PROXIMITY_EMOTE_MESSAGEBBEND;
            },
            RadioChannel::Common => {
                talk_space_prefix_bb_start = TALK_SPACE_COMMON_PREFIXBBSTART;
                talk_space_word = TALK_SPACE_COMMON_WORD;
                talk_space_prefix_bb_end = TALK_SPACE_COMMON_PREFIXBBEND;
                talk_space_message_bb_start=TALK_SPACE_COMMON_MESSAGEBBSTART;
                talk_space_message_bb_end=TALK_SPACE_COMMON_MESSAGEBBEND;
            },
            RadioChannel::Security => {
                talk_space_prefix_bb_start = TALK_SPACE_SECURITY_PREFIXBBSTART;
                talk_space_word = TALK_SPACE_SECURITY_WORD;
                talk_space_prefix_bb_end = TALK_SPACE_SECURITY_PREFIXBBEND;
                talk_space_message_bb_start=TALK_SPACE_SECURITY_MESSAGEBBSTART;
                talk_space_message_bb_end=TALK_SPACE_SECURITY_MESSAGEBBEND;
            },
            RadioChannel::SpecialOps => {
                talk_space_prefix_bb_start = TALK_SPACE_SPECIALOPS_PREFIXBBSTART; 
                talk_space_word = TALK_SPACE_SPECIALOPS_WORD;
                talk_space_prefix_bb_end = TALK_SPACE_SPECIALOPS_PREFIXBBEND;
                talk_space_message_bb_start=TALK_SPACE_SPECIALOPS_MESSAGEBBSTART;
                talk_space_message_bb_end=TALK_SPACE_SPECIALOPS_MESSAGEBBEND;
            },
            RadioChannel::OOC => {
                warn!("Processing OOC chat while we shouldn't?");
                return;
            },
        }


        let talk_font_nearby_start;
        let talk_font_nearby_end;
        
        let talk_font_nearby_start_1;
        let talk_font_nearby_end_1;
        let talk_style_variation_word;
        match communicator {
            Communicator::Standard => {
                talk_font_nearby_start = TALK_DATA_STANDARD_B_NEARBY_START;
                talk_font_nearby_end=TALK_DATA_STANDARD_B_NEARBY_END;

                
                talk_font_nearby_start_1 = TALK_TYPE_MACHINE_NEARBY_START;
                talk_font_nearby_end_1=TALK_TYPE_MACHINE_NEARBY_END;
                
                match talk_style_variation {
                    TalkStyleVariant::Standard => {talk_style_variation_word=TALK_STYLE_STANDARD_STANDARD;},
                    TalkStyleVariant::Shouts => {talk_style_variation_word=TALK_STYLE_STANDARD_SHOUTS;},
                    TalkStyleVariant::Exclaims => {talk_style_variation_word=TALK_STYLE_STANDARD_EXCLAIMS;},
                    TalkStyleVariant::Asks => {talk_style_variation_word=TALK_STYLE_STANDARD_ASKS;},
                }
            },
            Communicator::Machine => {
                talk_font_nearby_start = TALK_DATA_MACHINE_B_NEARBY_START;
                talk_font_nearby_end=TALK_DATA_MACHINE_B_NEARBY_END;

                talk_font_nearby_start_1 = TALK_TYPE_MACHINE_NEARBY_START;
                talk_font_nearby_end_1=TALK_TYPE_MACHINE_NEARBY_END;
                match talk_style_variation {
                    TalkStyleVariant::Standard => {talk_style_variation_word=TALK_STYLE_MACHINE_STANDARD;},
                    TalkStyleVariant::Shouts => {talk_style_variation_word=TALK_STYLE_MACHINE_SHOUTS;},
                    TalkStyleVariant::Exclaims => {talk_style_variation_word=TALK_STYLE_MACHINE_EXCLAIMS;},
                    TalkStyleVariant::Asks => {talk_style_variation_word=TALK_STYLE_MACHINE_ASKS;},
                }
            },
        } 
        
        

        let rank_word;
        match job {
            SpaceJobsEnum::Security => {rank_word = JOB_SECURITY_WORD;},
            SpaceJobsEnum::Control => {rank_word = JOB_CONTROL_WORD;},
        }


        if is_emote {
            radio_message = radio_message + talk_space_prefix_bb_start + "[" + talk_space_word + "][" + rank_word + "] " + talk_space_prefix_bb_end;
            radio_message = radio_message + talk_space_message_bb_start + talk_font_nearby_start;
            radio_message = radio_message + &message;
            radio_message = radio_message + talk_font_nearby_end + talk_space_message_bb_end;
        } else {
            radio_message = radio_message + talk_space_prefix_bb_start;
            radio_message = radio_message + talk_font_nearby_start;
            radio_message = radio_message + &name + " [" + talk_space_word + "][" + rank_word + "] ";
            radio_message = radio_message + talk_font_nearby_end + talk_space_prefix_bb_end;
            radio_message = radio_message + talk_space_message_bb_start;


            radio_message = radio_message + talk_style_variation_word + ",\n";


            if matches!(talk_style_variation, TalkStyleVariant::Shouts) {

                radio_message = radio_message + talk_font_nearby_start_1 + "[font=" + NEARBY_SHOUT_FONT + "]\"" + &message + "\"[/font]" + talk_font_nearby_end_1;

            } else {

                radio_message = radio_message + talk_font_nearby_start_1 + "\"" + &message + "\"" + talk_font_nearby_end_1;

            }

            radio_message = radio_message + talk_space_message_bb_end;
        }

        
        

    }



    // Build proximity message.
    // For 3 different distances.
    
    let mut proximity_message_nearby = "".to_string();
    let mut proximity_message_further= "".to_string();
    let mut proximity_message_far= "".to_string();
    
    if exclusive_radio == false {

        proximity_message_nearby = proximity_message_nearby + "[font=" + NEARBY_BOLD_FONT + "]" + TALK_SPACE_PROXIMITY_PREFIXBBSTART;
        proximity_message_further = proximity_message_further + "[font=" + FURTHER_BOLD_FONT + "]" + TALK_SPACE_PROXIMITY_PREFIXBBSTART;
        proximity_message_far = proximity_message_far + "[font=" + FAR_BOLD_FONT + "]" + TALK_SPACE_PROXIMITY_PREFIXBBSTART;

        
        let nearby_talk_data_b_end;
        let further_talk_data_b_end;
        let far_talk_data_b_end;

        match communicator {
            Communicator::Standard => {
                proximity_message_nearby = proximity_message_nearby + TALK_DATA_STANDARD_B_NEARBY_START;
                proximity_message_further = proximity_message_further + TALK_DATA_STANDARD_B_FURTHER_START; 
                proximity_message_far = proximity_message_far + TALK_DATA_STANDARD_B_FAR_START;
                
                nearby_talk_data_b_end = TALK_DATA_STANDARD_B_NEARBY_END;
                further_talk_data_b_end = TALK_DATA_STANDARD_B_FURTHER_END;
                far_talk_data_b_end = TALK_DATA_STANDARD_B_FAR_END;
            },
            Communicator::Machine => {
                proximity_message_nearby = proximity_message_nearby + TALK_DATA_MACHINE_B_NEARBY_START;
                proximity_message_further = proximity_message_further + TALK_DATA_MACHINE_B_FURTHER_START; 
                proximity_message_far = proximity_message_far + TALK_DATA_MACHINE_B_FAR_START;

                nearby_talk_data_b_end = TALK_DATA_MACHINE_B_NEARBY_END;
                further_talk_data_b_end = TALK_DATA_MACHINE_B_FURTHER_END;
                far_talk_data_b_end = TALK_DATA_MACHINE_B_FAR_END;
            },
        }

        proximity_message_nearby = proximity_message_nearby + &name + nearby_talk_data_b_end + " ";
        proximity_message_further = proximity_message_further + &name + further_talk_data_b_end + " "; 
        proximity_message_far = proximity_message_far + &name + far_talk_data_b_end + " ";

        let rank_word;
        match job {
            SpaceJobsEnum::Security => {rank_word = JOB_SECURITY_WORD;},
            SpaceJobsEnum::Control => {rank_word = JOB_CONTROL_WORD;},
        }

        if is_emote == false {

            proximity_message_nearby = proximity_message_nearby + "[" + rank_word + "]";
            proximity_message_further = proximity_message_further  + "[" + rank_word + "]"; 
            proximity_message_far = proximity_message_far + "[" + rank_word + "]";

        }

        proximity_message_nearby = proximity_message_nearby + TALK_SPACE_PROXIMITY_PREFIXBBEND + "[/font]" + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
        proximity_message_further = proximity_message_further + TALK_SPACE_PROXIMITY_PREFIXBBEND + "[/font]" + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;
        proximity_message_far = proximity_message_far + TALK_SPACE_PROXIMITY_PREFIXBBEND + "[/font]" + TALK_SPACE_PROXIMITY_MESSAGEBBSTART;



        if is_emote == false {

            let talk_style_variation_word;
            match communicator {
                Communicator::Standard => {
                    proximity_message_nearby = proximity_message_nearby + TALK_DATA_STANDARD_NORMAL_NEARBY_START;
                    proximity_message_further = proximity_message_further + TALK_DATA_STANDARD_NORMAL_FURTHER_START; 
                    proximity_message_far = proximity_message_far + TALK_DATA_STANDARD_NORMAL_FAR_START;

                    match talk_style_variation {
                        TalkStyleVariant::Standard => {talk_style_variation_word = TALK_STYLE_STANDARD_STANDARD;},
                        TalkStyleVariant::Shouts => {talk_style_variation_word = TALK_STYLE_STANDARD_SHOUTS;},
                        TalkStyleVariant::Exclaims => {talk_style_variation_word = TALK_STYLE_STANDARD_EXCLAIMS;},
                        TalkStyleVariant::Asks => {talk_style_variation_word = TALK_STYLE_STANDARD_ASKS;},
                    }

                },
                Communicator::Machine => {
                    proximity_message_nearby = proximity_message_nearby + TALK_DATA_MACHINE_NORMAL_NEARBY_START;
                    proximity_message_further = proximity_message_further + TALK_DATA_MACHINE_NORMAL_FURTHER_START; 
                    proximity_message_far = proximity_message_far + TALK_DATA_MACHINE_NORMAL_FAR_START;

                    match talk_style_variation {
                        TalkStyleVariant::Standard => {talk_style_variation_word = TALK_STYLE_MACHINE_STANDARD;},
                        TalkStyleVariant::Shouts => {talk_style_variation_word = TALK_STYLE_MACHINE_SHOUTS;},
                        TalkStyleVariant::Exclaims => {talk_style_variation_word = TALK_STYLE_MACHINE_EXCLAIMS;},
                        TalkStyleVariant::Asks => {talk_style_variation_word = TALK_STYLE_MACHINE_ASKS;},
                    }

                },
            }

            proximity_message_nearby = proximity_message_nearby + talk_style_variation_word + ",\n";
            proximity_message_further = proximity_message_further + talk_style_variation_word + ",\n"; 
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
                    proximity_message_nearby = proximity_message_nearby + TALK_DATA_STANDARD_NORMAL_NEARBY_END;
                    proximity_message_further = proximity_message_further + TALK_DATA_STANDARD_NORMAL_FURTHER_END; 
                    proximity_message_far = proximity_message_far + TALK_DATA_STANDARD_NORMAL_FAR_END;

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

                },
                Communicator::Machine => {
                    proximity_message_nearby = proximity_message_nearby + TALK_DATA_MACHINE_NORMAL_NEARBY_END;
                    proximity_message_further = proximity_message_further + TALK_DATA_MACHINE_NORMAL_FURTHER_END; 
                    proximity_message_far = proximity_message_far + TALK_DATA_MACHINE_NORMAL_FAR_END;

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
                },
            }

            if exclusive_proximity == false {

                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_i_start;
                proximity_message_further = proximity_message_further + further_talk_data_i_start; 
                proximity_message_far = proximity_message_far + far_talk_data_i_start;

            } else {

                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_normal_start;
                proximity_message_further = proximity_message_further + further_talk_data_normal_start; 
                proximity_message_far = proximity_message_far + far_talk_data_normal_start;

            }

            if matches!(talk_style_variation, TalkStyleVariant::Shouts) {

                if exclusive_proximity == false {

                    proximity_message_nearby = proximity_message_nearby + nearby_shout_data_i_start + &message + nearby_shout_data_i_end;
                    proximity_message_further = proximity_message_further + further_shout_data_i_start + &message + further_shout_data_i_end; 
                    proximity_message_far = proximity_message_far + far_shout_data_i_start + &message + far_shout_data_i_end;

                } else {

                    proximity_message_nearby = proximity_message_nearby + nearby_talk_data_b_start + "[font=" + NEARBY_SHOUT_FONT + "]\"" + &message + "\"[/font]" + nearby_talk_data_b_end;
                    proximity_message_further = proximity_message_further + further_talk_data_b_start + "[font=" + FURTHER_SHOUT_FONT + "]\"" + &message + "\"[/font]" + further_talk_data_b_end;
                    proximity_message_far = proximity_message_far + far_talk_data_b_start + "[font=" + FAR_SHOUT_FONT + "]\"" + &message + "\"[/font]" + far_talk_data_b_end;

                }

            } else {

                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_start + "\"" + &message + "\"" + nearby_talk_data_end;
                proximity_message_further = proximity_message_further + further_talk_data_start + "\"" + &message + "\"" + further_talk_data_end;
                proximity_message_far = proximity_message_far + far_talk_data_start + "\"" + &message + "\"" + far_talk_data_end;


            }

            if exclusive_proximity == false {

                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_i_end;
                proximity_message_further = proximity_message_further + further_talk_data_i_end;
                proximity_message_far = proximity_message_far + far_talk_data_i_end;

            } else {

                proximity_message_nearby = proximity_message_nearby + nearby_talk_data_normal_end;
                proximity_message_further = proximity_message_further + further_talk_data_normal_end;
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
            },
            Communicator::Machine => {
                nearby_talk_data_i_start = TALK_DATA_MACHINE_I_NEARBY_START;
                nearby_talk_data_i_end = TALK_DATA_STANDARD_I_NEARBY_END;

                nearby_talk_data_b_start = TALK_DATA_STANDARD_B_NEARBY_START;
                nearby_talk_data_b_end = TALK_DATA_STANDARD_B_NEARBY_END;
            },
        }

        if exclusive_proximity == false {
            billboard_message = billboard_message + nearby_talk_data_i_start;
        }

        if matches!(talk_style_variation, TalkStyleVariant::Shouts) {
            
            
            billboard_message = billboard_message + nearby_talk_data_b_start;

            if exclusive_proximity == false {
                billboard_message = billboard_message + "[font=" + BILLBOARD_SHOUT_FONT + "]" + &message + "[/font]";
            } else {
                billboard_message = billboard_message + "[font=" + BILLBOARD_SHOUT_ITALIC_FONT + "]" + &message + "[/font]";
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

    let mut handles_direct_proximity : Vec<u32> = vec![];
    let mut handles_radio : Vec<u32> = vec![];

    if exclusive_proximity == false {

        let mut has_radio_permission = false;

        match messenger_entity_option {
            Some(entity) => {
                let messenger_components_result = radio_pawns.get(*entity);

                match messenger_components_result {
                    Ok((_entity, radio_component, _rigid_body_handle_component, _persistent_player_data_component)) => {

                        if radio_component.speak_access.contains(&radio_channel) {

                            has_radio_permission=true;

                        }

                    },
                    Err(_rr) => {return;},
                }
            },
            None => {
                has_radio_permission=true;
            },
        }

        if has_radio_permission {


            for (entity, radio_component, _rigid_body_handle_component, _persistent_player_data_component) in radio_pawns.iter() {

                if radio_component.listen_access.contains(&radio_channel) {

                    let listener_handle_result = handle_to_entity.inv_map.get(&entity);
                    match listener_handle_result {
                        Some(listener_handle) => {
                            
                            net_new_chat_message_event.send(NetChatMessage{
                                handle: *listener_handle,
                                message: ReliableServerMessage::ChatMessage(radio_message.clone()),
                            });

                            handles_radio.push(*listener_handle);

                        },
                        None => {},
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

        parameters_entity_update.insert("billboardMessage".to_string(), EntityUpdateData::String(billboard_message.clone()));

        billboard_entity_update.insert("Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText".to_string(), parameters_entity_update);

        for entity in sensed_by_list {

            let sensed_by_entity_components_result = radio_pawns.get(entity);

            match sensed_by_entity_components_result {
                Ok((entity, _radio_component, rigid_body_position, _persistent_player_data_component)) => {
                    let listener_handle_result = handle_to_entity.inv_map.get(&entity);

                    match listener_handle_result {
                        Some(listener_handle) => {

                    
                            let listener_rigid_body_transform = rigid_body_position.position;
                            let listener_position = Vec3::new(
                                listener_rigid_body_transform.translation.x,
                                listener_rigid_body_transform.translation.y,
                                listener_rigid_body_transform.translation.z
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
                                    net_new_chat_message_event.send(NetChatMessage{
                                        handle: *listener_handle,
                                        message: ReliableServerMessage::ChatMessage(proximity_message_nearby.clone()),
                                    });
                                },
                                Distance::Further => {
                                    net_new_chat_message_event.send(NetChatMessage{
                                        handle: *listener_handle,
                                        message: ReliableServerMessage::ChatMessage(proximity_message_further.clone()),
                                    });
                                },
                                Distance::Far => {
                                    net_new_chat_message_event.send(NetChatMessage{
                                        handle: *listener_handle,
                                        message: ReliableServerMessage::ChatMessage(proximity_message_far.clone()),
                                    });
                                },
                            }
                                
                            

                            match net_send_entity_updates_option {
                                Some(ref mut net_send_entity_updates) => {
                                    
                                    match messenger_entity_option {
                                        Some(messenger_entity) => {
                                            net_send_entity_updates.send(NetSendEntityUpdates {
                                                handle: *listener_handle,
                                                message: ReliableServerMessage::EntityUpdate(messenger_entity.to_bits(), billboard_entity_update.clone(), false)
                                            });
                                            handles_direct_proximity.push(*listener_handle);
                                        },
                                        None => {
                                            error!("Cannot send proximity message without providing messeger_entity.");
                                        },
                                    }
                                },
                                None => {
                                    error!("Cannot send proximity message without providing EventWriter<NetSendEntityUpdates>.");
                                },
                            }
                            
                            

                        },
                        None => {},
                    }

                },
                Err(_rr) => {},
            }

        }

    }


    for player_handle in handles_direct_proximity.iter() {

        net_new_chat_message_event.send(NetChatMessage {
            handle: *player_handle,
            message: PlaySoundProximityMessage::get_message(position),
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
