use bevy::{core::{Time, Timer}, prelude::{Commands, Entity, Query, Res, ResMut, info}};

use crate::space_core::{components::{air_lock_closed_timer::AirLockClosedTimer, air_lock_denied_timer::AirLockDeniedTimer, air_lock_open_timer::AirLockOpenTimer}, resources::sfx_auto_destroy_timers::SfxAutoDestroyTimers};

pub fn tick_timers(
    time: Res<Time>, 
    mut query_timer: Query<&mut Timer>,
    mut query_air_lock_denied_timer: Query<&mut AirLockDeniedTimer>,
    mut query_air_lock_open_timer: Query<&mut AirLockOpenTimer>,
    mut query_air_lock_closed_timer: Query<&mut AirLockClosedTimer>,
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



}
