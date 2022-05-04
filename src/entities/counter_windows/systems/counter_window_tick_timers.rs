use bevy_core::Time;
use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query, Res, ResMut},
};

use crate::{
    core::sfx::resources::SfxAutoDestroyTimers,
    entities::counter_windows::components::CounterWindow,
};

pub fn counter_window_tick_timers(
    mut counter_windows: Query<&mut CounterWindow>,
    time: Res<Time>,

    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
) {
    for mut counter_window_component in counter_windows.iter_mut() {
        counter_window_component.timer.tick(time.delta());
        counter_window_component.denied_timer.tick(time.delta());
        counter_window_component.open_timer.tick(time.delta());
        counter_window_component.closed_timer.tick(time.delta());
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
