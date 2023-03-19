use bevy::{
    prelude::{
        AssetServer, BuildChildren, ButtonBundle, Color, Commands, Component, Entity, NodeBundle,
        Res, Resource, TextBundle,
    },
    text::TextStyle,
    ui::{
        AlignItems, Display, FlexDirection, FlexWrap, JustifyContent, Overflow, Size, Style, Val,
    },
};
use resources::binds::KeyBinds;
use ui::{
    button::ButtonVisuals,
    fonts::{ARIZONE_FONT, SOURCECODE_REGULAR_FONT},
};

#[derive(Component)]
pub struct EscapeMenuRoot;
#[derive(Component)]
pub struct GeneralSection;
#[derive(Component)]
pub struct ControlsSection;
#[derive(Component)]
pub struct GraphicsSection;

#[derive(Resource)]
pub struct EscapeMenuState {
    pub root: Entity,
    pub visible: bool,
    pub controls_section: Entity,
    pub graphics_section: Entity,
    pub general_section: Entity,
}
#[derive(Component)]
pub struct ControlsHeaderButton;
#[derive(Component)]
pub struct GeneralHeaderButton;
#[derive(Component)]
pub struct GraphicsHeaderButton;

pub(crate) fn build_escape_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sourcecode_font = asset_server.load(ARIZONE_FONT);
    let mut controls_section_entity = Entity::from_bits(0);
    let mut graphics_section_entity = Entity::from_bits(0);
    let mut general_section_entity = Entity::from_bits(0);

    let escape_root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EscapeMenuRoot)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(60.0)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.6, 0.73, 1., 0.6).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(7.)),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_wrap: FlexWrap::Wrap,
                                ..Default::default()
                            },
                            background_color: Color::DARK_GRAY.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(33.333333),
                                            Val::Percent(100.),
                                        ),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.),
                                                    Val::Percent(100.),
                                                ),
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(ButtonVisuals::default())
                                        .insert(ControlsHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Controls".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(33.333333),
                                            Val::Percent(100.),
                                        ),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.),
                                                    Val::Percent(100.),
                                                ),
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(ButtonVisuals::default())
                                        .insert(GeneralHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "General".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(
                                            Val::Percent(33.333333),
                                            Val::Percent(100.),
                                        ),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ButtonBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.),
                                                    Val::Percent(100.),
                                                ),
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                flex_wrap: FlexWrap::Wrap,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(ButtonVisuals::default())
                                        .insert(GraphicsHeaderButton)
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Graphics".to_string(),
                                                TextStyle {
                                                    font: sourcecode_font.clone(),
                                                    font_size: 12.0,
                                                    color: Color::WHITE.into(),
                                                },
                                            ));
                                        });
                                });
                        });
                    controls_section_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::Hidden,

                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(ControlsSection)
                        .id();
                    graphics_section_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(GraphicsSection)
                        .id();
                    general_section_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(GeneralSection)
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_wrap: FlexWrap::Wrap,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.), Val::Percent(6.)),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        flex_wrap: FlexWrap::Wrap,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(35.),
                                                    Val::Percent(100.),
                                                ),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn(ButtonBundle {
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Percent(100.),
                                                            Val::Percent(100.),
                                                        ),
                                                        flex_direction: FlexDirection::Row,
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        flex_wrap: FlexWrap::Wrap,
                                                        ..Default::default()
                                                    },
                                                    ..Default::default()
                                                })
                                                .insert(ButtonVisuals::default())
                                                .insert(ExitGameButton)
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle::from_section(
                                                        "Exit Game".to_string(),
                                                        TextStyle {
                                                            font: sourcecode_font.clone(),
                                                            font_size: 12.0,
                                                            color: Color::WHITE.into(),
                                                        },
                                                    ));
                                                });
                                        });
                                });
                        })
                        .id();
                });
        })
        .id();
    commands.insert_resource(EscapeMenuState {
        root: escape_root,
        visible: false,
        controls_section: controls_section_entity,
        graphics_section: graphics_section_entity,
        general_section: general_section_entity,
    })
}
#[derive(Component)]
pub struct ExitGameButton;
#[derive(Component)]
pub struct BindButton {
    pub bind_id: String,
}

pub(crate) fn build_controls_section(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<EscapeMenuState>,
    binds: Res<KeyBinds>,
) {
    let source_code = asset_server.load(SOURCECODE_REGULAR_FONT);
    commands
        .entity(state.controls_section)
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_wrap: FlexWrap::Wrap,

                    ..Default::default()
                },
                ..Default::default()
            });

            for (bind_id, bind) in binds.list.iter() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(60.), Val::Auto),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_wrap: FlexWrap::Wrap,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            bind.name.clone(),
                            TextStyle {
                                font: source_code.clone(),
                                font_size: 12.0,
                                color: Color::WHITE.into(),
                            },
                        ));
                        parent.spawn(TextBundle::from_section(
                            bind.description.clone(),
                            TextStyle {
                                font: source_code.clone(),
                                font_size: 12.0,
                                color: Color::WHITE.into(),
                            },
                        ));
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(10.), Val::Auto),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.), Val::Auto),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            flex_wrap: FlexWrap::Wrap,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .insert(ButtonVisuals::default())
                                    .insert(BindButton {
                                        bind_id: bind_id.clone(),
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            format!("{:?}", bind.key_code),
                                            TextStyle {
                                                font: source_code.clone(),
                                                font_size: 12.0,
                                                color: Color::WHITE.into(),
                                            },
                                        ));
                                    });
                            });
                    });
            }
        });
}
