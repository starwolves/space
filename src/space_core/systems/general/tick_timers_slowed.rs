use std::time::Duration;

use bevy::{prelude::{Query}};

use crate::space_core::components::{ambience_sfx_timer::AmbienceSfxTimer, sfx::Sfx};


pub fn tick_timers_slowed(
    mut query_ambience_sfx_timer : Query<(&mut AmbienceSfxTimer, &mut Sfx)>,
) {

    for (mut timer_component, mut sfx_component) in query_ambience_sfx_timer.iter_mut() {
        if timer_component.timer.tick(Duration::from_millis(500)).just_finished() {

            sfx_component.sfx_replay=true;

            timer_component.timer.pause();
            timer_component.timer.reset();
            timer_component.timer.unpause();

        } else {
            // This will sync the audio, but this currently causes a constant entityUpdate spam of the ambient sfx.
            sfx_component.play_back_position = timer_component.timer.elapsed_secs();
        }
    }

}
