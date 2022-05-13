use bevy_core::Time;
use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query, Res, ResMut},
};

use crate::{core::sfx::resources::SfxAutoDestroyTimers, entities::air_locks::components::AirLock};

pub fn air_lock_tick_timers(
    time: Res<Time>,
    mut air_locks: Query<&mut AirLock>,

    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
) {
    for mut air_lock_component in air_locks.iter_mut() {
        match air_lock_component.denied_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
        match air_lock_component.open_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
        match air_lock_component.closed_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
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
