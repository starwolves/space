use bevy::prelude::{
    not, resource_exists, App, FixedUpdate, IntoSystemConfigs, Plugin, PostStartup, Startup, Update,
};
use console_commands::net::ClientSideConsoleInput;
use resources::{is_server::is_server, sets::MainSet};
use ui::text_input::TextInputLabel;

use crate::{
    build::{create_hud, show_hud, ExpandedLeftContentHud},
    communication::{
        build::{build_communication_ui, console_welcome_message, toggle_console_button},
        chat::{display_chat_message, receive_chat_message, DisplayChatMessage},
        console::{
            console_input, display_console_message, receive_console_message, DisplayConsoleMessage,
        },
        input::{tab_communication_input_toggle, text_input},
    },
    expand::{expand_inventory_hud, ExpandInventoryHud},
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
        slots::{scale_slots, update_inventory_hud_slot, HudAddInventorySlot, InventoryHudSet},
    },
    mouse::{
        clear_window_focus_buffer, focus_state, grab_cursor, grab_mouse_hud_expand,
        grab_mouse_on_board, release_cursor, window_focus_buffer, window_unfocus_event, FocusState,
        GrabCursor, ReleaseCursor, WindowFocusBuffer,
    },
    server_stats::{build_server_stats, update_server_stats, ServerStatsState},
    style::button::{button_style_events, changed_focus},
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_event::<ExpandInventoryHud>()
                .init_resource::<WindowFocusBuffer>()
                .add_systems(
                    FixedUpdate,
                    (
                        (
                            open_hud.after(tab_communication_input_toggle),
                            expand_inventory_hud,
                            inventory_hud_key_press,
                            inventory_net_updates,
                            update_inventory_hud_slot.in_set(InventoryHudSet::UpdateSlot),
                            requeue_hud_add_item_to_slot.after(InventoryHudSet::QueueUpdate),
                            slot_item_button_events,
                            scale_slots,
                            change_active_item,
                            right_mouse_click_item,
                            slot_item_actions,
                            show_hud,
                            hide_actions,
                            item_actions_button_events,
                            create_text_tree_selection,
                            button_style_events,
                            hide_text_tree_selection,
                            text_tree_select_button,
                            changed_focus,
                            text_tree_select_submit_button,
                        )
                            .in_set(MainSet::Update),
                        (
                            grab_mouse_on_board,
                            grab_mouse_hud_expand
                                .before(grab_cursor)
                                .before(release_cursor),
                            window_unfocus_event,
                            release_cursor.after(grab_cursor),
                            grab_cursor.after(focus_state),
                            text_input,
                            receive_chat_message,
                            tab_communication_input_toggle
                                .before(TextInputLabel::MousePressUnfocus),
                            open_inventory_hud.after(open_hud),
                            toggle_console_button.before(TextInputLabel::MousePressUnfocus),
                            console_input,
                            receive_console_message,
                            display_console_message,
                            focus_state,
                            display_chat_message,
                            update_server_stats,
                            queue_inventory_updates
                                .run_if(not(resource_exists::<InventoryHudState>())),
                            console_welcome_message,
                        )
                            .in_set(MainSet::Update),
                    ),
                )
                .add_systems(Update, window_focus_buffer)
                .add_systems(
                    FixedUpdate,
                    clear_window_focus_buffer.in_set(MainSet::PostUpdate),
                )
                .add_systems(Startup, (create_hud, register_input))
                .add_systems(
                    PostStartup,
                    (
                        create_inventory_hud,
                        build_server_stats,
                        build_communication_ui,
                    ),
                )
                .add_event::<OpenHud>()
                .add_event::<HudAddItemToSlot>()
                .add_event::<HudAddInventorySlot>()
                .init_resource::<InventoryUpdatesQueue>()
                .add_event::<RequeueHudAddItemToSlot>()
                .init_resource::<HoveringSlotItem>()
                .init_resource::<TextTreeInputSelectionState>()
                .init_resource::<TextTreeSelectionState>()
                .add_event::<ExpandedLeftContentHud>()
                .add_event::<GrabCursor>()
                .add_event::<ReleaseCursor>()
                .add_event::<OpenInventoryHud>()
                .add_event::<ClientSideConsoleInput>()
                .add_event::<DisplayConsoleMessage>()
                .init_resource::<FocusState>()
                .add_event::<DisplayChatMessage>()
                .init_resource::<ServerStatsState>();
        }
    }
}
