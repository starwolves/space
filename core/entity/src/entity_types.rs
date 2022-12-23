use std::collections::HashMap;

use bevy::prelude::{App, Resource, SystemLabel};

use crate::spawn::EntityType;

/// Resource containing all registered entity types with [init_entity_type].
#[cfg(any(feature = "client", feature = "server"))]
#[derive(Default, Resource)]
pub struct EntityTypes {
    startup_types: Vec<String>,
    pub types: HashMap<String, u16>,
}
use bevy::prelude::ResMut;

#[cfg(any(feature = "client", feature = "server"))]
fn register_entity_type<T: EntityType>(mut types: ResMut<EntityTypes>) {
    types.startup_types.push(T::new().to_string());
}
use crate::spawn::SpawnEntity;
use bevy::prelude::IntoSystemDescriptor;

#[cfg(any(feature = "client", feature = "server"))]
pub fn init_entity_type<T: EntityType + 'static>(app: &mut App) {
    app.add_startup_system(register_entity_type::<T>.label(EntityTypeLabel::Register))
        .add_event::<SpawnEntity<T>>();
}
#[cfg(any(feature = "client", feature = "server"))]
pub(crate) fn finalize_register_entity_types(mut types: ResMut<EntityTypes>) {
    use bevy::prelude::info;

    types.startup_types.sort();
    let mut i = 0;
    let list = types.startup_types.clone();
    for entity_type in list.iter() {
        types.types.insert(entity_type.to_string(), i);
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
