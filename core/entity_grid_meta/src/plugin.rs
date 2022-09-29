use bevy::prelude::{App, Plugin};

use crate::core::EntityDataResource;

pub struct EntityGridMetaPlugin;
impl Plugin for EntityGridMetaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityDataResource>();
    }
}
