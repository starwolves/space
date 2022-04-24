use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::space::{
    core::entity::{
        functions::initialize_entity_data::initialize_entity_data,
        resources::{EntityDataProperties, EntityDataResource},
    },
    StartupLabels,
};

use self::{spawn::ComputerBundle, systems::computer_added};

pub mod components;
pub mod spawn;
pub mod systems;

pub struct ComputersPlugin;

impl Plugin for ComputersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(computer_added)
            .add_startup_system(content_initialization.before(StartupLabels::BuildGridmap));
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: "bridgeComputer".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(ComputerBundle::spawn),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
