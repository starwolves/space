use crate::diffusion::{get_atmos_index, AtmosphericsResource, CELCIUS_KELVIN_OFFSET};
use bevy::prelude::{warn, Query, Res, ResMut};
use chat_api::core::ATMOSPHERICS_TEXT_COLOR;
use chat_api::core::FURTHER_ITALIC_FONT;
use entity::senser::Senser;
use entity::senser::SensingAbility;
use math::grid::Vec2Int;
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetAtmosphericsMapExamine {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
use gridmap::examine::GridmapExamineMessages;

/// Examine text with data for cells with atmospherics.
#[cfg(feature = "server")]
pub(crate) fn examine_map_atmos(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    senser_entities: Query<&Senser>,
    atmospherics_resource: Res<AtmosphericsResource>,
) {
    for examine_event in examine_map_events.messages.iter_mut() {
        let examiner_senser_component;

        match senser_entities.get(examine_event.entity) {
            Ok(examiner_senser) => {
                examiner_senser_component = examiner_senser;
            }
            Err(_rr) => {
                warn!("Couldn't find examiner entity in &Senser query.");
                continue;
            }
        }

        let mut examine_text = "".to_string();

        for sensing_ability in examiner_senser_component.sensing_abilities.iter() {
            match sensing_ability {
                SensingAbility::AtmosphericsSensor => {
                    let id = Vec2Int {
                        x: examine_event.gridmap_cell_id.x,
                        y: examine_event.gridmap_cell_id.z,
                    };

                    if AtmosphericsResource::is_id_out_of_range(id) {
                        continue;
                    }

                    let atmospherics = atmospherics_resource
                        .atmospherics
                        .get(get_atmos_index(id))
                        .unwrap();

                    if atmospherics.blocked {
                        continue;
                    }

                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + ATMOSPHERICS_TEXT_COLOR
                        + "]"
                        + "\n"
                        + "Atmospherics DataLink: [/color]"
                        + "\n"
                        + "Temperature: "
                        + &(atmospherics.temperature - CELCIUS_KELVIN_OFFSET)
                            .floor()
                            .to_string()
                        + " c\n"
                        + "Pressure: "
                        + &atmospherics.get_pressure().floor().to_string()
                        + " kpa"
                        + "[/font]";
                }
                _ => (),
            }
        }
        examine_event.message = examine_event.message.clone() + &examine_text;
    }
}
