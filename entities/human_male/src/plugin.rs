use basic_console_commands::register::register_basic_console_commands_for_type;
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use construction_tool::spawn::ConstructionToolType;
use entity::{
    base_mesh::link_base_mesh, entity_types::register_entity_type, loading::load_entity,
    spawn::SpawnItemSet,
};

use networking::{client::detect_client_world_loaded, stamp::step_tickrate_stamp};
use physics::{spawn::build_rigid_bodies, sync::SpawningSimulation};
use player::boarding::player_boarded;
use resources::{
    correction::CorrectionSet,
    modes::{is_correction_mode, is_server_mode},
    sets::{BuildingSet, CombatSet, MainSet},
};

use crate::{
    boarding::spawn_boarding_player,
    hands_attack_handler::hands_attack_handler,
    setup_ui_showcase::human_male_setup_ui,
    spawn::{
        attach_human_male_camera, build_base_human_males, build_human_males,
        process_add_item_slot_buffer, process_add_slot_buffer, simulation_humanoid_spawn,
        spawn_held_item, AddItemToSlotBuffer, AddSlotBuffer, HumanMaleType,
    },
};
pub struct HumanMalePlugin;

impl Plugin for HumanMalePlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                FixedUpdate,
                (
                    hands_attack_handler
                        .in_set(CombatSet::WeaponHandler)
                        .after(CombatSet::CacheAttack),
                    human_male_setup_ui.in_set(BuildingSet::TriggerBuild),
                )
                    .in_set(MainSet::Update),
            )
            .add_systems(
                FixedUpdate,
                spawn_boarding_player
                    .before(player_boarded)
                    .in_set(BuildingSet::TriggerBuild)
                    .in_set(MainSet::Update),
            );
        } else {
            app.add_systems(
                FixedUpdate,
                (
                    link_base_mesh::<HumanMaleType>.after(SpawnItemSet::SpawnHeldItem),
                    load_entity::<HumanMaleType>
                        .before(SpawnItemSet::SpawnHeldItem)
                        .in_set(BuildingSet::TriggerBuild)
                        .in_set(CorrectionSet::Start),
                    attach_human_male_camera
                        .after(BuildingSet::TriggerBuild)
                        .after(detect_client_world_loaded),
                )
                    .in_set(MainSet::Update),
            );
        }
        if is_correction_mode(app) {
            app.add_systems(
                FixedUpdate,
                simulation_humanoid_spawn
                    .in_set(SpawningSimulation::Spawn)
                    .in_set(MainSet::Update),
            );
        }
        register_entity_type::<HumanMaleType>(app);
        register_basic_console_commands_for_type::<HumanMaleType>(app);
        app.add_systems(
            FixedUpdate,
            (
                build_human_males
                    .after(SpawnItemSet::SpawnHeldItem)
                    .in_set(BuildingSet::NormalBuild),
                (build_base_human_males::<HumanMaleType>).after(SpawnItemSet::SpawnHeldItem),
                (build_rigid_bodies::<HumanMaleType>).after(SpawnItemSet::SpawnHeldItem),
                spawn_held_item::<ConstructionToolType>
                    .in_set(SpawnItemSet::SpawnHeldItem)
                    .after(BuildingSet::TriggerBuild),
            )
                .in_set(MainSet::Update),
        )
        .add_systems(
            FixedUpdate,
            (
                process_add_item_slot_buffer.after(step_tickrate_stamp),
                process_add_slot_buffer,
            )
                .in_set(MainSet::PreUpdate),
        )
        .init_resource::<AddItemToSlotBuffer>()
        .init_resource::<AddSlotBuffer>();
    }
}
