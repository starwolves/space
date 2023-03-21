use bevy::prelude::{not, resource_exists, App, IntoSystemConfig, Plugin, StartupSet};
use console_commands::net::ClientSideConsoleInput;
use resources::is_server::is_server;

use crate::{
    communication::{
        build::{build_communication_ui, console_welcome_message, toggle_console_button},
        chat::{display_chat_message, receive_chat_message, DisplayChatMessage},
        console::{
            console_input, display_console_message, receive_console_message, DisplayConsoleMessage,
        },
        input::{tab_communication_input_toggle, text_input},
    },
    expand::{expand_inventory_hud, ExpandInventoryHud},
    hud::{create_hud, show_hud, ExpandedLeftContentHud},
    input::{
        binds::register_input,
        text_tree_selection::{
            create_text_tree_selection, hide_text_tree_selection, text_tree_select_button,
            text_tree_select_submit_button, TextTreeInputSelectionState, TextTreeSelectionState,
        },
    },
    inventory::{
        actions::{hide_actions, item_actions_button_events, slot_item_actions},
        build::{
            create_inventory_hud, inventory_hud_key_press, open_hud, open_inventory_hud,
            InventoryHudState, OpenHud, OpenInventoryHud,
        },
        items::{
            change_active_item, requeue_hud_add_item_to_slot, right_mouse_click_item,
            slot_item_button_events, HoveringSlotItem, HudAddItemToSlot,
        },
        queue::{
            inventory_net_updates, queue_inventory_updates, InventoryUpdatesQueue,
            RequeueHudAddItemToSlot,
        },
        slots::{scale_slots, update_inventory_hud_slot, HudAddInventorySlot, InventoryHudLabels},
    },
    mouse::{
        focus_state, grab_cursor, grab_mouse_hud_expand, grab_mouse_on_board, release_cursor,
        window_unfocus_event, FocusState, GrabCursor, ReleaseCursor,
    },
    server_stats::{build_server_stats, update_server_stats, ServerStatsState},
    style::button::{button_style_events, changed_focus},
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_event::<ExpandInventoryHud>()
                .add_system(expand_inventory_hud)
                .add_startup_system(create_inventory_hud.in_base_set(StartupSet::PostStartup))
                .add_startup_system(create_hud)
                .add_startup_system(build_server_stats.in_base_set(StartupSet::PostStartup))
                .add_startup_system(build_communication_ui.in_base_set(StartupSet::PostStartup))
                .add_event::<OpenHud>()
                .add_system(inventory_hud_key_press)
                .add_system(open_hud)
                .add_system(
                    queue_inventory_updates.run_if(not(resource_exists::<InventoryHudState>())),
                )
                .add_system(inventory_net_updates)
                .add_system(update_inventory_hud_slot.in_set(InventoryHudLabels::UpdateSlot))
                .add_event::<HudAddItemToSlot>()
                .add_event::<HudAddInventorySlot>()
                .init_resource::<InventoryUpdatesQueue>()
                .add_system(requeue_hud_add_item_to_slot.after(InventoryHudLabels::QueueUpdate))
                .add_event::<RequeueHudAddItemToSlot>()
                .add_system(scale_slots)
                .add_system(slot_item_button_events)
                .add_system(change_active_item)
                .init_resource::<HoveringSlotItem>()
                .add_system(right_mouse_click_item)
                .add_system(slot_item_actions)
                .add_system(show_hud)
                .add_system(hide_actions)
                .add_system(item_actions_button_events)
                .add_system(create_text_tree_selection)
                .add_system(button_style_events)
                .init_resource::<TextTreeInputSelectionState>()
                .add_system(hide_text_tree_selection)
                .add_system(text_tree_select_button)
                .init_resource::<TextTreeSelectionState>()
                .add_system(changed_focus)
                .add_system(text_tree_select_submit_button)
                .add_system(grab_mouse_on_board)
                .add_system(grab_mouse_hud_expand)
                .add_event::<ExpandedLeftContentHud>()
                .add_system(window_unfocus_event)
                .add_event::<GrabCursor>()
                .add_event::<ReleaseCursor>()
                .add_system(release_cursor.after(grab_cursor))
                .add_system(grab_cursor.after(focus_state))
                .add_system(text_input)
                .add_system(receive_chat_message)
                .add_system(tab_communication_input_toggle)
                .add_system(open_inventory_hud.after(open_hud))
                .add_event::<OpenInventoryHud>()
                .add_system(toggle_console_button)
                .add_event::<ClientSideConsoleInput>()
                .add_system(console_input)
                .add_system(receive_console_message)
                .add_startup_system(
                    console_welcome_message
                        .in_base_set(StartupSet::PostStartup)
                        .after(build_communication_ui),
                )
                .add_event::<DisplayConsoleMessage>()
                .add_system(display_console_message)
                .init_resource::<FocusState>()
                .add_system(focus_state)
                .add_event::<DisplayChatMessage>()
                .add_system(display_chat_message)
                .add_system(update_server_stats)
                .init_resource::<ServerStatsState>()
                .add_startup_system(register_input);
        }
    }
}
