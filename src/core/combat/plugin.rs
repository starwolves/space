use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};

use crate::core::space_plugin::plugin::UpdateLabels;

use super::attack::attack;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack.after(UpdateLabels::StandardCharacters));
    }
}
