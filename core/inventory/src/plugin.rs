use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use console_commands::commands::ConsoleCommandsLabels;
use networking::messaging::{register_reliable_message, MessageSender};
use resources::{is_server::is_server, labels::PostUpdateLabels};

use crate::{
    client::{
        items::{client_item_added_to_slot, set_active_item, ClientBuildInventoryLabel},
        slots::{client_slot_added, AddedSlot},
    },
    net::{InventoryClientMessage, InventoryServerMessage},
    server::{
        entity_update_item::inventory_item_update,
        inventory::{
            add_item_to_slot, add_slot_to_inventory, added_item_to_slot, AddItemToSlot, AddSlot,
            Inventory, InventorySlotLabel, ItemAddedToSlot, SpawnItemLabel,
        },
        set_active_item::process_request_set_active_item,
    },
    spawn_item::spawn_entity_for_client,
};

use bevy::app::CoreStage::PostUpdate;
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(inventory_item_update),
            )
            .add_startup_system(initialize_console_commands.before(ConsoleCommandsLabels::Finalize))
            .add_system(
                add_item_to_slot
                    .before(SpawnItemLabel::SpawnHeldItem)
                    .after(InventorySlotLabel::AddSlotToInventory),
            )
            .add_event::<ItemAddedToSlot>()
            .add_system(added_item_to_slot)
            .add_system(add_slot_to_inventory.label(InventorySlotLabel::AddSlotToInventory))
            .add_system(process_request_set_active_item)
            .add_system(spawn_entity_for_client);
        } else {
            app.add_system(client_item_added_to_slot.after(ClientBuildInventoryLabel::AddSlot))
                .add_system(client_slot_added.label(ClientBuildInventoryLabel::AddSlot))
                .init_resource::<Inventory>()
                .add_event::<AddedSlot>()
                .add_system(set_active_item);
        }
        app.add_event::<AddItemToSlot>().add_event::<AddSlot>();
        register_reliable_message::<InventoryServerMessage>(app, MessageSender::Server);
        register_reliable_message::<InventoryClientMessage>(app, MessageSender::Client);
    }
}
use networking::server::GodotVariant;

use bevy::prelude::ResMut;
use console_commands::commands::AllConsoleCommands;

pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "spawnHeld".to_string(),
        "For server administrators only. Spawn in held entities in hands or in proximity."
            .to_string(),
        vec![
            ("entity_name".to_string(), GodotVariant::String),
            ("player_selector".to_string(), GodotVariant::String),
        ],
    ));
}
