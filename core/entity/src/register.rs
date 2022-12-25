use resources::labels::StartupLabels;

use crate::{
    entity_types::{content_initialization, store_entity_type, EntityType, EntityTypeLabel},
    spawn::SpawnEntity,
};
use bevy::prelude::App;
use bevy::prelude::IntoSystemDescriptor;

#[cfg(any(feature = "client", feature = "server"))]
pub fn register_entity_type<T: EntityType + Clone + Default + 'static>(app: &mut App) {
    app.add_startup_system(store_entity_type::<T>.label(EntityTypeLabel::Register))
        .add_event::<SpawnEntity<T>>()
        .add_startup_system(content_initialization::<T>.before(StartupLabels::BuildGridmap));
}
