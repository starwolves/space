use bevy::prelude::{
    not, resource_exists, App, FixedUpdate, IntoSystemConfigs, Plugin, PostStartup, Startup, Update,
};
use bevy_renet::renet::RenetClient;
use console_commands::net::ClientSideConsoleInput;
use inventory::client::items::ClientBuildInventoryLabel;
use resources::{modes::is_server_mode, sets::MainSet};
use ui::{
    cursor::CursorSet,
    text_input::{FocusTextSet, TextInputLabel},
};

use crate::{
    build::{create_hud, show_hud, ExpandedLeftContentHud},
    communication::{
        build::{build_communication_ui, console_welcome_message, toggle_console_button},
        chat::{display_chat_message, receive_chat_message, DisplayChatMessage},
        console::{
            console_input, display_console_message, receive_console_message, DisplayConsoleMessage,
        },
        input::{
            communication_input_toggle, tab_communication_input_toggle, text_input,
            CommunicationToggleSet, ConsoleCommandsClientSet, ToggleCommunication,
        },
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
            InventoryHudState, OpenHud, OpenHudSet, OpenInventoryHud,
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
    mouse::{grab_mouse_hud_expand, mouse_press_hud_unfocus, window_unfocus_event},
    server_stats::{build_server_stats, update_server_stats, ServerStatsState},
    style::button::{button_style_events, changed_focus},
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        if !is_server_mode(app) {
            app.add_event::<ToggleCommunication>()
                .add_event::<ExpandInventoryHud>()
                .add_systems(
                    FixedUpdate,
                    (
                        (
                            expand_inventory_hud.after(OpenHudSet::ExpandInventory),
                            inventory_hud_key_press,
                            inventory_net_updates
                                .in_set(ClientBuildInventoryLabel::Net)
                                .after(ClientBuildInventoryLabel::AddSlot),
                            update_inventory_hud_slot
                                .after(ClientBuildInventoryLabel::Net)
                                .in_set(InventoryHudSet::UpdateSlot),
                            requeue_hud_add_item_to_slot.after(InventoryHudSet::QueueUpdate),
                            slot_item_button_events,
                            scale_slots,
                            change_active_item,
                            right_mouse_click_item,
                            slot_item_actions.in_set(OpenHudSet::ExpandInventory),
                            show_hud,
                            item_actions_button_events,
                            create_text_tree_selection,
                            button_style_events,
                            hide_text_tree_selection.after(OpenHudSet::ExpandInventory),
                            text_tree_select_button,
                            changed_focus,
                            text_tree_select_submit_button,
                        )
                            .in_set(MainSet::Update),
                        (
                            text_input.in_set(ConsoleCommandsClientSet::Submit),
                            receive_chat_message,
                            open_inventory_hud
                                .after(inventory_hud_key_press)
                                .after(OpenHudSet::Process)
                                .in_set(OpenHudSet::ExpandInventory),
                            toggle_console_button
                                .before(OpenHudSet::Process)
                                .before(TextInputLabel::MousePressUnfocus)
                                .in_set(FocusTextSet::Focus),
                            console_input
                                .after(ConsoleCommandsClientSet::Submit)
                                .before(ConsoleCommandsClientSet::Display),
                            receive_console_message.before(ConsoleCommandsClientSet::Display),
                            display_console_message.in_set(ConsoleCommandsClientSet::Display),
                            display_chat_message.after(receive_chat_message),
                            queue_inventory_updates
                                .after(ClientBuildInventoryLabel::AddSlot)
                                .run_if(not(resource_exists::<InventoryHudState>())),
                            console_welcome_message.before(ConsoleCommandsClientSet::Display),
                            update_server_stats.run_if(resource_exists::<RenetClient>()),
                            window_unfocus_event
                                .before(TextInputLabel::MousePressUnfocus)
                                .before(CursorSet::Perform),
                        )
                            .in_set(MainSet::Update),
                    ),
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
                .add_systems(
                    Update,
                    (
                        mouse_press_hud_unfocus
                            .before(TextInputLabel::MousePressUnfocus)
                            .before(CursorSet::Perform)
                            .in_set(CommunicationToggleSet::Toggle)
                            .in_set(FocusTextSet::Unfocus),
                        open_hud.in_set(OpenHudSet::Process),
                        hide_actions
                            .in_set(OpenHudSet::Process)
                            .in_set(MainSet::Update),
                        grab_mouse_hud_expand
                            .in_set(OpenHudSet::Process)
                            .before(CursorSet::Perform),
                        communication_input_toggle
                            .before(TextInputLabel::MousePressUnfocus)
                            .before(OpenHudSet::Process)
                            .in_set(CommunicationToggleSet::Toggle)
                            .in_set(FocusTextSet::Focus),
                        tab_communication_input_toggle.before(CommunicationToggleSet::Toggle),
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
                .add_event::<OpenInventoryHud>()
                .add_event::<ClientSideConsoleInput>()
                .add_event::<DisplayConsoleMessage>()
                .add_event::<DisplayChatMessage>()
                .init_resource::<ServerStatsState>();
        }
    }
}
