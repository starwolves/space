use bevy::{
    color::Srgba,
    prelude::{Button, Changed, Component, Query, With},
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
            Interaction::Pressed => {
                let mut midnight = ACTIONS_HUD_BG_COLOR;
                midnight = bevy::prelude::Color::Srgba(Srgba {
                    alpha: 1.0,
                    ..Srgba::from(midnight)
                });

                *bg_color = midnight.into();
            }
            Interaction::Hovered => {
                let gray = INVENTORY_HUD_BG_COLOR;
                *bg_color = gray.into();
            }
            Interaction::None => {
                let mut gray = bevy::color::palettes::css::GRAY;
                if style.selected {
                    gray = INVENTORY_HUD_BG_COLOR.into();
                    gray = bevy::prelude::Color::Srgba(Srgba {
                        alpha: 1.0,
                        ..Srgba::from(gray)
                    })
                    .into();
                }
                gray = bevy::prelude::Color::Srgba(Srgba {
                    alpha: 1.0,
                    ..Srgba::from(gray)
                })
                .into();
                *bg_color = gray.into();
            }
        }
    }
}
pub(crate) fn changed_focus(
    mut query: Query<(&mut BackgroundColor, &ButtonSelectionStyle), Changed<ButtonSelectionStyle>>,
) {
    for (mut bg, style) in query.iter_mut() {
        let mut gray = bevy::color::palettes::css::GRAY;
        if style.selected {
            gray = ACTIONS_HUD_BG_COLOR.into();
            gray = bevy::prelude::Color::Srgba(Srgba {
                alpha: 1.0,
                ..Srgba::from(gray)
            })
            .into()
        }
        *bg = gray.into();
    }
}
