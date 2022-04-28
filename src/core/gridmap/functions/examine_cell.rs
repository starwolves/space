use bevy_ecs::system::Res;
use const_format::concatcp;
use rand::Rng;

use crate::core::{
    chat::functions::{
        ASTRIX, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR,
    },
    gridmap::resources::{CellData, GridmapData},
    networking::resources::GridMapType,
};

pub const EXAMINATION_EMPTY: &str = "You cannot see what is there.";
pub const END_ASTRIX: &str = concatcp!("\n", ASTRIX, "[/font]");

pub fn examine_ship_cell(
    ship_cell: &CellData,
    gridmap_type: &GridMapType,
    gridmap_data: &Res<GridmapData>,
) -> String {
    let examine_text: &str;
    let mut message = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n";
    message = message
        + "[font="
        + FURTHER_ITALIC_FONT
        + "]"
        + "You examine the "
        + &gridmap_data
            .main_text_names
            .get(&ship_cell.item)
            .unwrap()
            .get_name()
        + ".[/font]\n";

    if ship_cell.item != -1 {
        match gridmap_type {
            GridMapType::Main => {
                examine_text = gridmap_data
                    .main_text_examine_desc
                    .get(&ship_cell.item)
                    .unwrap();
            }
            GridMapType::Details1 => {
                examine_text = gridmap_data
                    .details1_text_examine_desc
                    .get(&ship_cell.item)
                    .unwrap();
            }
        }
    } else {
        examine_text = EXAMINATION_EMPTY;
    }

    message = message + examine_text;

    if ship_cell.health.brute < 25. && ship_cell.health.burn < 25. && ship_cell.health.toxin < 25. {
        message = message
            + "[font="
            + FURTHER_ITALIC_FONT
            + "][color="
            + HEALTHY_COLOR
            + "]\nIt is in perfect shape.[/color][/font]";
    } else {
        if ship_cell.health.brute > 75. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt is heavily damaged.[/color][/font]";
        } else if ship_cell.health.brute > 50. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt is damaged.[/color][/font]";
        } else if ship_cell.health.brute > 25. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt is slightly damaged.[/color][/font]";
        }

        if ship_cell.health.burn > 75. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt suffers from heavy burn damage.[/color][/font]";
        } else if ship_cell.health.burn > 50. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt suffers burn damage.[/color][/font]";
        } else if ship_cell.health.burn > 25. {
            message = message
                + "[font="
                + FURTHER_ITALIC_FONT
                + "][color="
                + UNHEALTHY_COLOR
                + "]\nIt has slight burn damage.[/color][/font]";
        }
    }

    message
}

pub fn get_empty_cell_message() -> String {
    "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n" + EXAMINATION_EMPTY
}

pub fn get_space_message() -> String {
    let mut rng = rand::thread_rng();
    let random_pick: i32 = rng.gen_range(0..3);

    let mut msg = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n";
    msg = msg + "[font=" + FURTHER_ITALIC_FONT + "]" + "You examine the empty space.[/font]\n";

    if random_pick == 0 {
        msg = msg + "You are starstruck by the sight of space.";
    } else if random_pick == 1 {
        msg = msg + "That certainly looks like space.";
    } else {
        msg = msg + "Space.";
    }

    msg.to_string()
}
