use api::data::ActionsLabels;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};

use crate::examine_events::examine_entity;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(examine_entity.after(ActionsLabels::Action));
    }
}
