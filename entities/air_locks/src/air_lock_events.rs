use api::{
    chat::{FURTHER_ITALIC_FONT, WARNING_COLOR},
    data::{AirLockCloseRequest, LockedStatus, Vec2Int},
    entity_updates::EntityGroup,
    examinable::Examinable,
    gridmap::{get_atmos_index, world_to_cell_id},
    network::ReliableServerMessage,
};
use atmospherics::diffusion::AtmosphericsResource;
use bevy::{
    hierarchy::Children,
    prelude::{warn, Commands, Entity, EventReader, EventWriter, Query, ResMut, Transform},
};
use bevy_rapier3d::prelude::CollisionGroups;
use pawn::pawn::{Pawn, ShipAuthorization};
use physics::physics::{get_bit_masks, ColliderGroup};
use sfx::{builder::sfx_builder, entity_update::SfxAutoDestroyTimers};
use sounds::{
    air_lock::{
        air_lock_closed_sfx::AirLockClosedSfxBundle, air_lock_denied_sfx::AirLockDeniedSfxBundle,
        air_lock_open_sfx::AirLockOpenSfxBundle,
    },
    shared::sfx_auto_destroy,
};

use super::{
    air_lock_added::{
        AirLockCollision, AirLockLockClosed, AirLockLockOpen, AirLockUnlock, InputAirLockToggleOpen,
    },
    net::NetAirLock,
    resources::{
        closed_timer, denied_timer, open_timer, AccessLightsStatus, AirLock, AirLockStatus,
    },
};

pub struct AirLockOpenRequest {
    pub opener_option: Option<Entity>,
    pub opened: Entity,
}

/// Manage air lock events.
pub(crate) fn air_lock_events(
    mut air_lock_collisions: EventReader<AirLockCollision>,
    mut toggle_open_action: EventReader<InputAirLockToggleOpen>,
    transforms: Query<&Transform>,
    mut air_lock_query: Query<(&mut AirLock, Entity, &mut Examinable, &Children)>,
    pawn_query: Query<(&Pawn, &ShipAuthorization)>,
    mut auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
    mut air_lock_lock_open_event: EventReader<AirLockLockOpen>,
    mut air_lock_lock_close_event: EventReader<AirLockLockClosed>,
    mut unlock_events: EventReader<AirLockUnlock>,
    mut net_airlocks: EventWriter<NetAirLock>,
    mut collision_groups: Query<&mut CollisionGroups>,
) {
    let mut close_requests = vec![];
    let mut open_requests = vec![];

    for event in unlock_events.iter() {
        match air_lock_query.get_mut(event.locked) {
            Ok((mut air_lock_component, _air_lock_entity, mut examinable_component, _children)) => {
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
            Ok((mut air_lock_component, _air_lock_entity, mut examinable_component, _children)) => {
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
            Ok((mut air_lock_component, _air_lock_entity, mut examinable_component, _children)) => {
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

    for (mut air_lock_component, air_lock_entity, _examinable_component, children) in
        air_lock_query.iter_mut()
    {
        let rigid_body_position_component;

        match transforms.get(air_lock_entity) {
            Ok(tra) => {
                rigid_body_position_component = tra.clone();
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

                    let mut child_collider_entity = None;

                    for child in children.iter() {
                        match collision_groups.get(*child) {
                            Ok(_t) => {
                                child_collider_entity = Some(*child);
                            }
                            Err(_) => {}
                        }
                    }

                    match child_collider_entity {
                        Some(e) => {
                            let mut r = collision_groups.get_mut(e).unwrap();
                            let masks = get_bit_masks(ColliderGroup::Standard);

                            r.memberships = masks.0;
                            r.filters = masks.1;
                        }
                        None => {
                            warn!("Couldnt find collider child");
                            continue;
                        }
                    }

                    air_lock_component.access_lights = AccessLightsStatus::Neutral;

                    let sfx_entity = sfx_builder(
                        &mut commands,
                        rigid_body_position_component,
                        Box::new(AirLockClosedSfxBundle::new),
                    );

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
        match air_lock_query.get(event.opened) {
            Ok((air_lock_component, _air_lock_entity, _examinable_component, _children)) => {
                match air_lock_component.status {
                    AirLockStatus::Open => {
                        close_requests.push(AirLockCloseRequest {
                            interacter_option: Some(event.opener),
                            interacted: event.opened,
                        });
                    }
                    AirLockStatus::Closed => {
                        open_requests.push(AirLockOpenRequest {
                            opener_option: Some(event.opener),
                            opened: event.opened,
                        });
                    }
                }
            }
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
                children = result.3;
            }
            Err(_err) => {
                continue;
            }
        }

        match transforms.get(request.opened) {
            Ok(t) => {
                air_lock_static_transform_component = t.clone();
            }
            Err(_rr) => {
                warn!("Couldnt find transform of air_lock!");
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

        let collision_transform_component;

        match transforms.get(request.opened) {
            Ok(t) => {
                collision_transform_component = t;
            }
            Err(_rr) => {
                warn!("Couldnt find transform!!");
                continue;
            }
        }

        if pawn_has_permission == true {
            let cell_id = world_to_cell_id(collision_transform_component.translation);
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

            let mut collision_child_option = None;

            for child in children.iter() {
                match collision_groups.get(*child) {
                    Ok(_col) => {
                        collision_child_option = Some(*child);
                    }
                    Err(_rr) => {}
                }
            }

            match collision_child_option {
                Some(ent) => {
                    let mut r = collision_groups.get_mut(ent).unwrap();

                    let masks = get_bit_masks(ColliderGroup::NoCollision);

                    r.memberships = masks.0;
                    r.filters = masks.1;
                }
                None => {
                    warn!("Couldnt find collider child..");
                }
            }

            air_lock_component.open_timer_option = Some(open_timer());

            let sfx_entity = sfx_builder(
                &mut commands,
                air_lock_static_transform_component,
                Box::new(AirLockOpenSfxBundle::new),
            );
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        } else {
            air_lock_component.access_lights = AccessLightsStatus::Denied;

            air_lock_component.denied_timer_option = Some(denied_timer());

            let sfx_entity = sfx_builder(
                &mut commands,
                air_lock_static_transform_component,
                Box::new(AirLockDeniedSfxBundle::new),
            );
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        }
    }

    for request in close_requests {
        match air_lock_query.get_mut(request.interacted) {
            Ok((mut air_lock_component, _air_lock_entity, _examinable_component, _children)) => {
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
