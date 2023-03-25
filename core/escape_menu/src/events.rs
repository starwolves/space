use bevy::{
    app::AppExit,
    prelude::{
        info, warn, Button, Changed, Entity, EventReader, EventWriter, Input, KeyCode, Parent,
        Query, Res, ResMut, With,
    },
    ui::{Display, Interaction, Style},
};
use graphics::settings::{SetFxaa, SetMsaa, SetResolution, SetVsync, SetWindowMode};
use hud::inventory::build::OpenHud;
use num_traits::FromPrimitive;
use resources::{
    binds::{KeyBind, KeyBinds},
    hud::HudState,
};
use ui::{button::SFButton, hlist::HList, text_input::TextInputNode};

use crate::build::{
    ControlsBGSection, ControlsHeaderButton, EscapeMenuState, ExitGameButton, FxaaHList,
    GeneralHeaderButton, GeneralSection, GraphicsBGSection, GraphicsHeaderButton, MsaaHList,
    ResolutionInputApply, ResolutionXInput, ResolutionYInput, VsyncHList, WindowModeHList,
};

pub struct ToggleEscapeMenu {
    pub enabled: bool,
}

pub const ESC_MENU_BIND: &str = "escMenu";

pub(crate) fn register_input(mut keys: ResMut<KeyBinds>) {
    keys.list.insert(
        ESC_MENU_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::Escape,
            description: "Toggles the escape menu.".to_string(),
            name: "Escape Menu".to_string(),
        },
    );
}

pub(crate) fn esc_button_menu(
    mut events: EventWriter<ToggleEscapeMenu>,
    state: Res<EscapeMenuState>,
    keys: Res<KeyBinds>,
    keys2: Res<Input<KeyCode>>,
) {
    if keys2.just_pressed(keys.bind(ESC_MENU_BIND)) {
        events.send(ToggleEscapeMenu {
            enabled: !state.visible,
        });
    }
}

pub fn toggle_escape_menu(
    mut style_query: Query<&mut Style>,
    mut state: ResMut<EscapeMenuState>,
    mut events: EventReader<ToggleEscapeMenu>,
    hud_state: Res<HudState>,
    mut hud: EventWriter<OpenHud>,
    mut toggle_general: EventWriter<ToggleGeneralSection>,
    mut toggle_graphics: EventWriter<ToggleGraphicsSection>,
    mut toggle_controls: EventWriter<ToggleControlsSection>,
) {
    for toggle in events.iter() {
        state.visible = toggle.enabled;

        hud.send(OpenHud {
            open: state.visible,
        });

        let mut esc_root_style = style_query.get_mut(state.root).unwrap();
        if state.visible {
            esc_root_style.display = Display::Flex;
        } else {
            esc_root_style.display = Display::None;
        }
        let mut hud_root_style = style_query.get_mut(hud_state.root_entity).unwrap();
        if !state.visible {
            hud_root_style.display = Display::Flex;
        } else {
            hud_root_style.display = Display::None;
        }

        if state.visible {
            toggle_general.send(ToggleGeneralSection { enabled: true });
        } else {
            toggle_general.send(ToggleGeneralSection { enabled: false });
            toggle_controls.send(ToggleControlsSection { enabled: false });
            toggle_graphics.send(ToggleGraphicsSection { enabled: false });
        }
    }
}

pub struct ToggleGeneralSection {
    pub enabled: bool,
}
pub struct ToggleGraphicsSection {
    pub enabled: bool,
}
pub struct ToggleControlsSection {
    pub enabled: bool,
}

pub(crate) fn toggle_general_menu_section(
    mut events: EventReader<ToggleGeneralSection>,
    mut general_style: Query<&mut Style, With<GeneralSection>>,
    mut control_events: EventWriter<ToggleControlsSection>,
    mut graphics_events: EventWriter<ToggleGraphicsSection>,
) {
    for toggle in events.iter() {
        let mut style = general_style.get_single_mut().unwrap();
        if toggle.enabled {
            style.display = Display::Flex;
            control_events.send(ToggleControlsSection { enabled: false });
            graphics_events.send(ToggleGraphicsSection { enabled: false });
        } else {
            style.display = Display::None;
        }
    }
}

pub(crate) fn toggle_graphics_menu_section(
    mut events: EventReader<ToggleGraphicsSection>,
    mut general_style: Query<&mut Style, With<GraphicsBGSection>>,
    mut control_events: EventWriter<ToggleControlsSection>,
    mut general_events: EventWriter<ToggleGeneralSection>,
) {
    for toggle in events.iter() {
        let mut style = general_style.get_single_mut().unwrap();
        if toggle.enabled {
            style.display = Display::Flex;
            control_events.send(ToggleControlsSection { enabled: false });
            general_events.send(ToggleGeneralSection { enabled: false });
        } else {
            style.display = Display::None;
        }
    }
}
pub(crate) fn toggle_controls_menu_section(
    mut events: EventReader<ToggleControlsSection>,
    mut general_style: Query<&mut Style, With<ControlsBGSection>>,
    mut general_events: EventWriter<ToggleGeneralSection>,
    mut graphics_events: EventWriter<ToggleGraphicsSection>,
) {
    for toggle in events.iter() {
        let mut style = general_style.get_single_mut().unwrap();
        if toggle.enabled {
            style.display = Display::Flex;
            general_events.send(ToggleGeneralSection { enabled: false });
            graphics_events.send(ToggleGraphicsSection { enabled: false });
        } else {
            style.display = Display::None;
        }
    }
}
pub(crate) fn exit_button_pressed(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<ExitGameButton>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                info!("Exiting app.");
                exit.send(AppExit);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
pub(crate) fn general_section_button_pressed(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<GeneralHeaderButton>,
        ),
    >,
    mut events: EventWriter<ToggleGeneralSection>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                events.send(ToggleGeneralSection { enabled: true });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
pub(crate) fn graphics_section_button_pressed(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<GraphicsHeaderButton>,
        ),
    >,
    mut events: EventWriter<ToggleGraphicsSection>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                events.send(ToggleGraphicsSection { enabled: true });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
pub(crate) fn controls_section_button_pressed(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<ControlsHeaderButton>,
        ),
    >,
    mut events: EventWriter<ToggleControlsSection>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                events.send(ToggleControlsSection { enabled: true });
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub(crate) fn appply_resolution(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ResolutionInputApply>)>,
    x_input: Query<&TextInputNode, With<ResolutionXInput>>,
    y_input: Query<&TextInputNode, With<ResolutionYInput>>,
    mut events: EventWriter<SetResolution>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                let x = x_input.get_single().unwrap();
                let y = y_input.get_single().unwrap();

                events.send(SetResolution {
                    resolution: (
                        x.input.parse::<u32>().unwrap(),
                        y.input.parse::<u32>().unwrap(),
                    ),
                });
            }
            _ => (),
        }
    }
}

