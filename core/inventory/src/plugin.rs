use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};
use console_commands::commands::{ConsoleCommand, ConsoleCommandsSet};
use networking::{
    messaging::{register_reliable_message, MessageSender},
    server::ServerMessageSet,
};
use resources::{
    is_server::is_server,
    sets::{MainSet, PostUpdateSet},
};

use crate::{
    client::{
        items::{
            client_item_added_to_slot, set_active_item, ActiveItemCamera, ClientBuildInventoryLabel,
        },
        slots::{client_slot_added, AddedSlot},
    },
    net::{InventoryClientMessage, InventoryServerMessage},
    server::{
        entity_update_item::inventory_item_update,
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
        if is_server() {
            app.add_systems(
                FixedUpdate,
                inventory_item_update
                    .in_set(PostUpdateSet::EntityUpdate)
                    .in_set(MainSet::PostUpdate),
            )
            .add_systems(
                FixedUpdate,
                (
                    process_request_set_active_item,
                    spawn_entity_for_client,
                    add_slot_to_inventory
                        .in_set(InventorySlotLabel::AddSlotToInventory)
                        .before(ServerMessageSet::Send),
                    add_item_to_slot.after(InventorySlotLabel::AddSlotToInventory),
                    added_item_to_slot.after(add_item_to_slot),
                )
                    .in_set(MainSet::Update),
            )
            .add_event::<ItemAddedToSlot>();
        } else {
            app.add_systems(
                FixedUpdate,
                (
                    client_item_added_to_slot.after(ClientBuildInventoryLabel::AddSlot),
                    set_active_item,
                    client_slot_added.in_set(ClientBuildInventoryLabel::AddSlot),
                )
                    .in_set(MainSet::Update),
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
        register_reliable_message::<InventoryServerMessage>(app, MessageSender::Server);
        register_reliable_message::<InventoryClientMessage>(app, MessageSender::Client);
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
