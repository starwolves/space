use bevy::{
    ecs::schedule::{IntoSystemSetConfigs, SystemSet},
    prelude::{App, Plugin, Update as BevyUpdate},
};

use crate::{
    correction::StartCorrection,
    input::{buffer_input, clear_buffer, InputBuffer, KeyBinds},
    modes::is_server_mode,
    ordering::{BuildingSet, PostUpdate, PreUpdate},
    ui::MainMenuState,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if !is_server_mode(app) {
            app.init_resource::<KeyBinds>()
                .init_resource::<InputBuffer>()
                .add_systems(BevyUpdate, buffer_input)
                //.add_systems(BevyUpdate, sanitize_input.before(buffer_input))
                .add_systems(PostUpdate, clear_buffer)
                .init_resource::<MainMenuState>()
                .add_event::<StartCorrection>()
                .configure_sets(
                    PreUpdate,
                    (
                        BuildingSet::RawTriggerBuild,
                        BuildingSet::TriggerBuild,
                        SpawnItemSet::SpawnHeldItem,
                        BuildingSet::NormalBuild,
                    )
                        .chain(),
                );
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpawnItemSet {
    SpawnHeldItem,
    AddingComponent,
}
