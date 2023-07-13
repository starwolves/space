use bevy::prelude::{App, IntoSystemConfigs, Plugin, Update};
use controller::networking::InputUIInput;
use networking::{
    client::is_client_connected,
    messaging::{register_reliable_message, MessageSender},
};
use player::plugin::ConfigurationLabel;
use resources::{is_server::is_server, labels::BuildingLabels};

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
        if is_server() {
            app.add_systems(
                Update,
                (
                    ui_input_boarding,
                    initialize_setupui.in_set(BuildingLabels::TriggerBuild),
                    configure
                        .in_set(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                    new_clients_enable_setupui,
                    setupui_loaded,
                    receive_input_character_name,
                ),
            )
            .add_event::<InputUIInput>()
            .init_resource::<SetupUiState>()
            .init_resource::<SetupUiUserDataSets>();
        } else {
            app.add_systems(Update, client_setup_ui.run_if(is_client_connected));
        }

        register_reliable_message::<SetupUiServerMessage>(app, MessageSender::Server);
        register_reliable_message::<SetupUiClientMessage>(app, MessageSender::Client);
    }
}
