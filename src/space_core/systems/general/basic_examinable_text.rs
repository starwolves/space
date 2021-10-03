use bevy::{prelude::{Changed, Or, Query}};

use crate::space_core::{components::{examinable::Examinable, health::Health}, functions::entity::new_chat_message::{ASTRIX, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR}};

pub fn basic_examinable_text(

    mut changed_entities : Query<(&Health,&mut Examinable), Or<(Changed<Health>, Changed<Examinable>)>>

) {

    for (health_component, mut examinable_component) in changed_entities.iter_mut() {

        if examinable_component.custom_generator {
            continue;
        }

        match &health_component.health_container {
            crate::space_core::components::health::HealthContainer::Entity(entity_container) => {

                let mut examinable_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n";
                for (_text_id, assigned_text) in examinable_component.assigned_texts.iter() {
                    examinable_text = examinable_text + assigned_text;
                }

                if entity_container.brute < 25. && entity_container.burn < 25. && entity_container.toxin < 25. {

                    examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + HEALTHY_COLOR + "]\n\nIt is in perfect shape.[/color][/font]";

                } else {

                    if entity_container.brute > 75. {
                        examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\n\nIt is heavily damaged.[/color][/font]";
                    } else if entity_container.brute > 50. {
                        examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\n\nIt is damaged.[/color][/font]";
                    } else if entity_container.brute > 25. {
                        examinable_text = examinable_text + "[font=" + FURTHER_ITALIC_FONT + "][color=" + UNHEALTHY_COLOR + "]\n\nIt is slightly damaged.[/color][/font]";
                    }

                }
                
                examinable_text = examinable_text + "\n" + ASTRIX + "[/font]";

                examinable_component.examinable_text = examinable_text;

            },
            _=>(),
        }

    }

}
