use bevy::log::info;
use bevy::log::warn;
use bevy::{
    app::AppExit,
    prelude::{
        Button, Changed, Entity, Event, EventReader, EventWriter, KeyCode, Parent, Query, Res,
        ResMut, With,
    },
    ui::{Display, Interaction, Style},
};

use graphics::settings::SetRCAS;
use graphics::settings::SetSyncCorrection;
use graphics::settings::{SetFxaa, SetMsaa, SetResolution, SetVsync, SetWindowMode};
use hud::inventory::build::{InventoryHudState, OpenHud};
use num_traits::FromPrimitive;
use resources::{
    hud::{EscapeMenuState, HudState},
    input::{InputBuffer, KeyBind, KeyBinds, KeyCodeEnum},
    ui::MainMenuState,
};
use ui::{button::SFButton, hlist::HList, text_input::TextInputNode};

use crate::build::RCASHList;
use crate::build::SyncCorrectionHList;
use crate::build::{
    ControlsBGSection, ControlsHeaderButton, ExitGameButton, FxaaHList, GeneralHeaderButton,
    GeneralSection, MsaaHList, PerformanceBGSection, PerformanceHeaderButton, ResolutionInputApply,
    ResolutionXInput, ResolutionYInput, VsyncHList, WindowModeHList,
};
#[derive(Event)]
pub struct ToggleEscapeMenu {
    pub enabled: bool,
}

pub const ESC_MENU_BIND: &str = "escMenu";

pub(crate) fn register_input(mut keys: ResMut<KeyBinds>) {
    keys.list.insert(
        ESC_MENU_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::Escape),
            description: "Toggles the escape menu.".to_string(),
            name: "Escape Menu".to_string(),
            customizable: true,
        },
    );
}

pub(crate) fn esc_button_menu(
    mut events: EventWriter<ToggleEscapeMenu>,
    state: Res<EscapeMenuState>,
    keys2: Res<InputBuffer>,
    mn_state: Res<MainMenuState>,
) {
    if keys2.just_pressed(ESC_MENU_BIND) {
        if mn_state.enabled {
            return;
        }
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
    inventory_state: Res<InventoryHudState>,
) {
    for toggle in events.read() {
        state.visible = toggle.enabled;

        let visible_state;
        if !state.visible {
            visible_state = inventory_state.open;
        } else {
            visible_state = state.visible;
        }

        hud.send(OpenHud {
            open: visible_state,
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
#[derive(Event)]
pub struct ToggleGeneralSection {
    pub enabled: bool,
}
#[derive(Event)]
pub struct ToggleGraphicsSection {
    pub enabled: bool,
}
#[derive(Event)]
pub struct ToggleControlsSection {
    pub enabled: bool,
}

pub(crate) fn toggle_general_menu_section(
    mut events: EventReader<ToggleGeneralSection>,
    mut general_style: Query<&mut Style, With<GeneralSection>>,
    mut control_events: EventWriter<ToggleControlsSection>,
    mut graphics_events: EventWriter<ToggleGraphicsSection>,
) {
    for toggle in events.read() {
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
    mut general_style: Query<&mut Style, With<PerformanceBGSection>>,
    mut control_events: EventWriter<ToggleControlsSection>,
    mut general_events: EventWriter<ToggleGeneralSection>,
) {
    for toggle in events.read() {
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
    for toggle in events.read() {
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
            Interaction::Pressed => {
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
            Interaction::Pressed => {
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
            With<PerformanceHeaderButton>,
        ),
    >,
    mut events: EventWriter<ToggleGraphicsSection>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
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
            Interaction::Pressed => {
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
            Interaction::Pressed => {
                let x = x_input.get_single().unwrap();
                let y = y_input.get_single().unwrap();
                let xz;
                if !x.input.is_empty() {
                    match x.input.parse::<u32>() {
                        Ok(s) => {
                            xz = s;
                        }
                        Err(_) => {
                            warn!("Failed to parse resx: {}", x.input);
                            continue;
                        }
                    }
                } else {
                    match x.placeholder_text_option.clone().unwrap().parse::<u32>() {
                        Ok(s) => {
                            xz = s;
                        }
                        Err(_) => {
                            warn!("Failed to parse resx2: {}", x.input);
                            continue;
                        }
                    }
                }
                let yz;
                if !y.input.is_empty() {
                    match y.input.parse::<u32>() {
                        Ok(s) => {
                            yz = s;
                        }
                        Err(_) => {
                            warn!("Failed to parse resy: {}", x.input);
                            continue;
                        }
                    }
                } else {
                    match y.placeholder_text_option.clone().unwrap().parse::<u32>() {
                        Ok(s) => {
                            yz = s;
                        }
                        Err(_) => {
                            warn!("Failed to parse resy2: {}", x.input);
                            continue;
                        }
                    }
                }
                events.send(SetResolution {
                    resolution: (xz, yz),
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
            Interaction::Pressed => {
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

pub(crate) fn apply_rcas(
    interaction_query: Query<
        (Entity, &Interaction, &Parent),
        (Changed<Interaction>, With<SFButton>),
    >,
    parent_query: Query<&RCASHList>,
    mut events: EventWriter<SetRCAS>,
    hlist_query: Query<&HList>,
) {
    for (entity, interaction, parent) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
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

                        events.send(SetRCAS { enabled: id != 0 });
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

pub(crate) fn apply_syncronous_correction_setting(
    interaction_query: Query<
        (Entity, &Interaction, &Parent),
        (Changed<Interaction>, With<SFButton>),
    >,
    parent_query: Query<&SyncCorrectionHList>,
    mut events: EventWriter<SetSyncCorrection>,
    hlist_query: Query<&HList>,
) {
    for (entity, interaction, parent) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
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

                        events.send(SetSyncCorrection { enabled: id != 0 });
                    }
                    Err(_) => {
                        warn!("Couldnt find apply window hlist.");
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
            Interaction::Pressed => {
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
            Interaction::Pressed => {
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
            Interaction::Pressed => {
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
