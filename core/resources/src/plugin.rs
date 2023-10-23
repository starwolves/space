use bevy::prelude::{
    App, FixedUpdate, IntoSystemConfigs, IntoSystemSetConfigs, Plugin, PreUpdate, Update,
};
use bevy_xpbd_3d::PhysicsSet;

use crate::{
    input::{buffer_input, clear_buffer, InputBuffer, KeyBinds},
    is_server::is_server,
    sets::MainSet,
    ui::MainMenuState,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.init_resource::<KeyBinds>()
                .init_resource::<InputBuffer>()
                .add_systems(PreUpdate, buffer_input)
                .add_systems(FixedUpdate, clear_buffer.in_set(MainSet::PostUpdate))
                .init_resource::<MainMenuState>();
        }

        app.configure_sets(
            FixedUpdate,
            (
                MainSet::PreUpdate,
                MainSet::Update,
                MainSet::PostUpdate,
                PhysicsSet::Prepare,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                MainSet::PreUpdate,
                MainSet::Update,
                MainSet::PostUpdate,
                PhysicsSet::Prepare,
            )
                .chain(),
        );
    }
}
