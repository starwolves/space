use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use api::{combat::ProjectileFOV, data::UpdateLabels};

use crate::attack::Attack;

use super::attack::attack;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack.after(UpdateLabels::StandardCharacters))
            .add_event::<Attack>()
            .add_event::<ProjectileFOV>();
    }
}
