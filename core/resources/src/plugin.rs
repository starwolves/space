use bevy::{
    prelude::{
        App, FixedUpdate, IntoSystemConfigs, IntoSystemSetConfigs, Plugin, PreUpdate, Update,
    },
    transform::TransformSystem,
};
use bevy_xpbd_3d::PhysicsSet;

use crate::{
    correction::StartCorrection,
    input::{buffer_input, clear_buffer, sanitize_input, InputBuffer, KeyBinds},
    modes::is_server_mode,
    sets::MainSet,
    ui::MainMenuState,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if !is_server_mode(app) {
            app.init_resource::<KeyBinds>()
                .init_resource::<InputBuffer>()
                .add_systems(Update, buffer_input.in_set(MainSet::PreUpdate))
                .add_systems(
                    FixedUpdate,
                    sanitize_input
                        .after(buffer_input)
                        .in_set(MainSet::PreUpdate),
                )
                .add_systems(FixedUpdate, clear_buffer.in_set(MainSet::PostUpdate))
                .init_resource::<MainMenuState>()
                .add_event::<StartCorrection>();
        }
        let sets = (
            MainSet::PreUpdate,
            MainSet::Update,
            MainSet::PostUpdate,
            PhysicsSet::Prepare,
            PhysicsSet::Sync,
            MainSet::PostPhysics,
        );
        app.configure_sets(FixedUpdate, sets.clone().chain())
            .configure_sets(
                Update,
                sets.chain().before(TransformSystem::TransformPropagate),
            );
    }
}
