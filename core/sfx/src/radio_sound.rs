use crate::{builder::get_random_pitch_scale, net::SfxServerMessage};

/// Play radio sound message data.

pub struct PlaySoundRadioMessage;

impl PlaySoundRadioMessage {
    pub fn get_message() -> SfxServerMessage {
        SfxServerMessage::PlaySound(
            "/content/audio/chat/radio_message.sample".to_string(),
            -24.,
            get_random_pitch_scale(1.),
            None,
        )
    }
}
