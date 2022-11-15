use crate::build::NORMAL_BUTTON;
use bevy::prelude::{Changed, Color};
use bevy::ui::UiColor;
use bevy::{
    prelude::{Button, Query, With},
    ui::Interaction,
};

pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.49, 0.73, 0.91);

#[cfg(feature = "client")]
pub(crate) fn hover_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
