use bevy::{core::{Time, Timer}, prelude::{Commands, Entity, EventWriter, Query, Res, ResMut}};

use crate::space_core::{components::{air_lock_closed_timer::AirLockClosedTimer, air_lock_denied_timer::AirLockDeniedTimer, air_lock_open_timer::AirLockOpenTimer, counter_window_closed_timer::CounterWindowClosedTimer, counter_window_denied_timer::CounterWindowDeniedTimer, counter_window_open_timer::CounterWindowOpenTimer, sensable::Sensable, sfx::Sfx}, events::net::net_unload_entity::NetUnloadEntity, resources::{handle_to_entity::HandleToEntity, sfx_auto_destroy_timers::SfxAutoDestroyTimers}};

pub fn tick_timers(
    time: Res<Time>, 
    mut query_timer: Query<&mut Timer>,
    mut query_air_lock_denied_timer: Query<&mut AirLockDeniedTimer>,
    mut query_air_lock_open_timer: Query<&mut AirLockOpenTimer>,
    mut query_air_lock_closed_timer: Query<&mut AirLockClosedTimer>,
    mut query_counter_window_open_timer: Query<&mut CounterWindowOpenTimer>,
    mut query_counter_window_denied_timer: Query<&mut CounterWindowDeniedTimer>,
    mut query_counter_window_closed_timer: Query<&mut CounterWindowClosedTimer>,
    
    
    mut sfx_auto_destroy_timers : ResMut<SfxAutoDestroyTimers>,
    mut sfx_entities : Query<(&Sfx, &mut Sensable)>,
    handle_to_entity : Res<HandleToEntity>,
    mut net_unload_entity : EventWriter<NetUnloadEntity>,
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

        sfx_entities.get_mut(expired_sfx_entity).unwrap().1.despawn(expired_sfx_entity, &mut net_unload_entity, &handle_to_entity);

        commands.entity(expired_sfx_entity).despawn();
    }


    


}
