use std::collections::{hash_map::Entry, HashMap};

use bevy::prelude::{Component, Entity, EventReader, ResMut, Resource};
use entity::despawn::DespawnEntity;

/// A rigidbody that is linked to a decoupled entity.
#[derive(Component)]
pub struct SFRigidBody;
/// Entities that are linked to a decoupled rigidbody.
#[derive(Component)]
pub struct RigidBodyLink;

/// Resource linking rigidbodies to game entities.
#[derive(Resource, Default)]
pub struct RigidBodies {
    pub map: HashMap<Entity, Vec<Entity>>,
}

impl RigidBodies {
    pub fn get_entities(&self, entity: &Entity) -> Option<&Vec<Entity>> {
        self.map.get(entity)
    }
    pub fn get_rigidbody(&self, entity: &Entity) -> Option<&Entity> {
        for s in self.map.iter() {
            if s.1.contains(entity) {
                return Some(s.0);
            }
        }
        return None;
    }
    pub fn link_entity(&mut self, entity: &Entity, rigidbody: &Entity) {
        match self.map.entry(*rigidbody) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(*entity);
            }
            Entry::Vacant(e) => {
                e.insert(vec![*entity]);
            }
        }
    }
    pub fn remove_linked_entity(&mut self, entity: &Entity) {
        for s in self.map.iter_mut() {
            if s.1.contains(entity) {
                s.1.retain(|e| *e != *entity);
            }
        }
    }
    pub fn remove_rigidbody(&mut self, rigidbody: &Entity) {
        self.map.remove(rigidbody);
    }
}

pub(crate) fn remove_linked_entities(
    mut rigidbodies: ResMut<RigidBodies>,
    mut events: EventReader<DespawnEntity>,
) {
    for event in events.iter() {
        rigidbodies.remove_linked_entity(&event.entity)
    }
}

pub(crate) fn remove_rigidbodies(
    mut rigidbodies: ResMut<RigidBodies>,
    mut events: EventReader<DespawnEntity>,
) {
    for event in events.iter() {
        rigidbodies.remove_rigidbody(&event.entity);
    }
}
