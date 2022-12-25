use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use combat::sfx::health_combat_hit_result_sfx;
use entity::register::register_entity_type;
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
    physics_events::physics_events,
};

use super::{
    counter_window_added::{counter_window_added, counter_window_default_map_added},
    counter_window_events::{
        counter_window_events, CounterWindowLockClosed, CounterWindowLockOpen,
        CounterWindowSensorCollision, CounterWindowUnlock, InputCounterWindowToggleOpen,
    },
    counter_window_tick_timers::counter_window_tick_timers,
    entity_update::counter_window_update,
    spawn::{build_counter_windows, build_raw_counter_windows, CounterWindowType},
};
use crate::actions::build_actions;
use bevy::app::CoreStage::PostUpdate;
pub struct CounterWindowsPlugin;

impl Plugin for CounterWindowsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<CounterWindowSensorCollision>()
                .add_system(counter_window_tick_timers)
                .add_system(counter_window_events)
                .add_system(counter_window_added)
                .add_system(counter_window_default_map_added)
                .add_system(physics_events)
                .add_event::<InputCounterWindowToggleOpen>()
                .add_event::<CounterWindowLockOpen>()
                .add_event::<CounterWindowLockClosed>()
                .add_event::<CounterWindowUnlock>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(counter_window_update),
                )
                .add_system(
                    health_combat_hit_result_sfx::<CounterWindow>
                        .after(CombatLabels::FinalizeApplyDamage),
                )
                .add_system(
                    toggle_open_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    lock_open_action_prequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Build),
                )
                .add_system(
                    counter_window_actions
                        .label(ActionsLabels::Action)
                        .after(ActionsLabels::Approve),
                )
                .add_system(
                    build_actions
                        .label(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                );
        }
        register_entity_type::<CounterWindowType>(app);
        app.add_system(
            build_counter_windows::<CounterWindowType>.after(BuildingLabels::TriggerBuild),
        )
        .add_system((build_base_entities::<CounterWindowType>).after(BuildingLabels::TriggerBuild))
        .add_system((build_rigid_bodies::<CounterWindowType>).after(BuildingLabels::TriggerBuild))
        .add_system((build_raw_counter_windows).after(BuildingLabels::TriggerBuild));
    }
}
