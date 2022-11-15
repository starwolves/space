use bevy::{
    prelude::{
        AssetServer, BuildChildren, ButtonBundle, Camera2dBundle, Color, Commands, Entity,
        EventReader, EventWriter, NodeBundle, Res, ResMut, TextBundle,
    },
    text::TextStyle,
    ui::{
        AlignContent, AlignItems, FlexDirection, FlexWrap, JustifyContent, Size, Style, UiRect, Val,
    },
};

/// Event.
#[cfg(feature = "client")]
pub struct EnableMainMenu {
    pub enable: bool,
}

/// Resource containing the main menu state.
#[derive(Default)]
#[cfg(feature = "client")]
pub struct MainMenuState {
    pub enabled: bool,
    pub root: Option<Entity>,
    pub camera: Option<Entity>,
}

/// Shows main menu when the client starts.
#[cfg(feature = "client")]
pub(crate) fn startup_show_menu(mut enable_events: EventWriter<EnableMainMenu>) {
    enable_events.send(EnableMainMenu { enable: true });
}

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

/// System that toggles the visiblity of the main menu based on an event.
#[cfg(feature = "client")]
pub(crate) fn show_main_menu(
    mut enable_events: EventReader<EnableMainMenu>,
    mut state: ResMut<MainMenuState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if state.enabled {
        return;
    }

    for event in enable_events.iter() {
        if !event.enable {
            continue;
        }

        let camera_entity = commands
            .spawn()
            .insert_bundle(Camera2dBundle::default())
            .id();

        state.camera = Some(camera_entity);

        let mut builder = commands.spawn();

        let entity = builder.id();

        let button_padding = UiRect::new(
            Val::Undefined,
            Val::Undefined,
            Val::Undefined,
            Val::Percent(2.),
        );

        let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");

        builder
            .insert_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                color: Color::GRAY.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                            flex_wrap: FlexWrap::Wrap,
                            // Vertically align children.
                            align_content: AlignContent::FlexEnd,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::new(
                                Val::Undefined,
                                Val::Undefined,
                                Val::Percent(2.5),
                                Val::Undefined,
                            ),
                            ..Default::default()
                        },
                        color: NORMAL_BUTTON.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn()
                            .insert_bundle(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    flex_wrap: FlexWrap::Wrap,
                                    flex_direction: FlexDirection::Column,
                                    ..Default::default()
                                },
                                color: NORMAL_BUTTON.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn()
                                    .insert_bundle(ButtonBundle {
                                        color: NORMAL_BUTTON.into(),
                                        style: Style {
                                            padding: button_padding,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn().insert_bundle(TextBundle::from_section(
                                            "Exit",
                                            TextStyle {
                                                font: arizone_font.clone(),
                                                font_size: 20.0,
                                                color: TEXT_COLOR,
                                            },
                                        ));
                                    });
                                parent
                                    .spawn()
                                    .insert_bundle(ButtonBundle {
                                        color: NORMAL_BUTTON.into(),
                                        style: Style {
                                            padding: button_padding,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn().insert_bundle(TextBundle::from_section(
                                            "Settings",
                                            TextStyle {
                                                font: arizone_font.clone(),
                                                font_size: 20.0,
                                                color: TEXT_COLOR,
                                            },
                                        ));
                                    });
                                parent
                                    .spawn()
                                    .insert_bundle(ButtonBundle {
                                        color: NORMAL_BUTTON.into(),
                                        style: Style {
                                            padding: button_padding,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn().insert_bundle(TextBundle::from_section(
                                            "Play",
                                            TextStyle {
                                                font: arizone_font.clone(),
                                                font_size: 20.0,
                                                color: TEXT_COLOR,
                                            },
                                        ));
                                    });
                            });

                        parent
                            .spawn()
                            .insert_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Undefined),
                                    align_items: AlignItems::Center,
                                    flex_wrap: FlexWrap::Wrap,
                                    margin: UiRect::new(
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Percent(25.),
                                    ),
                                    flex_direction: FlexDirection::Column,
                                    ..Default::default()
                                },
                                color: NORMAL_BUTTON.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent.spawn().insert_bundle(TextBundle::from_section(
                                    "    ",
                                    TextStyle {
                                        font_size: 18.0,
                                        color: TEXT_COLOR,
                                        font: asset_server
                                            .load("fonts/FontAwesome6Free-Solid-900.otf"),
                                    },
                                ));
                                parent.spawn().insert_bundle(
                                    TextBundle::from_section(
                                        "SpaceFrontiers",
                                        TextStyle {
                                            font_size: 26.0,
                                            color: TEXT_COLOR,
                                            font: arizone_font.clone(),
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::new(
                                            Val::Undefined,
                                            Val::Undefined,
                                            Val::Undefined,
                                            Val::Percent(3.),
                                        ),
                                        ..Default::default()
                                    }),
                                );
                            });
                    });
            });

        state.enabled = true;
        state.root = Some(entity);
    }
}
