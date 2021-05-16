use bevy::prelude::{EventReader, Query, info};

use crate::space_core::{components::{air_lock::{AirLock, AirLockStatus}, entity_data::EntityGroup, pawn::Pawn, space_access::SpaceAccess}, events::physics::air_lock_collision::AirLockCollision};

pub fn air_lock_physics(
    mut air_lock_collisions : EventReader<AirLockCollision>,
    mut air_lock_query : Query<&mut AirLock>,
    pawn_query : Query<(&Pawn, &SpaceAccess)>
) {

    for collision_event in air_lock_collisions.iter() {

        if collision_event.started == false {
            continue;
        }

        let air_lock_entity;
        let pawn_entity;

        if matches!(collision_event.collider1_group, EntityGroup::AirLock) {

            air_lock_entity = collision_event.collider1_entity;
            pawn_entity = collision_event.collider2_entity;

        } else {

            air_lock_entity = collision_event.collider2_entity;
            pawn_entity = collision_event.collider1_entity;

        }

        
        let pawn_space_access_component_result = pawn_query.get_component::<SpaceAccess>(pawn_entity);
        let pawn_space_access_component;


        match pawn_space_access_component_result {
            Ok(result) => {
                pawn_space_access_component = result;
            }
            Err(_err) => {continue;}
        }


        let air_lock_component_result = air_lock_query.get_component_mut::<AirLock>(air_lock_entity);
        let mut air_lock_component;

        match air_lock_component_result {
            Ok(result) => {
                air_lock_component = result;
            }
            Err(_err) => {continue;}
        }

        let mut pawn_has_permission = false;

        for space_permission in &air_lock_component.access_permissions {
            
            if pawn_space_access_component.access.contains(space_permission) {
                pawn_has_permission=true;
                break;
            }

        }


        if pawn_has_permission == true {
            info!("Sesame open now!");
            air_lock_component.status = AirLockStatus::Open;

        }


    }

}
