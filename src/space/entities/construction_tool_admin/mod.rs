use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;

use crate::space::UpdateLabels;

use self::{
    events::{
        InputConstruct, InputConstructionOptions, InputConstructionOptionsSelection,
        InputDeconstruct, NetConstructionTool,
    },
    systems::construction_tool,
};

pub mod components;
pub mod events;
pub mod spawn;
pub mod systems;

pub struct ConstructionToolAdminPlugin;

impl Plugin for ConstructionToolAdminPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputConstruct>()
            .add_event::<InputDeconstruct>()
            .add_event::<InputConstructionOptions>()
            .add_event::<NetConstructionTool>()
            .add_event::<InputConstructionOptionsSelection>()
            .add_system(
                construction_tool
                    .after(UpdateLabels::TextTreeInputSelection)
                    .before(UpdateLabels::DeconstructCell),
            );
    }
}
