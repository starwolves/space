use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::sfx::components::Sfx;

pub struct CounterWindowDeniedSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1. + 1.;

impl CounterWindowDeniedSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 20.,
                stream_id: "/content/audio/counterWindow/windowAccessDenied.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                ..Default::default()
            },))
            .id()
    }
}
