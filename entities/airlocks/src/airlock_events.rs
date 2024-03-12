use bevy::log::warn;
use bevy::{
    hierarchy::Children,
    prelude::{Commands, Entity, Event, EventReader, Query, Transform},
};

use bevy_renet::renet::ClientId;
use entity::{entity_data::EntityGroup, examine::Examinable};
use pawn::pawn::{Pawn, ShipAuthorization};
use resources::math::{world_to_cell_id, Vec2Int};
use sfx::builder::sfx_builder;
use sounds::airlock::{
    airlock_closed_sfx::AirLockClosedSfxBundle, airlock_denied_sfx::AirLockDeniedSfxBundle,
    airlock_open_sfx::AirLockOpenSfxBundle,
};
use text_api::core::{FURTHER_ITALIC_FONT, WARNING_COLOR};

use super::resources::{
    closed_timer, denied_timer, open_timer, AccessLightsStatus, Airlock, AirlockStatus,
};

/// Air lock open request event.

pub struct AirLockOpenRequest {
    pub opener_option: Option<Entity>,
    pub opened: Entity,
}
use networking::server::NetworkingChatServerMessage;

use bevy::prelude::EventWriter;

use networking::server::OutgoingReliableServerMessage;
/// Manage air lock events.

pub(crate) fn airlock_events(
    mut airlock_collisions: EventReader<AirlockCollision>,
    mut toggle_open_action: EventReader<InputAirlockToggleOpen>,
    transforms: Query<&Transform>,
    mut airlock_query: Query<(&mut Airlock, Entity, &mut Examinable, &Children)>,
    pawn_query: Query<(&Pawn, &ShipAuthorization)>,
    mut commands: Commands,
    mut airlock_lock_open_event: EventReader<AirLockLockOpen>,
    mut airlock_lock_close_event: EventReader<AirlockLockClosed>,
    mut unlock_events: EventReader<AirlockUnlock>,
    mut server: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
) {
    let mut close_requests = vec![];
    let mut open_requests = vec![];

    for event in unlock_events.read() {
        match airlock_query.get_mut(event.locked) {
            Ok((mut airlock_component, _airlock_entity, mut examinable_component, _children)) => {
                airlock_component.locked_status = LockedStatus::None;
                airlock_component.access_lights = AccessLightsStatus::Neutral;

                match airlock_component.status {
                    AirlockStatus::Open => {
                        close_requests.push(AirlockCloseRequest {
                            interacter_option: None,
                            interacted: event.locked,
                        });
                    }
                    AirlockStatus::Closed => {}
                }

                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've unlocked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        server.send(OutgoingReliableServerMessage {
                            handle: t,
                            message: NetworkingChatServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                examinable_component.assigned_texts.remove(&11);
            }
            Err(_rr) => {}
        }
    }

    for event in airlock_lock_open_event.read() {
        match airlock_query.get_mut(event.locked) {
            Ok((mut airlock_component, _airlock_entity, mut examinable_component, _children)) => {
                airlock_component.locked_status = LockedStatus::Open;
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've opened and locked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        server.send(OutgoingReliableServerMessage {
                            handle: t,
                            message: NetworkingChatServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                match airlock_component.status {
                    AirlockStatus::Open => {}
                    AirlockStatus::Closed => {
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
    for event in airlock_lock_close_event.read() {
        match airlock_query.get_mut(event.locked) {
            Ok((mut airlock_component, _airlock_entity, mut examinable_component, _children)) => {
                airlock_component.locked_status = LockedStatus::Closed;
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've closed and locked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        server.send(OutgoingReliableServerMessage {
                            handle: t,
                            message: NetworkingChatServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                match airlock_component.status {
                    AirlockStatus::Open => {
                        close_requests.push(AirlockCloseRequest {
                            interacter_option: None,
                            interacted: event.locked,
                        });
                    }
                    AirlockStatus::Closed => {}
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

    for (mut airlock_component, airlock_entity, _examinable_component, _children) in
        airlock_query.iter_mut()
    {
        let rigid_body_position_component;

        match transforms.get(airlock_entity) {
            Ok(tra) => {
                rigid_body_position_component = tra.clone();
            }
            Err(_rr) => {
                warn!("Couldnt find transform!");
                continue;
            }
        }

        match airlock_component.locked_status {
            LockedStatus::Open => {
                if !matches!(airlock_component.access_lights, AccessLightsStatus::Denied) {
                    airlock_component.access_lights = AccessLightsStatus::Denied;
                }
            }
            LockedStatus::Closed => {
                if !matches!(airlock_component.access_lights, AccessLightsStatus::Denied) {
                    airlock_component.access_lights = AccessLightsStatus::Denied;
                }
            }
            LockedStatus::None => {}
        }

        match airlock_component.open_timer_option.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();
                    close_requests.push(AirlockCloseRequest {
                        interacter_option: None,
                        interacted: airlock_entity,
                    });
                }
            }
            None => {}
        }

        match airlock_component.closed_timer_option.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();

                    airlock_component.access_lights = AccessLightsStatus::Neutral;

                    sfx_builder(
                        &mut commands,
                        rigid_body_position_component,
                        Box::new(AirLockClosedSfxBundle::new),
                    );
                }
            }
            None => {}
        }

        match airlock_component.denied_timer_option.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();

                    airlock_component.access_lights = AccessLightsStatus::Neutral;
                }
            }
            None => {}
        }
    }

    for event in toggle_open_action.read() {
        match airlock_query.get(event.opened) {
            Ok((airlock_component, _airlock_entity, _examinable_component, _children)) => {
                match airlock_component.status {
                    AirlockStatus::Open => {
                        close_requests.push(AirlockCloseRequest {
                            interacter_option: Some(event.opener),
                            interacted: event.opened,
                        });
                    }
                    AirlockStatus::Closed => {
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

    for collision_event in airlock_collisions.read() {
        if collision_event.started == false {
            continue;
        }

        let airlock_entity;
        let pawn_entity;

        if matches!(collision_event.collider1_group, EntityGroup::AirLock) {
            airlock_entity = collision_event.collider1_entity;
            pawn_entity = collision_event.collider2_entity;
        } else {
            airlock_entity = collision_event.collider2_entity;
            pawn_entity = collision_event.collider1_entity;
        }

        open_requests.push(AirLockOpenRequest {
            opener_option: Some(pawn_entity),
            opened: airlock_entity,
        });
    }

    for request in open_requests {
        let airlock_components_result = airlock_query.get_mut(request.opened);

        let mut airlock_component;
        let airlock_static_transform_component;

        match airlock_components_result {
            Ok(result) => {
                airlock_component = result.0;
            }
            Err(_err) => {
                continue;
            }
        }

        match transforms.get(request.opened) {
            Ok(t) => {
                airlock_static_transform_component = t.clone();
            }
            Err(_rr) => {
                warn!("Couldnt find transform of airlock!");
                continue;
            }
        }

        match airlock_component.locked_status {
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
                let pawn_space_access_component_result = pawn_query.get(opener);
                let pawn_space_access_component;

                match pawn_space_access_component_result {
                    Ok(result) => {
                        pawn_space_access_component = result;
                    }
                    Err(_err) => {
                        continue;
                    }
                }

                for space_permission in &airlock_component.access_permissions {
                    if pawn_space_access_component
                        .1
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
            let _cell_id2 = Vec2Int {
                x: cell_id.x,
                y: cell_id.z,
            };
            airlock_component.status = AirlockStatus::Open;
            airlock_component.access_lights = AccessLightsStatus::Granted;

            airlock_component.open_timer_option = Some(open_timer());

            sfx_builder(
                &mut commands,
                airlock_static_transform_component,
                Box::new(AirLockOpenSfxBundle::new),
            );
        } else {
            airlock_component.access_lights = AccessLightsStatus::Denied;

            airlock_component.denied_timer_option = Some(denied_timer());

            sfx_builder(
                &mut commands,
                airlock_static_transform_component,
                Box::new(AirLockDeniedSfxBundle::new),
            );
        }
    }

    for request in close_requests {
        match airlock_query.get_mut(request.interacted) {
            Ok((mut airlock_component, _airlock_entity, _examinable_component, _children)) => {
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

                match airlock_component.locked_status {
                    LockedStatus::Open => {
                        continue;
                    }
                    LockedStatus::Closed => {}
                    LockedStatus::None => {}
                }

                let mut pawn_has_permission = false;

                match request.interacter_option {
                    Some(interacter) => {
                        let pawn_space_access_component_result = pawn_query.get(interacter);
                        let pawn_space_access_component;

                        match pawn_space_access_component_result {
                            Ok(result) => {
                                pawn_space_access_component = result;
                            }
                            Err(_err) => {
                                continue;
                            }
                        }

                        for space_permission in &airlock_component.access_permissions {
                            if pawn_space_access_component
                                .1
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
                let _cell_id2 = Vec2Int {
                    x: cell_id.x,
                    y: cell_id.z,
                };
                airlock_component.status = AirlockStatus::Closed;

                airlock_component.closed_timer_option = Some(closed_timer());
            }

            Err(_rr) => {}
        }
    }
}

/// Air lock collision event.
#[derive(Event)]
pub struct AirlockCollision {
    pub collider1_entity: Entity,
    pub collider2_entity: Entity,

    pub collider1_group: EntityGroup,
    pub collider2_group: EntityGroup,

    /// Collision started or ended.
    pub started: bool,
}

/// Air lock toggle open event.
#[derive(Event)]
pub struct InputAirlockToggleOpen {
    pub handle_option: Option<ClientId>,

    pub opener: Entity,
    pub opened: Entity,
}
/// Air lock , lock the door to open event.
#[derive(Event)]
pub struct AirLockLockOpen {
    pub handle_option: Option<ClientId>,

    pub locked: Entity,
    pub locker: Entity,
}
/// Air lock , lock the door to closed event.
#[derive(Event)]
pub struct AirlockLockClosed {
    pub handle_option: Option<ClientId>,

    pub locked: Entity,
    pub locker: Entity,
}
/// Unlock the air lock event.
#[derive(Event)]
pub struct AirlockUnlock {
    pub handle_option: Option<ClientId>,
    pub locked: Entity,
    pub locker: Entity,
}

pub enum LockedStatus {
    Open,
    Closed,
    None,
}
/// Air lock open request event.

pub struct AirlockCloseRequest {
    pub interacter_option: Option<Entity>,
    pub interacted: Entity,
}