pub(crate) fn apply_window_mode(
    interaction_query: Query<
        (Entity, &Interaction, &Parent),
        (Changed<Interaction>, With<SFButton>),
    >,
    parent_query: Query<&WindowModeHList>,
    mut events: EventWriter<SetWindowMode>,
    hlist_query: Query<&HList>,
) {
    for (entity, interaction, parent) in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                match parent_query.get(**parent) {
                    Ok(_) => {}
                    Err(_) => {
                        continue;
                    }
                }
                match hlist_query.get(**parent) {
                    Ok(hlist) => {
                        let id = hlist
                            .selections_entities
                            .iter()
                            .position(|&r| r == entity)
                            .unwrap() as u8;

                        let mode;

                        match FromPrimitive::from_u8(id) {
                            Some(t) => {
                                mode = t;
                            }
                            None => {
                                warn!("Couldnt convert window mode enum.");
                                continue;
                            }
                        }
                        events.send(SetWindowMode { window_mode: mode });
                    }
                    Err(_) => {
                        warn!("Couildnt find apply window hlist.");
                    }
                }
            }
            _ => (),
        }
    }
}
pub(crate) fn apply_vsync(
    interaction_query: Query<
        (Entity, &Interaction, &Parent),
        (Changed<Interaction>, With<SFButton>),
    >,
    parent_query: Query<&VsyncHList>,
    mut events: EventWriter<SetVsync>,
    hlist_query: Query<&HList>,
) {
    for (entity, interaction, parent) in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                match parent_query.get(**parent) {
                    Ok(_) => {}
                    Err(_) => {
                        continue;
                    }
                }
                match hlist_query.get(**parent) {
                    Ok(hlist) => {
                        let id = hlist
                            .selections_entities
                            .iter()
                            .position(|&r| r == entity)
                            .unwrap() as u8;

                        events.send(SetVsync { enabled: id != 0 });
                    }
                    Err(_) => {
                        warn!("Couildnt find apply window hlist.");
                    }
                }
            }
            _ => (),
        }
    }
}
pub(crate) fn apply_fxaa(
    interaction_query: Query<
        (Entity, &Interaction, &Parent),
        (Changed<Interaction>, With<SFButton>),
    >,
    parent_query: Query<&FxaaHList>,
    mut events: EventWriter<SetFxaa>,
    hlist_query: Query<&HList>,
) {
    for (entity, interaction, parent) in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                match parent_query.get(**parent) {
                    Ok(_) => {}
                    Err(_) => {
                        continue;
                    }
                }
                match hlist_query.get(**parent) {
                    Ok(hlist) => {
                        let id = hlist
                            .selections_entities
                            .iter()
                            .position(|&r| r == entity)
                            .unwrap() as u8;

                        let mode;

                        if id == 0 {
                            mode = None;
                        } else {
                            match FromPrimitive::from_u8(id - 1) {
                                Some(t) => {
                                    mode = Some(t);
                                }
                                None => {
                                    warn!("Couldnt convert window mode enum.");
                                    continue;
                                }
                            }
                        }
                        events.send(SetFxaa { mode: mode });
                    }
                    Err(_) => {
                        warn!("Couildnt find apply window hlist.");
                    }
                }
            }
            _ => (),
        }
    }
}

pub(crate) fn apply_msaa(
    interaction_query: Query<
        (Entity, &Interaction, &Parent),
        (Changed<Interaction>, With<SFButton>),
    >,
    parent_query: Query<&MsaaHList>,
    mut events: EventWriter<SetMsaa>,
    hlist_query: Query<&HList>,
) {
    for (entity, interaction, parent) in interaction_query.iter() {
        match interaction {
            Interaction::Clicked => {
                match parent_query.get(**parent) {
                    Ok(_) => {}
                    Err(_) => {
                        continue;
                    }
                }
                match hlist_query.get(**parent) {
                    Ok(hlist) => {
                        let id = hlist
                            .selections_entities
                            .iter()
                            .position(|&r| r == entity)
                            .unwrap() as u8;

                        let mode;

                        match FromPrimitive::from_u8(id) {
                            Some(t) => {
                                mode = t;
                            }
                            None => {
                                warn!("Couldnt convert window mode enum.");
                                continue;
                            }
                        }
                        events.send(SetMsaa { mode: mode });
                    }
                    Err(_) => {
                        warn!("Couildnt find apply window hlist.");
                    }
                }
            }
            _ => (),
        }
    }
}
