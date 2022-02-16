use crate::space::core::{
    networking::resources::ReliableServerMessage, sfx::components::get_random_pitch_scale,
};

pub struct PlaySoundRadioMessage;

impl PlaySoundRadioMessage {
    pub fn get_message() -> ReliableServerMessage {
        ReliableServerMessage::PlaySound(
            "radio_message".to_string(),
            -24.,
            get_random_pitch_scale(1.),
            None,
        )
    }
}
