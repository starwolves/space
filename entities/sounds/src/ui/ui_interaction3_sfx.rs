use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct UIInteraction3SfxBundle;

#[cfg(feature = "server")]
pub const UI_INTERACTION3_PLAY_BACK_DURATION: f32 = 2.4 + 1.;

#[cfg(feature = "server")]
impl UIInteraction3SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: "/content/audio/ui_interactions/ui_interaction3.sample".to_string(),
                play_back_duration: UI_INTERACTION3_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
