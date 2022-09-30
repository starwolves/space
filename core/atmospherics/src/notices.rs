use std::collections::HashMap;

use bevy::prelude::{Entity, EventWriter, Local, Query, Res, Transform};
use math::grid::{world_to_cell_id, Vec2Int};
use networking::messages::ReliableServerMessage;
use pawn::pawn::Pawn;
use server::core::ConnectedPlayer;

use crate::{
    diffusion::{get_atmos_index, AtmosphericsResource},
    zero_gravity::ZeroGravity,
};

use super::{
    map_events::{
        MAXIMUM_LIVABLE_PRESSURE, MAXIMUM_LIVABLE_TEMPERATURE, MINIMUM_LIVABLE_PRESSURE,
        MINIMUM_LIVABLE_TEMPERATURE,
    },
    net::NetAtmosphericsNotices,
};

/// Resource with atmospherics notices that warn players when they are under certain atmospherics conditions like unlivable atmospherics conditions.
#[derive(Default)]
pub struct AtmosphericsNotices {
    cache: HashMap<Entity, Vec<String>>,
}

/// Manage visible atmospherics notices for clients.
pub(crate) fn atmospherics_notices(
    mut net: EventWriter<NetAtmosphericsNotices>,
    atmospherics_resource: Res<AtmosphericsResource>,
    pawns: Query<(
        Entity,
        &Pawn,
        &Transform,
        &ConnectedPlayer,
        Option<&ZeroGravity>,
    )>,
    mut atmos_notices: Local<AtmosphericsNotices>,
) {
    for (
        pawn_entity,
        _pawn_component,
        rigid_body_position_component,
        connected_player_component,
        zero_gravity_component_option,
    ) in pawns.iter()
    {
        let cached_atmos_notices;

        match atmos_notices.cache.get_mut(&pawn_entity) {
            Some(n) => {
                cached_atmos_notices = n;
            }
            None => {
                atmos_notices.cache.insert(pawn_entity, vec![]);
                cached_atmos_notices = atmos_notices.cache.get_mut(&pawn_entity).unwrap();
            }
        }

        let cell_id = world_to_cell_id(rigid_body_position_component.translation);

        let atmospherics = atmospherics_resource
            .atmospherics
            .get(get_atmos_index(Vec2Int {
                x: cell_id.x,
                y: cell_id.z,
            }))
            .unwrap();

        let pressure = atmospherics.get_pressure();

        let mut new_notices = vec![];

        if pressure < MINIMUM_LIVABLE_PRESSURE {
            new_notices.push("coldNotice".to_string());
        } else if pressure > MAXIMUM_LIVABLE_PRESSURE {
            new_notices.push("hotNotice".to_string());
        }

        if atmospherics.temperature < MINIMUM_LIVABLE_TEMPERATURE {
            new_notices.push("lowpkaNotice".to_string());
        } else if atmospherics.temperature > MAXIMUM_LIVABLE_TEMPERATURE {
            new_notices.push("highpkaNotice".to_string());
        }

        if zero_gravity_component_option.is_some() {
            new_notices.push("zeroGravityNotice".to_string());
        }

        let mut added_notices = vec![];
        let mut removed_notices = vec![];

        for new_notice in new_notices.iter() {
            if !cached_atmos_notices.contains(&new_notice) {
                added_notices.push(new_notice.clone());
                cached_atmos_notices.push(new_notice.clone());
            }
        }
        let mut i = 0;
        let mut remove_is = vec![];
        for cached_notice in cached_atmos_notices.iter() {
            if !new_notices.contains(cached_notice) {
                removed_notices.push(cached_notice.clone());
                remove_is.push(i);
            }
            i += 1;
        }
        remove_is.reverse();
        for j in remove_is {
            cached_atmos_notices.remove(j);
        }

        for add in added_notices {
            net.send(NetAtmosphericsNotices {
                handle: connected_player_component.handle,
                message: ReliableServerMessage::UIAddNotice(add),
            });
        }
        for remove in removed_notices {
            net.send(NetAtmosphericsNotices {
                handle: connected_player_component.handle,
                message: ReliableServerMessage::UIRemoveNotice(remove),
            });
        }
    }
}
