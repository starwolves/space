use bevy::{core::{Time, Timer}, prelude::{Commands, Entity, Query, Res, ResMut}};

use crate::space_core::{components::{air_lock_closed_timer::AirLockClosedTimer, air_lock_denied_timer::AirLockDeniedTimer, air_lock_open_timer::AirLockOpenTimer, ambience_sfx_timer::AmbienceSfxTimer, counter_window_closed_timer::CounterWindowClosedTimer, counter_window_denied_timer::CounterWindowDeniedTimer, counter_window_open_timer::CounterWindowOpenTimer, sfx::Sfx}, resources::sfx_auto_destroy_timers::SfxAutoDestroyTimers};

pub fn tick_timers(
    time: Res<Time>, 
    mut query_timer: Query<&mut Timer>,
    mut query_air_lock_denied_timer: Query<&mut AirLockDeniedTimer>,
    mut query_air_lock_open_timer: Query<&mut AirLockOpenTimer>,
    mut query_air_lock_closed_timer: Query<&mut AirLockClosedTimer>,
    mut query_counter_window_open_timer: Query<&mut CounterWindowOpenTimer>,
    mut query_counter_window_denied_timer: Query<&mut CounterWindowDeniedTimer>,
    mut query_counter_window_closed_timer: Query<&mut CounterWindowClosedTimer>,
    mut query_ambience_sfx_timer : Query<(&mut AmbienceSfxTimer, &mut Sfx)>,
    
    mut sfx_auto_destroy_timers : ResMut<SfxAutoDestroyTimers>,
    mut commands : Commands
) {
    for mut timer in query_timer.iter_mut() {
        timer.tick(time.delta());
    }
    for mut timer in query_air_lock_denied_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_air_lock_open_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_air_lock_closed_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_counter_window_open_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_counter_window_denied_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_counter_window_closed_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }

    let mut expired_sfx_entities : Vec<Entity> =  vec![];
    let mut expired_sfx_entities_i : usize = 0;

    for (sfx_entity, timer) in &mut sfx_auto_destroy_timers.timers {

        if timer.tick(time.delta()).just_finished() {

            
            expired_sfx_entities.insert(expired_sfx_entities_i, *sfx_entity);
            expired_sfx_entities_i+=1;

        }

    }

    for expired_sfx_entity in expired_sfx_entities {

        sfx_auto_destroy_timers.timers.remove(&expired_sfx_entity);
        commands.entity(expired_sfx_entity).despawn();
    }


    for (mut timer_component, mut sfx_component) in query_ambience_sfx_timer.iter_mut() {
        if timer_component.timer.tick(time.delta()).just_finished() {

            sfx_component.sfx_replay=true;

            timer_component.timer.pause();
            timer_component.timer.reset();
            timer_component.timer.unpause();

        } else {
            // This will sync the audio, but this currently causes a constant entityUpdate spam of the ambient sfx.
            //sfx_component.play_back_position = timer_component.timer.elapsed_secs();
        }
    }


}
