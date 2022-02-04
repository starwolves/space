use bevy::math::Vec3;
use rand::Rng;

use crate::space_core::{ecs::{sfx::components::get_random_pitch_scale, networking::resources::ReliableServerMessage}};

pub struct PlaySoundProximityMessage;

impl PlaySoundProximityMessage {
    pub fn get_message(position : Vec3) ->  ReliableServerMessage {

        let mut rng = rand::thread_rng();

        let random_index = rng.gen_range(0..SFX_NAMES.len());

        ReliableServerMessage::PlaySound(SFX_NAMES[random_index].to_string(), 1., get_random_pitch_scale(1.), Some(position))

    }
}



const SFX_NAMES : [&str;6] = [
    "proximity_message1",
    "proximity_message2",
    "proximity_message3",
    "proximity_message4",
    "proximity_message5",
    "proximity_message6",
];
