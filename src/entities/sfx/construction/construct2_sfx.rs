use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct Construct2SfxBundle;

pub const CONSTRUCT2_PLAY_BACK_DURATION: f32 = 0.7 + 1.;

impl Construct2SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 11.,
                unit_size: 1.,
                stream_id: "/content/audio/construction/construct2.sample".to_string(),
                play_back_duration: CONSTRUCT2_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
