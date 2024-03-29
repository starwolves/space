use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use combat::sfx::health_combat_hit_result_sfx;
use entity::entity_types::register_entity_type;
use entity::spawn::build_base_entities;
use physics::spawn::build_rigid_bodies;
use resources::modes::is_server_mode;
use resources::ordering::{ActionsSet, BuildingSet, CombatSet, PreUpdate, Update};

use crate::{
    actions::{
        counter_window_actions, lock_open_action_prequisite_check,
        toggle_open_action_prequisite_check,
    },
    counter_window_events::CounterWindow,
};

use super::{
    counter_window_added::counter_window_default_map_added,
    counter_window_events::{
        counter_window_events, CounterWindowLockClosed, CounterWindowLockOpen,
        CounterWindowSensorCollision, CounterWindowUnlock, InputCounterWindowToggleOpen,
    },
    counter_window_tick_timers::counter_window_tick_timers,
    spawn::{build_counter_windows, CounterWindowType},
};
use crate::actions::build_actions;
pub struct CounterWindowsPlugin;

impl Plugin for CounterWindowsPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_event::<CounterWindowSensorCollision>()
                .add_systems(
                    Update,
                    (
                        counter_window_tick_timers,
                        counter_window_default_map_added,
                        counter_window_events,
                        health_combat_hit_result_sfx::<CounterWindow>
                            .after(CombatSet::FinalizeApplyDamage),
                        toggle_open_action_prequisite_check
                            .in_set(ActionsSet::Approve)
                            .after(ActionsSet::Build),
                        lock_open_action_prequisite_check
                            .in_set(ActionsSet::Approve)
                            .after(ActionsSet::Build),
                        counter_window_actions
                            .in_set(ActionsSet::Action)
                            .after(ActionsSet::Approve),
                        build_actions
                            .in_set(ActionsSet::Build)
                            .after(ActionsSet::Init),
                    ),
                )
                .add_event::<InputCounterWindowToggleOpen>()
                .add_event::<CounterWindowLockOpen>()
                .add_event::<CounterWindowLockClosed>()
                .add_event::<CounterWindowUnlock>();
        }
        register_entity_type::<CounterWindowType>(app);
        app.add_systems(
            PreUpdate,
            (
                build_counter_windows::<CounterWindowType>.in_set(BuildingSet::NormalBuild),
                (build_rigid_bodies::<CounterWindowType>).in_set(BuildingSet::NormalBuild),
                (build_base_entities::<CounterWindowType>).in_set(BuildingSet::NormalBuild),
            ),
        );
    }
}
