use bevy::prelude::{warn, Query, ResMut};
use shared::{
    chat::{ENGINEERING_TEXT_COLOR, FURTHER_ITALIC_FONT},
    gridmap::GridmapExamineMessages,
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    senser::{Senser, SensingAbility},
};

pub struct NetPawnExamine {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetPawnExamine {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

pub fn examine_map(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    senser_entities: Query<&Senser>,
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
                SensingAbility::ShipEngineerKnowledge => {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + ENGINEERING_TEXT_COLOR
                        + "]"
                        + "\n"
                        + "Ship Engineer Knowledge: [/color]"
                        + "\n"
                        + "Reference shows coordinates ("
                        + &examine_event.gridmap_cell_id.x.to_string()
                        + " , "
                        + &examine_event.gridmap_cell_id.z.to_string()
                        + ")."
                        + "[/font]\n";
                }
                _ => (),
            }
        }
        examine_event.message = examine_event.message.clone() + &examine_text;
    }
}
