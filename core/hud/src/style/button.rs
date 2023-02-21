use bevy::{
    prelude::{Button, Changed, Color, Component, Query, With},
    ui::{BackgroundColor, Interaction},
};

use crate::inventory::actions::{ACTIONS_HUD_BG_COLOR, INVENTORY_HUD_BG_COLOR};

#[derive(Component, Default)]
pub struct ButtonSelectionStyle {
    pub selected: bool,
}
pub(crate) fn button_style_events(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonSelectionStyle),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg_color, style) in interaction_query.iter_mut() {
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
                if style.selected {
                    gray = INVENTORY_HUD_BG_COLOR;
                    gray.set_a(1.);
                }
                gray.set_a(1.);
                *bg_color = gray.into();
            }
        }
    }
}
pub(crate) fn changed_focus(
    mut query: Query<(&mut BackgroundColor, &ButtonSelectionStyle), Changed<ButtonSelectionStyle>>,
) {
    for (mut bg, style) in query.iter_mut() {
        let mut gray = Color::GRAY;
        if style.selected {
            gray = ACTIONS_HUD_BG_COLOR;
            gray.set_a(1.);
        }
        *bg = gray.into();
    }
}
