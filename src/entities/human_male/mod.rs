use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::core::{
    entity::{
        functions::initialize_entity_data::initialize_entity_data,
        resources::{EntityDataProperties, EntityDataResource},
    },
    space_plugin::StartupLabels,
};

use self::spawn::HumanMaleBundle;

pub mod functions;
pub mod spawn;

pub struct HumanMalePlugin;

impl Plugin for HumanMalePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities));
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: "humanDummy".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HumanMaleBundle::spawn),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: "humanMale".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HumanMaleBundle::spawn),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
