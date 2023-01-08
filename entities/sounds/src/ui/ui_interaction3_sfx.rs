use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct UIInteraction3SfxBundle;

pub const UI_INTERACTION3_PLAY_BACK_DURATION: f32 = 2.4 + 1.;

impl UIInteraction3SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "ui_interaction3",
                play_back_duration: UI_INTERACTION3_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
