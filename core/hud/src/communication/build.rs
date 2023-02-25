use bevy::{
    prelude::{BuildChildren, Color, Commands, NodeBundle, Res},
    ui::{FlexDirection, Size, Style, Val},
};
use resources::hud::HudState;

pub(crate) fn build_communication_ui(hud_state: Res<HudState>, mut commands: Commands) {
    commands
        .entity(hud_state.left_edge_node)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(25.)),
                            ..Default::default()
                        },
                        background_color: Color::WHITE.into(),
                        ..Default::default()
                    });
                });
        });
}
