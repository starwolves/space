use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};
use bevy_ecs::system::ResMut;
use bevy_math::Quat;
use bevy_transform::components::Transform;

use crate::space::core::entity::functions::initialize_entity_data::initialize_entity_data;
use crate::space::core::entity::resources::{
    EntityDataProperties, EntityDataResource, GridItemData,
};
use crate::space::core::tab_actions::TabActionsQueueLabels;
use crate::space::{PostUpdateLabels, StartupLabels};

use self::events::{counter_windows_actions, CounterWindowUnlock};
use self::spawn::CounterWindowBundle;
use self::{
    entity_update::counter_window_update,
    events::{
        CounterWindowLockClosed, CounterWindowLockOpen, CounterWindowSensorCollision,
        InputCounterWindowToggleOpen,
    },
    systems::{
        counter_window_added, counter_window_default_map_added, counter_window_events,
        counter_window_tick_timers,
    },
};

pub mod components;
pub mod entity_update;
pub mod events;
pub mod functions;
pub mod spawn;
pub mod systems;

pub struct CounterWindowsPlugin;

impl Plugin for CounterWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CounterWindowSensorCollision>()
            .add_system(counter_window_events)
            .add_system(counter_window_tick_timers)
            .add_system(counter_window_added)
            .add_system(counter_window_default_map_added)
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
            .add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
            .add_system(counter_windows_actions.after(TabActionsQueueLabels::TabAction));
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let mut transform = Transform::identity();
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    let entity_properties = EntityDataProperties {
        name: "securityCounterWindow".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(CounterWindowBundle::spawn),
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
        name: "bridgeCounterWindow".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(CounterWindowBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["bridgeCounter".to_string()],
        }),
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
