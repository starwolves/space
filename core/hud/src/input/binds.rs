use bevy::prelude::{KeyCode, MouseButton, ResMut};
use resources::input::{KeyBind, KeyBinds, KeyCodeEnum};

pub const TOGGLE_CONSOLE_BIND: &str = "toggleConsole";
pub const SUBMIT_CONSOLE_BIND: &str = "submitConsoleInput";
pub const TOGGLE_CHAT: &str = "toggleChat";
pub const TOGGLE_INVENTORY: &str = "toggleInventoryHud";

pub(crate) fn register_input(mut binds: ResMut<KeyBinds>) {
    binds.list.insert(
        TOGGLE_CONSOLE_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::Backquote),
            description: "Toggle the developer console with console commands.".to_string(),
            name: "Toggle Console".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        SUBMIT_CONSOLE_BIND.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::Backspace),
            description: "Submits the given console input.".to_string(),
            name: "Submit Console Input".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        TOGGLE_CHAT.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::Tab),
            description: "Toggle the chat to communicate with players.".to_string(),
            name: "Toggle Chat".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        TOGGLE_INVENTORY.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Keyboard(KeyCode::KeyI),
            description: "Toggles the inventory heads up display.".to_string(),
            name: "Toggle Inventory HUD".to_string(),
            customizable: true,
        },
    );
    binds.list.insert(
        SHOW_TAB_ACTIONS.to_string(),
        KeyBind {
            key_code: KeyCodeEnum::Mouse(MouseButton::Right),
            description: "Use button on an inventory item to get available actions.".to_string(),
            name: "Get item actions.".to_string(),
            customizable: false,
        },
    );
}
pub const SHOW_TAB_ACTIONS: &str = "showTabActions";
