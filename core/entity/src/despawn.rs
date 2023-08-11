use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Event, EventReader};

/// The event to use to despawn an entity.
#[derive(Event)]
pub struct DespawnEntity {
    pub entity: Entity,
}

pub(crate) fn despawn_entities(mut events: EventReader<DespawnEntity>, mut commands: Commands) {
    for event in events.iter() {
        commands.entity(event.entity).despawn_recursive();
    }
}
