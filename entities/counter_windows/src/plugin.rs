use bevy::prelude::{App, IntoSystemConfigs, Plugin, PostUpdate, Update};
use combat::sfx::health_combat_hit_result_sfx;
use entity::entity_types::register_entity_type;
use entity::spawn::build_base_entities;
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    labels::{ActionsLabels, BuildingLabels, CombatLabels, PostUpdateLabels},
};

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
    entity_update::counter_window_update,
    spawn::{build_counter_windows, CounterWindowType},
};
use crate::actions::build_actions;
pub struct CounterWindowsPlugin;

impl Plugin for CounterWindowsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<CounterWindowSensorCollision>()
                .add_systems(
                    Update,
                    (
                        counter_window_tick_timers,
                        counter_window_default_map_added,
                        counter_window_events,
                        health_combat_hit_result_sfx::<CounterWindow>
                            .after(CombatLabels::FinalizeApplyDamage),
                        toggle_open_action_prequisite_check
                            .in_set(ActionsLabels::Approve)
                            .after(ActionsLabels::Build),
                        lock_open_action_prequisite_check
                            .in_set(ActionsLabels::Approve)
                            .after(ActionsLabels::Build),
                        counter_window_actions
                            .in_set(ActionsLabels::Action)
                            .after(ActionsLabels::Approve),
                        build_actions
                            .in_set(ActionsLabels::Build)
                            .after(ActionsLabels::Init),
                    ),
                )
                .add_event::<InputCounterWindowToggleOpen>()
                .add_event::<CounterWindowLockOpen>()
                .add_event::<CounterWindowLockClosed>()
                .add_event::<CounterWindowUnlock>()
                .add_systems(
                    PostUpdate,
                    counter_window_update.in_set(PostUpdateLabels::EntityUpdate),
                );
        }
        register_entity_type::<CounterWindowType>(app);
        app.add_systems(
            Update,
            (
                build_counter_windows::<CounterWindowType>.after(BuildingLabels::TriggerBuild),
                (build_rigid_bodies::<CounterWindowType>).after(BuildingLabels::TriggerBuild),
                (build_base_entities::<CounterWindowType>).after(BuildingLabels::TriggerBuild),
            ),
        );
    }
}
