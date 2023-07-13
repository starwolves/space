use std::collections::HashMap;

use bevy::prelude::{IntoSystemConfigs, Resource, Startup, SystemSet, Update};

/// Resource containing all registered entity types with [init_entity_type].
#[derive(Default, Resource)]
pub struct EntityTypes {
    startup_types: Vec<String>,
    pub netcode_types: HashMap<String, u16>,
    pub types: HashMap<String, BoxedEntityType>,
}
use bevy::prelude::ResMut;
use dyn_clone::DynClone;

pub fn store_entity_type<T: EntityType + 'static>(mut types: ResMut<EntityTypes>) {
    types.startup_types.push(T::new().get_identity());
    types
        .types
        .insert(T::new().get_identity(), Box::new(T::new()));
}

use bevy::prelude::info;

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
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum EntityTypeLabel {
    Register,
}
/// Each entity type has a type struct. Types are identified using unhygenic strings with prefixes.
/// String identifiers for entity types are useful for future persistent storage including databases.
/// A hardcoded string name per entity type makes it so that each entity name keeps the same name at all times despite any amount of code changes.
/// This way entities stored inside a database will remain identfiable across different codebases.
/// Use with [BoxedEntityType].

pub trait EntityType: Send + Sync + DynClone {
    /// Persistent string identifier of entity type. Unhygenic.
    fn get_identity(&self) -> String;
    fn get_clean_identity(&self) -> String;
    fn is_type(&self, identifier: String) -> bool;
    fn new() -> Self
    where
        Self: Sized;
}
dyn_clone::clone_trait_object!(EntityType);

pub type BoxedEntityType = Box<dyn EntityType>;
use crate::spawn::SpawnEntity;
use bevy::prelude::App;
use resources::labels::BuildingLabels;

pub fn register_entity_type<T: EntityType + Clone + Default + 'static>(app: &mut App) {
    app.add_systems(
        Startup,
        store_entity_type::<T>.in_set(EntityTypeLabel::Register),
    )
    .add_event::<SpawnEntity<T>>()
    .add_systems(
        Update,
        (build_raw_entities::<T>).after(BuildingLabels::TriggerBuild),
    );
}

use bevy::prelude::{Commands, EventReader, EventWriter};

use bevy::prelude::Transform;

use crate::entity_data::RawSpawnEvent;
use crate::spawn::EntityBuildData;

pub fn build_raw_entities<T: EntityType + Default + Send + Sync + 'static>(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut builder_computer: EventWriter<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != T::default().get_identity() {
            continue;
        }

        let mut entity_transform = Transform::from_translation(spawn_event.raw_entity.translation);
        entity_transform.rotation = spawn_event.raw_entity.rotation;
        entity_transform.scale = spawn_event.raw_entity.scale;
        builder_computer.send(SpawnEntity {
            spawn_data: EntityBuildData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity: commands.spawn(()).id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            entity_type: T::default(),
        });
    }
}
