use std::time::Duration;

use bevy_ecs::{
    prelude::{Commands, Entity, ResMut},
    system::Query,
};

use super::{
    components::{AmbienceSfxTimer, Sfx},
    resources::SfxAutoDestroyTimers,
};

pub fn tick_timers_slowed(mut query_ambience_sfx_timer: Query<(&mut AmbienceSfxTimer, &mut Sfx)>) {
    for (mut timer_component, mut sfx_component) in query_ambience_sfx_timer.iter_mut() {
        if timer_component
            .timer
            .tick(Duration::from_millis(500))
            .just_finished()
        {
            sfx_component.sfx_replay = true;

            timer_component.timer.pause();
            timer_component.timer.reset();
            timer_component.timer.unpause();
        } else {
            // This will sync the audio, but this currently causes a constant entityUpdate spam of the ambient sfx.
            sfx_component.play_back_position = timer_component.timer.elapsed_secs();
        }
    }
}

pub fn free_sfx(mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>, mut commands: Commands) {
    let mut expired_sfx_entities: Vec<Entity> = vec![];

    for (sfx_entity, incremental) in &mut sfx_auto_destroy_timers.timers {
        *incremental += 1;
        if incremental >= &mut 2 {
            expired_sfx_entities.push(*sfx_entity);
        }
    }

    for i in 0..expired_sfx_entities.len() {
        let this_entity_id = expired_sfx_entities[i];

        let mut j = 0;
        for (sfx_entity, _timer) in &mut sfx_auto_destroy_timers.timers {
            if this_entity_id == *sfx_entity {
                break;
            }
            j += 1;
        }

        sfx_auto_destroy_timers.timers.remove(j);

        commands.entity(this_entity_id).despawn();
    }
}
