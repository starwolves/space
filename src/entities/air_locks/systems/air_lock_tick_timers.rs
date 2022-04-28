use bevy_core::{Time, Timer};
use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query, Res, ResMut},
};

use crate::{
    core::sfx::resources::SfxAutoDestroyTimers,
    entities::air_locks::components::{AirLockClosedTimer, AirLockDeniedTimer, AirLockOpenTimer},
};

pub fn air_lock_tick_timers(
    time: Res<Time>,
    mut query_timer: Query<&mut Timer>,
    mut query_air_lock_denied_timer: Query<&mut AirLockDeniedTimer>,
    mut query_air_lock_open_timer: Query<&mut AirLockOpenTimer>,
    mut query_air_lock_closed_timer: Query<&mut AirLockClosedTimer>,

    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
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
