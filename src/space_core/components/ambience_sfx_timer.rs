use bevy::{core::Timer, prelude::Component};

use crate::space_core::bundles::ambience_sfx::AMBIENCE_SFX_PLAY_BACK_DURATION;
#[derive(Component)]
pub struct AmbienceSfxTimer {
    pub timer : Timer
}

impl Default for AmbienceSfxTimer {
    fn default() -> Self {
        Self {
            timer : Timer::from_seconds(AMBIENCE_SFX_PLAY_BACK_DURATION, false),
        }
    }
}
