
use crate::space_core::{functions::entity::new_chat_message::{ASTRIX, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR}, resources::{gridmap_main::CellData, network_messages::GridMapType}};


const EXAMINATION_MAIN : [&str;13] = [
    "An Aluminum floor. This one is painted with security department colors.",
    "An Aluminum wall. This one is painted with security department colors.",
    "You cannot see what is there.",
    "An Aluminum floor.",
    "An Aluminum wall.",
    "An Aluminum floor. This one is painted with security department colors.",
    "An Aluminum floor. This one is painted with security department colors.",
    "An Aluminum floor. This one is painted with security department colors.",
    "An Aluminum security counter.",
    "An Aluminum wall. This one is painted with security department colors.",
    "An Aluminum wall. This one is painted with security department colors.",
    "An Aluminum wall.",
    "You cannot see what is there.",
];

const EXAMINATION_DETAILS1 : [&str;13] = [
    "A fluorescent floor light.",
    "A glorious Red Dragon poster for security personnel to remind you of the collective's might. \n\"Protect\nControl\nPrevent\nSecure\"",
    "A glorious Red Dragon poster for security personnel. A nation to look up to with pride. \n\"Our\nFather\"",
    "A glorious Red Dragon poster for security personnel. This one has a famous picture printed on it from hundreds of years ago, the start of the great nation captured in a single picture. \n\"We\nRose\"",
    "A glorious Red Dragon poster. \n\"Hail our\nRed\nNation\"",
    "A poster. \n \"Colonise\nSpace\"",
    "A Red Dragon poster for security personnel. \n\"I\nServe\"",
    "You cannot see what is there.",
    "A Red Dragon poster. Here to remind you that the nation's surveillance systems have never been as effective and important as it is now. \n\"Always\nWatchful\"",
    "A liquid drain. It transports liquids through dedicated piping to a different destination.",
    "An air exhaust. Here to ventilate and circulate oxygen throughout the spaceship.",
    "An administrative personal computer (APC). Authorized personnel can use these computers to check on the status of the sub-systems this room utilises.",
    "A well-preserved ancient collectible pop music poster, it must be at least a thousand years old. \n\"Starboy\"",
];

pub const EXAMINATION_EMPTY : &str = "You cannot see what is there.";

pub fn examine_ship_cell(
    ship_cell : &CellData,
    gridmap_type : &GridMapType,
) -> String {

    let examine_text;
        
    if ship_cell.item != -1 {
        match gridmap_type {
            GridMapType::Main => {
                examine_text = EXAMINATION_MAIN[ship_cell.item as usize];
            },
            GridMapType::Details1 => {
                examine_text = EXAMINATION_DETAILS1[ship_cell.item as usize];
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
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt has suffered from heavy burn damage.[/color][/font]";
        } else if ship_cell.health.burn > 50. {
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt has suffered burn damage.[/color][/font]";
        } else if ship_cell.health.burn > 25. {
            message = message + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\nIt is has slight burn damage.[/color][/font]";
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
