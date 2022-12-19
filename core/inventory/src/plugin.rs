use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use console_commands::commands::ConsoleCommandsLabels;
use networking::messaging::{init_reliable_message, MessageSender};
use resources::{
    is_server::is_server,
    labels::{ActionsLabels, PostUpdateLabels, UpdateLabels},
};

use crate::{
    actions::pickup_prerequisite_check,
    actions_item::build_actions,
    entity_update_item::inventory_item_update,
    item_events::{
        pickup_world_item_action, InputDropCurrentItem, InputTakeOffItem, InputThrowItem,
        InputUseWorldItem, InputWearItem, ThrownItem,
    },
    networking::{incoming_messages, InventoryClientMessage, InventoryServerMessage},
    switch_hands::InputSwitchHands,
};

use super::{
    entity_update::inventory_update,
    item_events::{drop_current_item, pickup_world_item, take_off_item, throw_item, wear_item},
    switch_hands::switch_hands,
};
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(pickup_world_item)
                .add_system(switch_hands)
                .add_system(wear_item)
                .add_system(take_off_item)
                .add_system(throw_item)
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(inventory_update),
                )
                .add_system(drop_current_item.label(UpdateLabels::DropCurrentItem))
                .add_system(
                    pickup_prerequisite_check
                        .label(ActionsLabels::Approve)
                        .after(ActionsLabels::Init),
                )
                .add_system(
                    pickup_world_item_action
                        .label(ActionsLabels::Action)
                        .after(ActionsLabels::Approve),
                )
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<InputDropCurrentItem>()
                .add_event::<InputThrowItem>()
                .add_event::<InputSwitchHands>()
                .add_event::<InputTakeOffItem>()
                .add_event::<InputUseWorldItem>()
                .add_event::<InputWearItem>()
                .add_event::<ThrownItem>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(inventory_item_update),
                )
                .add_startup_system(
                    initialize_console_commands.before(ConsoleCommandsLabels::Finalize),
                )
                .add_system(
                    build_actions
                        .label(ActionsLabels::Build)
                        .after(ActionsLabels::Init),
                );
        }

        init_reliable_message::<InventoryServerMessage>(app, MessageSender::Server);
        init_reliable_message::<InventoryClientMessage>(app, MessageSender::Client);
    }
}
use networking::server::GodotVariant;

use bevy::prelude::ResMut;
use console_commands::commands::AllConsoleCommands;
#[cfg(feature = "server")]
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
