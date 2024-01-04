use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{Query, Res},
    },
    log::{info, warn},
    transform::components::Transform,
};
use entity::despawn::DespawnEntity;

use crate::entity::{RigidBodies, SFRigidBody};

pub const MAX_BOUNDS: f32 = 4000.0;

pub(crate) fn despawn_out_of_bounds(
    query: Query<(Entity, &Transform), With<SFRigidBody>>,
    rigidbodies: Res<RigidBodies>,
    mut despawn: EventWriter<DespawnEntity>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x > MAX_BOUNDS
            || transform.translation.x < -MAX_BOUNDS
            || transform.translation.y > MAX_BOUNDS
            || transform.translation.y < -MAX_BOUNDS
            || transform.translation.z > MAX_BOUNDS
            || transform.translation.z < -MAX_BOUNDS
        {
            let ent;
            match rigidbodies.get_rigidbody_entity(&entity) {
                Some(e) => {
                    ent = *e;
                }
                None => {
                    warn!("Rigidbody is not linked to any entity.");
                    continue;
                }
            }
            despawn.send(DespawnEntity { entity: ent });

            info!("Despawning {:?}.", ent,);
        }
    }
}
