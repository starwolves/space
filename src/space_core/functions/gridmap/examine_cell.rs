
use bevy::prelude::{Res};

use crate::space_core::{functions::entity::new_chat_message::{ASTRIX, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR}, resources::{gridmap_data::GridmapData, gridmap_main::CellData, network_messages::GridMapType}};

pub const EXAMINATION_EMPTY : &str = "You cannot see what is there.";

pub fn examine_ship_cell(
    ship_cell : &CellData,
    gridmap_type : &GridMapType,
    gridmap_data : &Res<GridmapData>,
) -> String {

    let examine_text : &str;
        
    if ship_cell.item != -1 {
        match gridmap_type {
            GridMapType::Main => {
                examine_text = gridmap_data.main_text_examine_desc.get(&ship_cell.item).unwrap();
            },
            GridMapType::Details1 => {
                examine_text = gridmap_data.details1_text_examine_desc.get(&ship_cell.item).unwrap();
            },
        }
        
    } else {
        examine_text = EXAMINATION_EMPTY;
    }

    let mut message = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n"
    + examine_text;

    if ship_cell.health.brute < 25. && ship_cell.health.burn < 25. && ship_cell.health.toxin < 25. {

        message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + HEALTHY_COLOR + "]\nIt is in perfect shape.[/color][/font]";

    } else {

        if ship_cell.health.brute > 75. {
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt is heavily damaged.[/color][/font]";
        } else if ship_cell.health.brute > 50. {
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt is damaged.[/color][/font]";
        } else if ship_cell.health.brute > 25. {
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt is slightly damaged.[/color][/font]";
        }

        if ship_cell.health.burn > 75. {
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt suffers from heavy burn damage.[/color][/font]";
        } else if ship_cell.health.burn > 50. {
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt suffers burn damage.[/color][/font]";
        } else if ship_cell.health.burn > 25. {
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt has slight burn damage.[/color][/font]";
        }

    }

    message = message + "\n" + ASTRIX + "[/font]";

    message

}

pub fn get_empty_cell_message() -> String {
    "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n"
    + EXAMINATION_EMPTY
    + "\n" + ASTRIX + "[/font]"
}
