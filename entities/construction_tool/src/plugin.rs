use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{resource_exists, App, IntoSystemConfigs, Plugin, Update};
use combat::melee_queries::melee_attack_handler;
use combat::sfx::{attack_sfx, health_combat_hit_result_sfx};
use entity::base_mesh::link_base_mesh;
use entity::entity_types::register_entity_type;
use entity::loading::load_entity;
use entity::spawn::build_base_entities;

use gridmap::construction::{GridmapConstructionState, ShowYLevelPlane};
use hud::inventory::items::update_inventory_hud_add_item_to_slot;
use hud::inventory::slots::InventoryHudLabels;
use inventory::client::items::active_item_display_camera;
use inventory::server::inventory::SpawnItemLabel;
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_bodies;
use resources::is_server::is_server;
use resources::labels::{ActionsLabels, BuildingLabels, CombatLabels, UpdateLabels};

use crate::action::{
    build_actions, construct_action_prequisite_check, construction_tool_actions,
    construction_tool_inventory_prequisite_check, construction_tool_select_construction_option,
    deconstruct_action_prequisite_check, send_constructable_items,
};
use crate::construction_tool::ConstructionTool;
use crate::map_construction::{
    construction_tool_enable_select_cell_in_front_camera, mouse_click_input,
};

use super::{
    construction_tool::{InputConstruct, InputConstructionOptions, InputDeconstruct},
    spawn::{build_construction_tools, ConstructionToolType},
};

pub struct ConstructionToolAdminPlugin;

impl Plugin for ConstructionToolAdminPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<InputConstruct>()
                .add_event::<InputDeconstruct>()
                .add_event::<InputConstructionOptions>()
                .add_systems(
                    Update,
                    (
                        melee_attack_handler::<ConstructionTool>
                            .in_set(CombatLabels::WeaponHandler)
                            .after(CombatLabels::CacheAttack),
                        attack_sfx::<ConstructionTool>
                            .after(CombatLabels::WeaponHandler)
                            .after(CombatLabels::Query),
                        health_combat_hit_result_sfx::<ConstructionTool>
                            .after(CombatLabels::FinalizeApplyDamage),
                        construction_tool_inventory_prequisite_check
                            .in_set(ActionsLabels::Approve)
                            .after(ActionsLabels::Build),
                        deconstruct_action_prequisite_check
                            .in_set(ActionsLabels::Approve)
                            .after(ActionsLabels::Build),
                        construct_action_prequisite_check
                            .in_set(ActionsLabels::Approve)
                            .after(ActionsLabels::Build),
                        construction_tool_actions
                            .in_set(ActionsLabels::Action)
                            .after(ActionsLabels::Approve),
                        build_actions
                            .in_set(ActionsLabels::Build)
                            .after(ActionsLabels::Init),
                        construction_tool_select_construction_option
                            .in_set(UpdateLabels::TextTreeInputSelection),
                        send_constructable_items,
                        mouse_click_input,
                    ),
                );
        } else {
            app.add_systems(
                Update,
                (
                    update_inventory_hud_add_item_to_slot::<ConstructionToolType>
                        .after(InventoryHudLabels::UpdateSlot)
                        .in_set(InventoryHudLabels::QueueUpdate),
                    load_entity::<ConstructionToolType>,
                    link_base_mesh::<ConstructionToolType>,
                    active_item_display_camera::<ConstructionToolType>,
                    construction_tool_enable_select_cell_in_front_camera
                        .run_if(resource_exists::<GridmapConstructionState>()),
                ),
            );
        }
        register_entity_type::<ConstructionToolType>(app);
        register_basic_console_commands_for_type::<ConstructionToolType>(app);
        register_basic_console_commands_for_inventory_item_type::<ConstructionToolType>(app);
        app.add_systems(
            Update,
            (
                build_construction_tools::<ConstructionToolType>
                    .after(BuildingLabels::TriggerBuild),
                (build_rigid_bodies::<ConstructionToolType>).after(BuildingLabels::TriggerBuild),
                (build_base_entities::<ConstructionToolType>).after(BuildingLabels::TriggerBuild),
                (build_inventory_items::<ConstructionToolType>)
                    .after(BuildingLabels::TriggerBuild)
                    .after(SpawnItemLabel::SpawnHeldItem)
                    .in_set(SpawnItemLabel::AddingComponent),
            ),
        )
        .add_event::<ShowYLevelPlane>();
    }
}
