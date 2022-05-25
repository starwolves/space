use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct UIInteraction1SfxBundle;

pub const UI_INTERACTION1_PLAY_BACK_DURATION: f32 = 1.3 + 1.;

impl UIInteraction1SfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 15.,
            unit_size: 1.,
            stream_id: "/content/audio/ui_interactions/ui_interaction1.sample".to_string(),
            play_back_duration: UI_INTERACTION1_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },));
        commands
    }
}
