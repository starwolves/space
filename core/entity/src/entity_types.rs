use std::collections::HashMap;

use bevy::prelude::{App, Resource, SystemLabel};

/// Resource containing all registered entity types with [init_entity_type].
#[cfg(any(feature = "client", feature = "server"))]
#[derive(Default, Resource)]
pub struct EntityTypes {
    startup_types: Vec<String>,
    pub netcode_types: HashMap<String, u16>,
    pub types: HashMap<String, BoxedEntityType>,
}
use bevy::prelude::ResMut;
use dyn_clone::DynClone;

#[cfg(any(feature = "client", feature = "server"))]
fn register_entity_type<T: EntityType + 'static>(mut types: ResMut<EntityTypes>) {
    types.startup_types.push(T::new().to_string());
    types.types.insert(T::new().to_string(), Box::new(T::new()));
}
use crate::spawn::SpawnEntity;
use bevy::prelude::IntoSystemDescriptor;
use resources::labels::StartupLabels;

#[cfg(any(feature = "client", feature = "server"))]
pub fn init_entity_type<T: EntityType + 'static>(app: &mut App) {
    app.add_startup_system(register_entity_type::<T>.label(EntityTypeLabel::Register))
        .add_event::<SpawnEntity<T>>()
        .add_startup_system(content_initialization::<T>.before(StartupLabels::BuildGridmap));
}
use crate::entity_data::initialize_entity_data;
use crate::meta::{EntityDataProperties, EntityDataResource};

#[cfg(feature = "server")]
pub(crate) fn content_initialization<T: EntityType>(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: T::new().to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };
    initialize_entity_data(&mut entity_data, entity_properties);
}

use bevy::prelude::info;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) fn finalize_register_entity_types(mut types: ResMut<EntityTypes>) {
    types.startup_types.sort();
    let mut i = 0;
    let list = types.startup_types.clone();
    for entity_type in list.iter() {
        types.netcode_types.insert(entity_type.to_string(), i);
        i += 1;
    }
    info!("Loaded {:?} entity types.", i);
}
/// System label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(any(feature = "client", feature = "server"))]
pub enum EntityTypeLabel {
    Register,
}
/// Each entity type has a type struct. Types are identified using unhygenic strings with prefixes.
/// String identifiers for entity types are useful for future persistent storage including databases.
/// A hardcoded string name per entity type makes it so that each entity name keeps the same name at all times despite any amount of code changes.
/// This way entities stored inside a database will remain identfiable across different codebases.
/// Use with [BoxedEntityType].
#[cfg(any(feature = "server", feature = "client"))]
pub trait EntityType: Send + Sync + DynClone {
    /// Persistent string identifier of entity type. Unhygenic.
    fn to_string(&self) -> String;
    fn is_type(&self, other_type: BoxedEntityType) -> bool;
    fn new() -> Self
    where
        Self: Sized;
}
dyn_clone::clone_trait_object!(EntityType);

pub type BoxedEntityType = Box<dyn EntityType>;
