use api::network::ReliableServerMessage;

use crate::builder::get_random_pitch_scale;

pub struct PlaySoundRadioMessage;

impl PlaySoundRadioMessage {
    pub fn get_message() -> ReliableServerMessage {
        ReliableServerMessage::PlaySound(
            "/content/audio/chat/radio_message.sample".to_string(),
            -24.,
            get_random_pitch_scale(1.),
            None,
        )
    }
}
