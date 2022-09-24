use api::{
    chat::{ATMOSPHERICS_TEXT_COLOR, FURTHER_ITALIC_FONT},
    data::Vec2Int,
    gridmap::{get_atmos_index, GridmapExamineMessages},
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    senser::{Senser, SensingAbility},
};
use bevy::prelude::{warn, Query, Res, ResMut};

use crate::diffusion::{AtmosphericsResource, CELCIUS_KELVIN_OFFSET};

pub(crate) struct NetAtmosphericsMapExamine {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetAtmosphericsMapExamine {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Examine text for cells with atmospherics.
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
