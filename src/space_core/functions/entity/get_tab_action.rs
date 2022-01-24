use std::sync::Arc;

use crate::space_core::{components::pawn::TabAction, resources::{network_messages::GridMapType}};

pub fn get_tab_action(
    id : &str,
) -> Option<TabAction> {

    let result;

    if id == "examine" {

        result = Some(TabAction {
            id: id.to_string(),
            text: "Examine".to_string(),
            tab_list_priority: u8::MAX,
            prerequisite_check: Arc::new(examine_tab_prerequisite_check),
        });

    } else {
        result = None;
    }

    result

}

pub fn examine_tab_prerequisite_check(
    entity_id_bits_option : Option<u64>,
    cell_id_option : Option<(GridMapType, i16,i16,i16)>,
) -> bool {
    cell_id_option.is_some() || entity_id_bits_option.is_some()
}
