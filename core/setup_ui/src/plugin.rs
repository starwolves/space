use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemLabel, SystemSet};
use networking::server::net_system;
use resources::labels::{PostUpdateLabels, SummoningLabels};

use crate::setup_ui::{
    configure, initialize_setupui, register_ui_input_boarding, ui_input_boarding, InputUIInput,
    NetConfigure, NetOnSetupUI, NetUIInputTransmitData,
};
use bevy::app::CoreStage::PostUpdate;
/// Atmospherics systems ordering label.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ConfigurationLabel {
    SpawnEntity,
    Main,
}

pub struct SetupUiPlugin;

impl Plugin for SetupUiPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(register_ui_input_boarding)
                .add_system(ui_input_boarding)
                .add_system(initialize_setupui.label(SummoningLabels::TriggerSummon))
                .add_event::<NetUIInputTransmitData>()
                .add_event::<InputUIInput>()
                .add_event::<NetOnSetupUI>()
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .after(PostUpdateLabels::VisibleChecker)
                        .label(PostUpdateLabels::Net)
                        .with_system(net_system::<NetUIInputTransmitData>)
                        .with_system(net_system::<NetOnSetupUI>),
                )
                .add_event::<NetConfigure>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                );
        }
    }
}
