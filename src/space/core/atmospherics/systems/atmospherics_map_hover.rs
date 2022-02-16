use bevy::prelude::{Entity, EventWriter, Query, Res, ResMut};

use crate::space::core::{
    atmospherics::{
        events::NetMapHoverAtmospherics,
        functions::get_atmos_index,
        resources::{AtmosphericsResource, MapHolders, CELCIUS_KELVIN_OFFSET},
    },
    gridmap::resources::Vec2Int,
    map::components::Map,
    networking::resources::{NetMessageType, ReliableServerMessage},
    pawn::components::ConnectedPlayer,
};

pub fn atmospherics_map_hover(
    map_holders: Query<(Entity, &Map, &ConnectedPlayer)>,
    atmospherics: Res<AtmosphericsResource>,
    mut display_atmos_state: ResMut<MapHolders>,
    mut net: EventWriter<NetMapHoverAtmospherics>,
) {
    for (map_holder_entity, map_component, connected_player_component) in map_holders.iter() {
        match map_component.passed_mouse_cell {
            Some((idx, idy)) => {
                let id = Vec2Int { x: idx, y: idy };

                if AtmosphericsResource::is_id_out_of_range(id) {
                    continue;
                }

                let cell_i = get_atmos_index(id);

                let cell_atmos = atmospherics.atmospherics.get(cell_i).unwrap();

                let data;

                if cell_atmos.blocked {
                    data = "".to_string();
                } else {
                    data = "Temperature: ".to_owned()
                        + &(cell_atmos.temperature - CELCIUS_KELVIN_OFFSET)
                            .floor()
                            .to_string()
                        + " c\n"
                        + "Pressure: "
                        + &cell_atmos.get_pressure().floor().to_string()
                        + " kpa";
                }

                match display_atmos_state.holders.get_mut(&map_holder_entity) {
                    Some(map_holder_data) => {
                        if map_holder_data.hovering_data != data {
                            net.send(NetMapHoverAtmospherics {
                                handle: connected_player_component.handle,
                                message: NetMessageType::Reliable(
                                    ReliableServerMessage::MapOverlayHoverData(data.to_string()),
                                ),
                            });
                            map_holder_data.hovering_data = data;
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}
