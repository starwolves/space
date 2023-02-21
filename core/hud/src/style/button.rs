use bevy::{
    prelude::{Button, Changed, Color, Component, Query, With},
    ui::{BackgroundColor, Interaction},
};

use crate::inventory::actions::{ACTIONS_HUD_BG_COLOR, INVENTORY_HUD_BG_COLOR};

#[derive(Component)]
pub struct ButtonSelectionStyle;
pub(crate) fn button_style_events(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<ButtonSelectionStyle>,
        ),
    >,
) {
    for (interaction, mut bg_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                let mut midnight = ACTIONS_HUD_BG_COLOR;
                midnight.set_a(1.);
                *bg_color = midnight.into();
            }
            Interaction::Hovered => {
                let gray = INVENTORY_HUD_BG_COLOR;
                *bg_color = gray.into();
            }
            Interaction::None => {
                let mut gray = Color::GRAY;
                gray.set_a(1.);
                *bg_color = gray.into();
            }
        }
    }
}
