use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, IntoSystemSetConfigs, Plugin, PreUpdate};
use bevy_xpbd_3d::PhysicsSet;

use crate::{
    binds::KeyBinds,
    input::{buffer_input, clear_buffer, InputBuffer},
    is_server::is_server,
    sets::MainSet,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.init_resource::<KeyBinds>()
                .init_resource::<InputBuffer>()
                .add_systems(PreUpdate, buffer_input)
                .add_systems(FixedUpdate, clear_buffer.in_set(MainSet::PostUpdate));
        }

        app.configure_sets(
            FixedUpdate,
            (MainSet::PreUpdate, MainSet::Update, MainSet::PostUpdate)
                .chain()
                .before(PhysicsSet::Prepare),
        );
    }
}
