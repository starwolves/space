use basic_console_commands::register::register_basic_console_commands_for_type;
use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut};
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    register::register_entity_type,
};
use humanoid::humanoid::HUMAN_MALE_ENTITY_NAME;

use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels, StartupLabels},
};

use crate::{
    boarding::spawn_boarding_player,
    hands_attack_handler::hands_attack_handler,
    setup_ui_showcase::human_male_setup_ui,
    spawn::{build_base_human_males, build_human_males, HumanMaleType},
};
use bevy::app::CoreStage::PostUpdate;
pub struct HumanMalePlugin;

impl Plugin for HumanMalePlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
                hands_attack_handler
                    .label(CombatLabels::WeaponHandler)
                    .after(CombatLabels::CacheAttack),
            )
            .add_system(human_male_setup_ui.label(BuildingLabels::TriggerBuild))
            .add_system_to_stage(PostUpdate, spawn_boarding_player);
        }
        register_entity_type::<HumanMaleType>(app);
        register_basic_console_commands_for_type::<HumanMaleType>(app);
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(
                build_human_males
                    .before(BuildingLabels::TriggerBuild)
                    .label(BuildingLabels::NormalBuild),
            )
            .add_system(
                (build_base_human_males::<HumanMaleType>).after(BuildingLabels::TriggerBuild),
            )
            .add_system((build_rigid_bodies::<HumanMaleType>).after(BuildingLabels::TriggerBuild));
    }
}

#[cfg(feature = "server")]
pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: HUMAN_MALE_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
