use api::network::ReliableServerMessage;
use bevy::math::Vec3;
use rand::Rng;

use crate::builder::get_random_pitch_scale;

/// Proximity message sound data.
pub struct PlaySoundProximityMessageData;

impl PlaySoundProximityMessageData {
    pub fn get_message(position: Vec3) -> ReliableServerMessage {
        let mut rng = rand::thread_rng();

        let random_index = rng.gen_range(0..SFX_NAMES.len());

        ReliableServerMessage::PlaySound(
            SFX_NAMES[random_index].to_string(),
            1.,
            get_random_pitch_scale(1.),
            Some(position),
        )
    }
}

/// All proximity message sound effect ids.
const SFX_NAMES: [&str; 6] = [
    "/content/audio/chat/proximity_message1.sample",
    "/content/audio/chat/proximity_message2.sample",
    "/content/audio/chat/proximity_message3.sample",
    "/content/audio/chat/proximity_message4.sample",
    "/content/audio/chat/proximity_message5.sample",
    "/content/audio/chat/proximity_message6.sample",
];
