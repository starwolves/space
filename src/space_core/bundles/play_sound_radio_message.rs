use crate::space_core::{components::sfx::get_random_pitch_scale, resources::network_messages::ReliableServerMessage};

pub struct PlaySoundRadioMessage;

impl PlaySoundRadioMessage {
    pub fn get_message() ->  ReliableServerMessage {

        ReliableServerMessage::PlaySound("radio_message".to_string(), -24., get_random_pitch_scale(1.), None)

    }
}
