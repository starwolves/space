use bevy_app::{App, Plugin};

use self::systems::attack;

use super::plugin::UpdateLabels;

pub mod systems;

use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack.after(UpdateLabels::StandardCharacters));
    }
}
