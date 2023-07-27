use bevy::prelude::{KeyCode, ResMut};
use resources::binds::{KeyBind, KeyBinds};

pub const TOGGLE_CONSOLE_BIND: &str = "toggleConsole";
pub const SUBMIT_CONSOLE_BIND: &str = "submitConsoleInput";
pub const TOGGLE_CHAT: &str = "toggleChat";
pub const TOGGLE_INVENTORY: &str = "toggleInventoryHud";

pub(crate) fn register_input(mut binds: ResMut<KeyBinds>) {
    binds.list.insert(
        TOGGLE_CONSOLE_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::Grave,
            description: "Toggle the developer console with console commands.".to_string(),
            name: "Toggle Console".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        SUBMIT_CONSOLE_BIND.to_string(),
        KeyBind {
            key_code: KeyCode::Return,
            description: "Submits the given console input.".to_string(),
            name: "Submit Console Input".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        TOGGLE_CHAT.to_string(),
        KeyBind {
            key_code: KeyCode::Tab,
            description: "Toggle the chat to communicate with players.".to_string(),
            name: "Toggle Chat".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        TOGGLE_INVENTORY.to_string(),
        KeyBind {
            key_code: KeyCode::I,
            description: "Toggles the inventory heads up display.".to_string(),
            name: "Toggle Inventory HUD".to_string(),
            customizable: true,
        },
    );
}
