use std::collections::{hash_map::Entry, HashMap};

use bevy::{
    prelude::{
        warn, Component, Entity, Event, EventReader, EventWriter, Local, Query, Res, ResMut,
        Resource, Transform, With, Without,
    },
    time::Time,
};
use entity::{
    despawn::DespawnEntity,
    entity_data::{WorldMode, WorldModes},
};
use resources::{core::TickRate, grid::Tile};

/// A rigidbody that is linked to a decoupled entity.
#[derive(Component)]
pub struct SFRigidBody;
/// Entities that are linked to a decoupled rigidbody.
#[derive(Component, Default)]
pub struct RigidBodyLink {
    pub offset: Transform,
    // Used for client-side interpolation.
    pub target_transform: Transform,
    pub origin_transfom: Transform,
}

/// Resource linking rigidbodies to game entities.
#[derive(Resource, Default)]
pub struct RigidBodies {
    pub entity_map: HashMap<Entity, Vec<Entity>>,
    pub tile_map: HashMap<Entity, Vec<Entity>>,
}

impl RigidBodies {
    pub fn get_entity_rigidbody(&self, entity: &Entity) -> Option<&Entity> {
        for s in self.entity_map.iter() {
            if s.1.contains(entity) {
                return Some(s.0);
            }
        }
        return None;
    }
    pub fn get_tile_rigidbody(&self, entity: &Entity) -> Option<&Entity> {
        for s in self.tile_map.iter() {
            if s.1.contains(entity) {
                return Some(s.0);
            }
        }
        return None;
    }
    pub fn link_entity(&mut self, entity: &Entity, rigidbody: &Entity) {
        match self.entity_map.entry(*rigidbody) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(*entity);
            }
            Entry::Vacant(e) => {
                e.insert(vec![*entity]);
            }
        }
    }
    pub fn link_tile(&mut self, entity: &Entity, rigidbody: &Entity) {
        match self.tile_map.entry(*rigidbody) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(*entity);
            }
            Entry::Vacant(e) => {
                e.insert(vec![*entity]);
            }
        }
    }
    pub fn remove_linked_entity(&mut self, entity: &Entity) {
        for s in self.entity_map.iter_mut() {
            if s.1.contains(entity) {
                s.1.retain(|e| *e != *entity);
            }
        }
    }
    pub fn remove_linked_tile(&mut self, entity: &Entity) {
        for s in self.tile_map.iter_mut() {
            if s.1.contains(entity) {
                s.1.retain(|e| *e != *entity);
            }
        }
    }
    pub fn remove_entity_rigidbody(&mut self, rigidbody: &Entity) {
        self.entity_map.remove(rigidbody);
    }
    pub fn remove_tile_rigidbody(&mut self, rigidbody: &Entity) {
        self.tile_map.remove(rigidbody);
    }
}

pub(crate) fn remove_links(
    mut rigidbodies: ResMut<RigidBodies>,
    mut events: EventReader<DespawnEntity>,
) {
    for event in events.iter() {
        rigidbodies.remove_linked_entity(&event.entity);
        rigidbodies.remove_linked_tile(&event.entity);
    }
}

pub(crate) fn remove_rigidbodies(
    mut rigidbodies: ResMut<RigidBodies>,
    mut events: EventReader<DespawnEntity>,
) {
    for event in events.iter() {
        rigidbodies.remove_entity_rigidbody(&event.entity);
        rigidbodies.remove_tile_rigidbody(&event.entity);
    }
}

pub(crate) fn server_mirror_link_transform(
    mut transforms: Query<&mut Transform, Without<Tile>>,
    links_query: Query<&WorldMode, (With<RigidBodyLink>, Without<Tile>)>,
    rigidbodies: Res<RigidBodies>,
) {
    for (rigidbody, links) in rigidbodies.entity_map.iter() {
        let rbt;
        match transforms.get(*rigidbody) {
            Ok(t) => {
                rbt = t.clone();
            }
            Err(_) => {
                warn!("Couldnt find server_mirror_link_transform components.");
                continue;
            }
        }

        for link in links.iter() {
            match links_query.get(*link) {
                Ok(world_mode) => {
                    if !matches!(world_mode.mode, WorldModes::Physics) {
                        continue;
                    }
                }
                Err(_) => {
                    warn!("Couldnt find link components.");
                    continue;
                }
            }
            match transforms.get_mut(*link) {
                Ok(mut t) => {
                    *t = rbt;
                }
                Err(_) => {
                    warn!("Couldnt find link entity transform.");
                    continue;
                }
            }
        }
    }
}
#[derive(Event)]
pub struct ResetLerp;

pub(crate) fn client_mirror_link_target_transform(
    transforms: Query<&Transform, Without<Tile>>,
    mut target_transforms: Query<(&mut RigidBodyLink, &WorldMode), Without<Tile>>,
    rigidbodies: Res<RigidBodies>,
    mut reset: EventWriter<ResetLerp>,
) {
    for (rigidbody, links) in rigidbodies.entity_map.iter() {
        let rbt;
        match transforms.get(*rigidbody) {
            Ok(t) => {
                rbt = t.clone();
            }
            Err(_) => {
                warn!("Couldnt find client_mirror_link_target_transform components.");
                continue;
            }
        }

        for link in links.iter() {
            match target_transforms.get_mut(*link) {
                Ok((mut t, world_mode)) => {
                    if !matches!(world_mode.mode, WorldModes::Physics) {
                        continue;
                    }
                    let mut fin_transform = rbt.clone();
                    fin_transform.translation += t.offset.translation;
                    fin_transform.rotation *= t.offset.rotation;
                    fin_transform.scale *= t.offset.scale;

                    t.origin_transfom = t.target_transform.clone();
                    t.target_transform = fin_transform;
                }
                Err(_) => {
                    warn!("Couldnt find link entity transform.");
                    continue;
                }
            }
        }
    }
    reset.send(ResetLerp);
}

pub(crate) fn client_interpolate_link_transform(
    mut query: Query<(&mut Transform, &RigidBodyLink, &WorldMode), Without<Tile>>,
    rigidbodies: Res<RigidBodies>,
    time: Res<Time>,
    rate: Res<TickRate>,
    mut local_delta: Local<f32>,
    mut resets: EventReader<ResetLerp>,
) {
    let mut reset = false;
    for _ in resets.iter() {
        reset = true;
        break;
    }

    if reset {
        *local_delta = 0.;
    }

    let total_time = 1. / rate.physics_rate as f32;
    let dt = time.delta().as_secs_f32();
    let relative_delta = *local_delta / total_time;

    for links in rigidbodies.entity_map.values() {
        for link in links.iter() {
            match query.get_mut(*link) {
                Ok((mut transform, link_component, world_mode)) => {
                    if !matches!(world_mode.mode, WorldModes::Physics) {
                        continue;
                    }

                    let interp_position = link_component
                        .origin_transfom
                        .translation
                        .lerp(link_component.target_transform.translation, relative_delta);

                    let interp_scale = link_component
                        .origin_transfom
                        .scale
                        .lerp(link_component.target_transform.scale, relative_delta);

                    let interp_rotation = link_component
                        .origin_transfom
                        .rotation
                        .slerp(link_component.target_transform.rotation, relative_delta);

                    transform.translation = interp_position;
                    transform.rotation = interp_rotation;
                    transform.scale = interp_scale;
                }
                Err(_) => {
                    warn!("Couldnt find client_interpolate_link_transform components.");
                }
            }
        }
    }
    *local_delta += dt;
    if *local_delta > total_time {
        *local_delta = total_time;
    }
}
