use bevy::prelude::SystemSet;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use bevy_renet::RenetServerPlugin;
use shared::data::{PostUpdateLabels, PreUpdateLabels};
use shared::network::{
    InputChatMessage, PendingMessage, PendingNetworkMessage, ReliableServerMessage,
};

use crate::messages::{
    net_system, ExamineEntityMessages, InputAltItemAttack, InputAttackCell, InputAttackEntity,
    InputBuildGraphics, InputConsoleCommand, InputDropCurrentItem, InputMap,
    InputMapChangeDisplayMode, InputMapRequestDisplayModes, InputMouseAction,
    InputMouseDirectionUpdate, InputMovementInput, InputSceneReady, InputSelectBodyPart,
    InputSprinting, InputSwitchHands, InputTabAction, InputTabDataMap, InputTakeOffItem,
    InputThrowItem, InputToggleAutoMove, InputToggleCombatMode, InputUseWorldItem, InputUserName,
    InputWearItem, NetPlayerConn, TextTreeInputSelection,
};

use super::messages::{incoming_messages, startup_listen_connections};
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetServerPlugin)
            .insert_resource(startup_listen_connections())
            .add_system_to_stage(
                PreUpdate,
                incoming_messages.after(PreUpdateLabels::NetEvents),
            )
            .add_event::<NetPlayerConn>()
            .add_event::<PendingNetworkMessage>()
            .add_event::<InputTabDataMap>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetPlayerConn>),
            )
            .add_event::<InputConsoleCommand>()
            .add_event::<InputMapChangeDisplayMode>()
            .add_event::<InputMapRequestDisplayModes>()
            .add_event::<InputMap>()
            .init_resource::<ExamineEntityMessages>()
            .add_event::<InputChatMessage>()
            .add_event::<InputToggleCombatMode>()
            .add_event::<InputUserName>()
            .add_event::<InputDropCurrentItem>()
            .add_event::<InputSwitchHands>()
            .add_event::<InputUseWorldItem>()
            .add_event::<InputWearItem>()
            .add_event::<InputTakeOffItem>()
            .add_event::<InputThrowItem>()
            .add_event::<InputTabAction>()
            .add_event::<InputBuildGraphics>()
            .add_event::<InputMovementInput>()
            .add_event::<InputSprinting>()
            .add_event::<InputAttackCell>()
            .add_event::<InputToggleAutoMove>()
            .add_event::<TextTreeInputSelection>()
            .add_event::<InputMouseAction>()
            .add_event::<InputAltItemAttack>()
            .add_event::<InputAttackEntity>()
            .add_event::<InputSelectBodyPart>()
            .add_event::<InputMouseDirectionUpdate>()
            .add_event::<InputSceneReady>()
            .add_event::<NetTabData>();
    }
}

pub const RENET_RELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const RENET_BLOCKING_CHANNEL_ID: u8 = 2;

pub struct NetTabData {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetTabData {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub struct NetEvent {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetEvent {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
