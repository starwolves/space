use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::Children;
use bevy_log::warn;
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        atmospherics::{functions::get_atmos_index, resources::AtmosphericsResource},
        chat::functions::{FURTHER_ITALIC_FONT, WARNING_COLOR},
        entity::components::EntityGroup,
        examinable::components::Examinable,
        gridmap::{functions::gridmap_functions::world_to_cell_id, resources::Vec2Int},
        networking::resources::ReliableServerMessage,
        pawn::components::{Pawn, ShipAuthorization},
        sfx::{components::sfx_auto_destroy, resources::SfxAutoDestroyTimers},
        static_body::components::StaticTransform,
    },
    entities::{
        air_locks::{
            components::{
                closed_timer, denied_timer, open_timer, AccessLightsStatus, AirLock, AirLockStatus,
                LockedStatus,
            },
            events::{
                AirLockCollision, AirLockLockClosed, AirLockLockOpen, AirLockUnlock,
                InputAirLockToggleOpen, NetAirLock,
            },
        },
        sfx::air_lock::{
            air_lock_closed_sfx::AirLockClosedSfxBundle,
            air_lock_denied_sfx::AirLockDeniedSfxBundle, air_lock_open_sfx::AirLockOpenSfxBundle,
        },
    },
};

pub struct AirLockOpenRequest {
    pub opener_option: Option<Entity>,
    pub opened: Entity,
}

pub struct AirLockCloseRequest {
    pub interacter_option: Option<Entity>,
    pub interacted: Entity,
}

