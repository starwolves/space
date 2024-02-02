use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use networking::{
    client::is_client_connected,
    messaging::{register_reliable_message, MessageSender, MessagingSet},
};
use player::{boarding::done_boarding, connections::process_response, plugin::ConfigurationLabel};
use resources::{
    modes::is_server_mode,
    ordering::{BuildingSet, PreUpdate, Update},
};

use crate::{
    core::{
        client_setup_ui, configure, initialize_setupui, new_clients_enable_setupui,
        receive_input_character_name, setupui_loaded, ui_input_boarding, SetupUiState,
        SetupUiUserDataSets,
    },
    net::{SetupUiClientMessage, SetupUiServerMessage},
};
pub struct SetupMenuPlugin;

impl Plugin for SetupMenuPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (
                    initialize_setupui.in_set(BuildingSet::TriggerBuild),
                    configure
                        .in_set(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity)
                        .after(process_response),
                    new_clients_enable_setupui,
                    setupui_loaded,
                    receive_input_character_name,
                ),
            )
            .add_systems(
                PreUpdate,
                (ui_input_boarding
                    .after(MessagingSet::DeserializeIncoming)
                    .before(done_boarding),),
            )
            .init_resource::<SetupUiState>()
            .init_resource::<SetupUiUserDataSets>();
        } else {
            app.add_systems(Update, client_setup_ui.run_if(is_client_connected));
        }

        register_reliable_message::<SetupUiServerMessage>(app, MessageSender::Server, true);
        register_reliable_message::<SetupUiClientMessage>(app, MessageSender::Client, true);
    }
}
