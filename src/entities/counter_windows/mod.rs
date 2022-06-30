use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemLabel, SystemSet};
use bevy_ecs::system::ResMut;
use bevy_math::Quat;
use bevy_transform::components::Transform;

use crate::core::entity::functions::initialize_entity_data::initialize_entity_data;
use crate::core::entity::resources::{EntityDataProperties, EntityDataResource, GridItemData};
use crate::core::entity::spawn::{summon_base_entity, SpawnEvent};
use crate::core::rigid_body::spawn::summon_rigid_body;
use crate::core::tab_actions::TabActionsQueueLabels;
use crate::core::{PostUpdateLabels, StartupLabels, SummoningLabels};

use self::events::{CounterWindowUnlock, NetCounterWindow};
use self::spawn::{
    default_summon_counter_window, summon_counter_window, summon_raw_counter_window,
    CounterWindowSummoner, BRIDGE_COUNTER_WINDOW_ENTITY_NAME, SECURITY_COUNTER_WINDOW_ENTITY_NAME,
};
use self::systems::actions::actions;
use self::systems::counter_window_added::counter_window_added;
use self::systems::counter_window_default_map_added::counter_window_default_map_added;
use self::systems::counter_window_events::counter_window_events;
use self::systems::counter_window_tick_timers::counter_window_tick_timers;
use self::systems::net_system::net_system;
use self::{
    entity_update::counter_window_update,
    events::{
        CounterWindowLockClosed, CounterWindowLockOpen, CounterWindowSensorCollision,
        InputCounterWindowToggleOpen,
    },
};

pub mod components;
pub mod entity_update;
pub mod events;
pub mod functions;
pub mod spawn;
pub mod systems;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CounterWindowTimers {
    Timer,
}
pub struct CounterWindowsPlugin;

impl Plugin for CounterWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CounterWindowSensorCollision>()
            .add_system(counter_window_tick_timers.label(CounterWindowTimers::Timer))
            .add_system(counter_window_events.after(CounterWindowTimers::Timer))
            .add_system(counter_window_added)
            .add_system(counter_window_default_map_added)
            .add_event::<InputCounterWindowToggleOpen>()
            .add_event::<CounterWindowLockOpen>()
            .add_event::<CounterWindowLockClosed>()
            .add_event::<CounterWindowUnlock>()
            .add_event::<NetCounterWindow>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(counter_window_update),
            )
            .add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
            .add_system(actions.after(TabActionsQueueLabels::TabAction))
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            )
            .add_system(
                summon_counter_window::<CounterWindowSummoner>
                    .after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_base_entity::<CounterWindowSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_rigid_body::<CounterWindowSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_raw_counter_window).after(SummoningLabels::TriggerSummon))
            .add_event::<SpawnEvent<CounterWindowSummoner>>()
            .add_system((default_summon_counter_window).after(SummoningLabels::DefaultSummon));
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let mut transform = Transform::identity();
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    let entity_properties = EntityDataProperties {
        name: SECURITY_COUNTER_WINDOW_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["securityCounter1".to_string()],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let mut transform = Transform::identity();
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    let entity_properties = EntityDataProperties {
        name: BRIDGE_COUNTER_WINDOW_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["bridgeCounter".to_string()],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