pub fn air_lock_events(
    mut air_lock_collisions: EventReader<AirLockCollision>,
    mut toggle_open_action: EventReader<InputAirLockToggleOpen>,
    mut transforms: Query<&mut Transform>,
    mut air_lock_query: Query<(
        &mut AirLock,
        &StaticTransform,
        Entity,
        &mut Examinable,
        &Children,
    )>,
    pawn_query: Query<(&Pawn, &ShipAuthorization)>,
    mut auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
    mut air_lock_lock_open_event: EventReader<AirLockLockOpen>,
    mut air_lock_lock_close_event: EventReader<AirLockLockClosed>,
    mut unlock_events: EventReader<AirLockUnlock>,
    mut net_airlocks: EventWriter<NetAirLock>,
) {
    let mut close_requests = vec![];
    let mut open_requests = vec![];

    for event in unlock_events.iter() {
        match air_lock_query.get_mut(event.locked) {
            Ok((
                mut air_lock_component,
                _static_transform_component,
                _air_lock_entity,
                mut examinable_component,
                _children,
            )) => {
                air_lock_component.locked_status = LockedStatus::None;
                air_lock_component.access_lights = AccessLightsStatus::Neutral;

                match air_lock_component.status {
                    AirLockStatus::Open => {
                        close_requests.push(AirLockCloseRequest {
                            interacter_option: None,
                            interacted: event.locked,
                        });
                    }
                    AirLockStatus::Closed => {}
                }

                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've unlocked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_airlocks.send(NetAirLock {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                examinable_component.assigned_texts.remove(&11);
            }
            Err(_rr) => {}
        }
    }

    for event in air_lock_lock_open_event.iter() {
        match air_lock_query.get_mut(event.locked) {
            Ok((
                mut air_lock_component,
                _static_transform_component,
                _air_lock_entity,
                mut examinable_component,
                _children,
            )) => {
                air_lock_component.locked_status = LockedStatus::Open;
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've opened and locked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_airlocks.send(NetAirLock {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                match air_lock_component.status {
                    AirLockStatus::Open => {}
                    AirLockStatus::Closed => {
                        open_requests.push(AirLockOpenRequest {
                            opener_option: None,
                            opened: event.locked,
                        });
                    }
                }
                examinable_component.assigned_texts.insert(
                    11,
                    "[font=".to_string()
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + WARNING_COLOR
                        + "]It is locked open.[/color][/font]",
                );
            }
            Err(_rr) => {}
        }
    }
    for event in air_lock_lock_close_event.iter() {
        match air_lock_query.get_mut(event.locked) {
            Ok((
                mut air_lock_component,
                _static_transform_component,
                _air_lock_entity,
                mut examinable_component,
                _children,
            )) => {
                air_lock_component.locked_status = LockedStatus::Closed;
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've closed and locked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_airlocks.send(NetAirLock {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                match air_lock_component.status {
                    AirLockStatus::Open => {
                        close_requests.push(AirLockCloseRequest {
                            interacter_option: None,
                            interacted: event.locked,
                        });
                    }
                    AirLockStatus::Closed => {}
                }

                examinable_component.assigned_texts.insert(
                    11,
                    "[font=".to_string()
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + WARNING_COLOR
                        + "]It is locked shut.[/color][/font]",
                );
            }
            Err(_rr) => {}
        }
    }

    for (
        mut air_lock_component,
        static_transform_component,
        air_lock_entity,
        _examinable_component,
        _children,
    ) in air_lock_query.iter_mut()
    {
        let mut rigid_body_position_component;

        match transforms.get_mut(air_lock_entity) {
            Ok(tra) => {
                rigid_body_position_component = tra;
            }
            Err(_rr) => {
                warn!("Couldnt find transform!");
                continue;
            }
        }

        match air_lock_component.locked_status {
            LockedStatus::Open => {
                if !matches!(air_lock_component.access_lights, AccessLightsStatus::Denied) {
                    air_lock_component.access_lights = AccessLightsStatus::Denied;
                }
            }
            LockedStatus::Closed => {
                if !matches!(air_lock_component.access_lights, AccessLightsStatus::Denied) {
                    air_lock_component.access_lights = AccessLightsStatus::Denied;
                }
            }
            LockedStatus::None => {}
        }

        match air_lock_component.open_timer_option.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();
                    close_requests.push(AirLockCloseRequest {
                        interacter_option: None,
                        interacted: air_lock_entity,
                    });
                }
            }
            None => {}
        }

        match air_lock_component.closed_timer_option.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();

                    let mut air_lock_rigid_body_position = rigid_body_position_component.clone();

                    air_lock_rigid_body_position.translation.y = 0.;

                    rigid_body_position_component.translation =
                        air_lock_rigid_body_position.translation;
                    rigid_body_position_component.rotation = air_lock_rigid_body_position.rotation;
                    rigid_body_position_component.scale = air_lock_rigid_body_position.scale;

                    air_lock_component.access_lights = AccessLightsStatus::Neutral;

                    let sfx_entity = commands
                        .spawn()
                        .insert_bundle(AirLockClosedSfxBundle::new(
                            static_transform_component.transform,
                        ))
                        .id();
                    sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
                }
            }
            None => {}
        }

        match air_lock_component.denied_timer_option.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();

                    air_lock_component.access_lights = AccessLightsStatus::Neutral;
                }
            }
            None => {}
        }
    }

    for event in toggle_open_action.iter() {
        match air_lock_query.get(Entity::from_bits(event.opened)) {
            Ok((
                air_lock_component,
                _static_transform_component,
                _air_lock_entity,
                _examinable_component,
                _children,
            )) => match air_lock_component.status {
                AirLockStatus::Open => {
                    close_requests.push(AirLockCloseRequest {
                        interacter_option: Some(event.opener),
                        interacted: Entity::from_bits(event.opened),
                    });
                }
                AirLockStatus::Closed => {
                    open_requests.push(AirLockOpenRequest {
                        opener_option: Some(event.opener),
                        opened: Entity::from_bits(event.opened),
                    });
                }
            },
            Err(_rr) => {}
        }
    }

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

        open_requests.push(AirLockOpenRequest {
            opener_option: Some(pawn_entity),
            opened: air_lock_entity,
        });
    }

    for request in open_requests {
        let air_lock_components_result = air_lock_query.get_mut(request.opened);

        let mut air_lock_component;
        let children;
        let air_lock_static_transform_component;

        match air_lock_components_result {
            Ok(result) => {
                air_lock_component = result.0;
                air_lock_static_transform_component = result.1;
                children = result.4;
            }
            Err(_err) => {
                continue;
            }
        }

        match air_lock_component.locked_status {
            LockedStatus::Open => {}
            LockedStatus::Closed => {
                // Locked and closed, won't open.
                continue;
            }
            LockedStatus::None => {}
        }

        let mut pawn_has_permission = false;

        match request.opener_option {
            Some(opener) => {
                let pawn_space_access_component_result =
                    pawn_query.get_component::<ShipAuthorization>(opener);
                let pawn_space_access_component;

                match pawn_space_access_component_result {
                    Ok(result) => {
                        pawn_space_access_component = result;
                    }
                    Err(_err) => {
                        continue;
                    }
                }

                for space_permission in &air_lock_component.access_permissions {
                    if pawn_space_access_component
                        .access
                        .contains(space_permission)
                        == true
                    {
                        pawn_has_permission = true;
                        break;
                    }
                }
            }
            None => {
                pawn_has_permission = true;
            }
        }

        let mut collision_transform_component;

        let child_option = children.get(0);
        match child_option {
            Some(child) => match transforms.get_mut(*child) {
                Ok(t) => {
                    collision_transform_component = t;
                }
                Err(_rr) => {
                    continue;
                }
            },
            None => {
                warn!("No children!");
                continue;
            }
        }

        if pawn_has_permission == true {
            let cell_id = world_to_cell_id(collision_transform_component.translation.into());
            let cell_id2 = Vec2Int {
                x: cell_id.x,
                y: cell_id.z,
            };
            if AtmosphericsResource::is_id_out_of_range(cell_id2) {
                continue;
            }
            let atmos_id = get_atmos_index(cell_id2);
            let atmospherics = atmospherics_resource
                .atmospherics
                .get_mut(atmos_id)
                .unwrap();

            atmospherics.blocked = false;
            air_lock_component.status = AirLockStatus::Open;
            air_lock_component.access_lights = AccessLightsStatus::Granted;

            let mut air_lock_rigid_body_position = collision_transform_component.clone();
            air_lock_rigid_body_position.translation.y = 2.;

            collision_transform_component.translation = air_lock_rigid_body_position.translation;
            collision_transform_component.scale = air_lock_rigid_body_position.scale;
            collision_transform_component.rotation = air_lock_rigid_body_position.rotation;

            air_lock_component.open_timer_option = Some(open_timer());

            let sfx_entity = commands
                .spawn()
                .insert_bundle(AirLockOpenSfxBundle::new(
                    air_lock_static_transform_component.transform,
                ))
                .id();
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        } else {
            air_lock_component.access_lights = AccessLightsStatus::Denied;

            air_lock_component.denied_timer_option = Some(denied_timer());

            let sfx_entity = commands
                .spawn()
                .insert_bundle(AirLockDeniedSfxBundle::new(
                    air_lock_static_transform_component.transform,
                ))
                .id();
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        }
    }

    for request in close_requests {
        match air_lock_query.get_mut(request.interacted) {
            Ok((
                mut air_lock_component,
                _static_transform_component,
                _air_lock_entity,
                _examinable_component,
                _children,
            )) => {
                let rigid_body_position_component;

                match transforms.get(request.interacted) {
                    Ok(tra) => {
                        rigid_body_position_component = tra;
                    }
                    Err(_rr) => {
                        warn!("Couldnt find transform!");
                        continue;
                    }
                }

                match air_lock_component.locked_status {
                    LockedStatus::Open => {
                        continue;
                    }
                    LockedStatus::Closed => {}
                    LockedStatus::None => {}
                }

                let mut pawn_has_permission = false;

                match request.interacter_option {
                    Some(interacter) => {
                        let pawn_space_access_component_result =
                            pawn_query.get_component::<ShipAuthorization>(interacter);
                        let pawn_space_access_component;

                        match pawn_space_access_component_result {
                            Ok(result) => {
                                pawn_space_access_component = result;
                            }
                            Err(_err) => {
                                continue;
                            }
                        }

                        for space_permission in &air_lock_component.access_permissions {
                            if pawn_space_access_component
                                .access
                                .contains(space_permission)
                                == true
                            {
                                pawn_has_permission = true;
                                break;
                            }
                        }
                    }
                    None => {
                        pawn_has_permission = true;
                    }
                }

                if pawn_has_permission == false {
                    continue;
                }

                let cell_id = world_to_cell_id(rigid_body_position_component.translation.into());
                let cell_id2 = Vec2Int {
                    x: cell_id.x,
                    y: cell_id.z,
                };
                if AtmosphericsResource::is_id_out_of_range(cell_id2) {
                    continue;
                }
                let atmos_id = get_atmos_index(cell_id2);
                let atmospherics = atmospherics_resource
                    .atmospherics
                    .get_mut(atmos_id)
                    .unwrap();

                atmospherics.blocked = true;
                air_lock_component.status = AirLockStatus::Closed;

                air_lock_component.closed_timer_option = Some(closed_timer());
            }

            Err(_rr) => {}
        }
    }
}
