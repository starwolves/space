use bevy::prelude::EventReader;

use crate::space_core::events::physics::air_lock_collision::AirLockCollision;

pub fn air_lock_physics(
    mut air_lock_collisions : EventReader<AirLockCollision>
) {

    for collision_event in air_lock_collisions.iter() {

        

    }

}
