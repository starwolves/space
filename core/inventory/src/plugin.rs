use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup};
use console_commands::commands::{ConsoleCommand, ConsoleCommandsSet};
use networking::{
    messaging::{register_reliable_message, MessageSender},
    server::{EntityUpdatesSet, ServerMessageSet},
};
use resources::{
    modes::is_server_mode,
    ordering::{PostUpdateSet, Update},
};

use crate::{
    client::{
        items::{
            client_item_added_to_slot, set_active_item, ActiveItemCamera, ClientActiveCameraItem,
            ClientBuildInventoryLabel,
        },
        slots::{client_slot_added, AddedSlot},
    },
    net::{InventoryClientMessage, InventoryServerMessage},
    server::{
        inventory::{
            add_item_to_slot, add_slot_to_inventory, added_item_to_slot, AddItemToSlot, AddSlot,
            Inventory, InventorySlotLabel, ItemAddedToSlot,
        },
        set_active_item::process_request_set_active_item,
    },
    spawn_item::spawn_entity_for_client,
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (spawn_entity_for_client
                    .after(PostUpdateSet::VisibleChecker)
                    .before(ServerMessageSet::Send),)
                    .in_set(EntityUpdatesSet::Ready),
            )
            .add_systems(
                Update,
                (
                    process_request_set_active_item,
                    add_slot_to_inventory
                        .in_set(InventorySlotLabel::AddSlotToInventory)
                        .before(ServerMessageSet::Send),
                    add_item_to_slot.after(InventorySlotLabel::AddSlotToInventory),
                    added_item_to_slot.after(add_item_to_slot),
                ),
            )
            .add_event::<ItemAddedToSlot>();
        } else {
            app.add_systems(
                Update,
                (
                    client_item_added_to_slot.after(ClientBuildInventoryLabel::AddSlot),
                    set_active_item.in_set(ClientActiveCameraItem::ActivateItem),
                    client_slot_added.in_set(ClientBuildInventoryLabel::AddSlot),
                ),
            )
            .init_resource::<Inventory>()
            .add_event::<AddedSlot>()
            .add_event::<ActiveItemCamera>();
        }
        app.add_event::<AddItemToSlot>()
            .add_event::<AddSlot>()
            .add_systems(
                Startup,
                initialize_console_commands.before(ConsoleCommandsSet::Finalize),
            );
        register_reliable_message::<InventoryServerMessage>(app, MessageSender::Server, true);
        register_reliable_message::<InventoryClientMessage>(app, MessageSender::Client, true);
    }
}
use networking::server::ConsoleArgVariant;

use bevy::prelude::ResMut;
use console_commands::commands::AllConsoleCommands;

pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push(ConsoleCommand {
        base: "spawnHeld".to_string(),
        description: "Spawn in held entities in hands or in proximity.".to_string(),
        args: vec![
            ("entity_name".to_string(), ConsoleArgVariant::String),
            ("player_selector".to_string(), ConsoleArgVariant::String),
        ],
    });
}
