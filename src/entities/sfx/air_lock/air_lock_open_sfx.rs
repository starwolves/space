use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct AirLockOpenSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 4.5 + 1.;

impl AirLockOpenSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 13.,
                stream_id: "/content/audio/airLock/doorOpen.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.6),
                ..Default::default()
            },))
            .id()
    }
}
