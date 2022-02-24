use bevy::prelude::Query;

use crate::space::core::{
    entity::components::Examinable,
    health::components::Health,
    inventory::components::Inventory,
    pawn::functions::new_chat_message::{
        ASTRIX, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR,
    },
};

pub fn generate_human_examine_text(
    character_name: &str,
    inventory_component_option: Option<&Inventory>,
    examinables: &Query<&Examinable>,
    health_component: &Health,
) -> String {
    let mut examine_text = "[font=".to_owned()
        + FURTHER_NORMAL_FONT
        + "]"
        + ASTRIX
        + "\n"
        + character_name
        + ", a Security Officer.\n"
        + "He is human.\n";

    match &health_component.health_container {
        crate::space::core::health::components::HealthContainer::Humanoid(humanoid_container) => {
            let head_damage = humanoid_container.head_brute
                + humanoid_container.head_burn
                + humanoid_container.head_toxin;
            let torso_damage = humanoid_container.torso_brute
                + humanoid_container.torso_burn
                + humanoid_container.torso_toxin;
            let left_arm_damage = humanoid_container.left_arm_brute
                + humanoid_container.left_arm_burn
                + humanoid_container.left_arm_toxin;
            let right_arm_damage = humanoid_container.right_arm_brute
                + humanoid_container.right_arm_burn
                + humanoid_container.right_arm_toxin;
            let left_leg_damage = humanoid_container.left_leg_brute
                + humanoid_container.left_leg_burn
                + humanoid_container.left_leg_toxin;
            let right_leg_damage = humanoid_container.right_leg_brute
                + humanoid_container.right_leg_burn
                + humanoid_container.right_leg_toxin;

            if head_damage < 25.
                && torso_damage < 25.
                && left_arm_damage < 25.
                && right_arm_damage < 25.
                && left_leg_damage < 25.
                && right_leg_damage < 25.
            {
                examine_text = examine_text
                    + "[font="
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]He is in perfect shape.[/color][/font]\n";
            } else {
                if humanoid_container.head_brute > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His head is heavily injured.[/color][/font]\n";
                } else if humanoid_container.head_brute > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His head is injured.[/color][/font]\n";
                } else if humanoid_container.head_brute > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His head is bruised.[/color][/font]\n";
                }

                if humanoid_container.torso_brute > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His torso is heavily injured.[/color][/font]\n";
                } else if humanoid_container.torso_brute > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His torso is injured.[/color][/font]\n";
                } else if humanoid_container.torso_brute > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His torso is bruised.[/color][/font]\n";
                }

                if humanoid_container.left_arm_brute > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left arm is heavily injured.[/color][/font]\n";
                } else if humanoid_container.left_arm_brute > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left arm is injured.[/color][/font]\n";
                } else if humanoid_container.left_arm_brute > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left arm is bruised.[/color][/font]\n";
                }

                if humanoid_container.right_arm_brute > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right arm is heavily injured.[/color][/font]\n";
                } else if humanoid_container.right_arm_brute > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right arm is injured.[/color][/font]\n";
                } else if humanoid_container.right_arm_brute > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right arm is bruised.[/color][/font]\n";
                }

                if humanoid_container.left_leg_brute > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left leg is heavily injured.[/color][/font]\n";
                } else if humanoid_container.left_leg_brute > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left leg is injured.[/color][/font]\n";
                } else if humanoid_container.left_leg_brute > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left leg is bruised.[/color][/font]\n";
                }

                if humanoid_container.right_leg_brute > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right leg is heavily injured.[/color][/font]\n";
                } else if humanoid_container.right_leg_brute > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right leg is injured.[/color][/font]\n";
                } else if humanoid_container.right_leg_brute > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right leg is bruised.[/color][/font]\n";
                }

                if humanoid_container.head_burn > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His head has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.head_burn > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His head has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.head_burn > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His head has visible first degree burns.[/color][/font]\n";
                }

                if humanoid_container.torso_burn > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His torso has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.torso_burn > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His torso has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.torso_burn > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His torso has visible first degree burns.[/color][/font]\n";
                }

                if humanoid_container.left_arm_burn > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left arm has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.left_arm_burn > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left arm has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.left_arm_burn > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left arm has visible first degree burns.[/color][/font]\n";
                }

                if humanoid_container.right_arm_burn > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right arm has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.right_arm_burn > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right arm has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.right_arm_burn > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right arm has visible first degree burns.[/color][/font]\n";
                }

                if humanoid_container.left_leg_burn > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left leg has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.left_leg_burn > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left leg has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.left_leg_burn > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His left leg has visible first degree burns.[/color][/font]\n";
                }

                if humanoid_container.right_leg_burn > 75. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right leg has visible third degree burns, ouch![/color][/font]\n";
                } else if humanoid_container.right_leg_burn > 50. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right leg has visible second degree burns.[/color][/font]\n";
                } else if humanoid_container.right_leg_burn > 25. {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + UNHEALTHY_COLOR
                        + "]His right leg has visible first degree burns.[/color][/font]\n";
                }
            }
        }
        _ => (),
    }

    match inventory_component_option {
        Some(inventory_component) => {
            examine_text = examine_text + "\n";
            for slot in inventory_component.slots.iter() {
                match slot.slot_item {
                    Some(slot_item_entity) => {
                        let examinable = examinables.get(slot_item_entity)
                        .expect("inventory_update.rs::generate_human_examine_text couldn't find inventory_item_component of an item from passed inventory.");

                        if slot.slot_name == "left_hand" {
                            examine_text = examine_text
                                + "He is holding "
                                + &examinable.name.get_a_name()
                                + " in his left hand.\n";
                        } else if slot.slot_name == "right_hand" {
                            examine_text = examine_text
                                + "He is holding "
                                + &examinable.name.get_a_name()
                                + " in his right hand.\n";
                        } else if slot.slot_name == "helmet" {
                            examine_text = examine_text
                                + "He is wearing "
                                + &examinable.name.get_a_name()
                                + " on his head.\n";
                        } else if slot.slot_name == "jumpsuit" {
                            examine_text = examine_text
                                + "He is wearing "
                                + &examinable.name.get_a_name()
                                + " on his body.\n";
                        } else if slot.slot_name == "holster" {
                            examine_text = examine_text
                                + &examinable.name.get_a_name()
                                + " is attached to his holster.\n";
                        } else {
                            examine_text = examine_text
                                + "He is wearing "
                                + &examinable.name.get_a_name()
                                + ".\n";
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }

    examine_text = examine_text + ASTRIX + "[/font]";

    examine_text
}
