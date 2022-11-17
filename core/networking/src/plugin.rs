use std::env;

use bevy::prelude::SystemSet;
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};
use bevy_renet::renet::NETCODE_KEY_BYTES;
use bevy_renet::RenetServerPlugin;
use networking_macros::NetMessage;
use resources::labels::{PostUpdateLabels, PreUpdateLabels};

use super::server::{incoming_messages, startup_server_listen_connections};
use crate::client::{connect_to_server, ConnectToServer, Connection, ConnectionPreferences};
use crate::server::PendingMessage;
use crate::server::{
    net_system, InputAction, InputAltItemAttack, InputAttackCell, InputAttackEntity,
    InputBuildGraphics, InputChatMessage, InputConsoleCommand, InputDropCurrentItem,
    InputExamineEntity, InputExamineMap, InputListActionsMap, InputMap, InputMapChangeDisplayMode,
    InputMapRequestOverlay, InputMouseAction, InputMouseDirectionUpdate, InputMovementInput,
    InputSceneReady, InputSelectBodyPart, InputSprinting, InputSwitchHands, InputTakeOffItem,
    InputThrowItem, InputToggleAutoMove, InputToggleCombatMode, InputUIInput,
    InputUIInputTransmitText, InputUseWorldItem, InputUserName, InputWearItem, NetHealth,
    NetLoadEntity, NetPlayerConn, NetSendEntityUpdates, NetUnloadEntity, PendingNetworkMessage,
    ReliableServerMessage, TextTreeInputSelection,
};
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;
pub struct NetworkingPlugin;

#[cfg(any(feature = "server", feature = "client"))]
pub(crate) const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"lFNpVdFi5LhL8xlDFtnobx5onFR30afX";

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_plugin(RenetServerPlugin);
            app.insert_resource(startup_server_listen_connections(*PRIVATE_KEY))
                .add_system_to_stage(
                    PreUpdate,
                    incoming_messages
                        .after(PreUpdateLabels::NetEvents)
                        .label(PreUpdateLabels::ProcessInput),
                )
                .add_event::<NetPlayerConn>()
                .add_event::<PendingNetworkMessage>()
                .add_event::<InputListActionsMap>()
                .add_event::<InputConsoleCommand>()
                .add_event::<InputMapChangeDisplayMode>()
                .add_event::<InputMapRequestOverlay>()
                .add_event::<InputMap>()
                .add_event::<InputChatMessage>()
                .add_event::<InputToggleCombatMode>()
                .add_event::<InputUserName>()
                .add_event::<InputDropCurrentItem>()
                .add_event::<InputSwitchHands>()
                .add_event::<InputUseWorldItem>()
                .add_event::<InputWearItem>()
                .add_event::<InputTakeOffItem>()
                .add_event::<InputThrowItem>()
                .add_event::<InputAction>()
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
                .add_event::<InputUIInputTransmitText>()
                .add_event::<InputUIInput>()
                .add_event::<InputExamineEntity>()
                .add_event::<InputExamineMap>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetHealth>)
                        .with_system(net_system::<NetPlayerConn>)
                        .with_system(net_system::<NetLoadEntity>)
                        .with_system(net_system::<NetUnloadEntity>)
                        .with_system(net_system::<NetSendEntityUpdates>),
                )
                .add_event::<NetHealth>()
                .add_event::<NetSendEntityUpdates>()
                .add_event::<NetUnloadEntity>()
                .add_event::<NetLoadEntity>();
        } else {
            app.add_system(connect_to_server)
                .add_event::<ConnectToServer>()
                .init_resource::<ConnectionPreferences>()
                .init_resource::<Connection>();
        }
    }
}

pub const RENET_RELIABLE_CHANNEL_ID: u8 = 0;
pub const RENET_UNRELIABLE_CHANNEL_ID: u8 = 1;
pub const RENET_BLOCKING_CHANNEL_ID: u8 = 2;
#[derive(NetMessage)]
pub struct NetEvent {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
