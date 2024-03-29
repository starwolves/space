use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{resource_exists, App, IntoSystemConfigs, Plugin, PostUpdate};
use combat::melee_queries::melee_attack_handler;
use combat::sfx::{attack_sfx, health_combat_hit_result_sfx};
use entity::base_mesh::link_base_mesh;
use entity::entity_types::register_entity_type;
use entity::loading::load_entity;
use entity::spawn::build_base_entities;

use gridmap::construction::{GridmapConstructionState, ShowYLevelPlane, YPlaneSet};
use gridmap::grid::EditTileSet;
use hud::inventory::items::update_inventory_hud_add_item_to_slot;
use hud::inventory::slots::InventoryHudSet;
use inventory::client::items::{
    active_item_display_camera, ClientActiveCameraItem, ClientBuildInventoryLabel,
};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_bodies;
use resources::modes::is_server_mode;
use resources::ordering::{ActionsSet, BuildingSet, CombatSet, PreUpdate, Update, UpdateSet};
use resources::plugin::SpawnItemSet;
use ui::text_input::TextTree;

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
        if is_server_mode(app) {
            app.add_event::<InputConstruct>()
                .add_event::<InputDeconstruct>()
                .add_event::<InputConstructionOptions>()
                .add_systems(
                    Update,
                    (
                        melee_attack_handler::<ConstructionTool>
                            .in_set(CombatSet::WeaponHandler)
                            .after(CombatSet::CacheAttack),
                        attack_sfx::<ConstructionTool>
                            .after(CombatSet::WeaponHandler)
                            .after(CombatSet::Query),
                        health_combat_hit_result_sfx::<ConstructionTool>
                            .after(CombatSet::FinalizeApplyDamage),
                        construction_tool_inventory_prequisite_check
                            .in_set(ActionsSet::Approve)
                            .after(ActionsSet::Build),
                        deconstruct_action_prequisite_check
                            .in_set(ActionsSet::Approve)
                            .after(ActionsSet::Build),
                        construct_action_prequisite_check
                            .in_set(ActionsSet::Approve)
                            .after(ActionsSet::Build),
                        construction_tool_actions
                            .in_set(ActionsSet::Action)
                            .after(ActionsSet::Approve),
                        build_actions
                            .in_set(ActionsSet::Build)
                            .after(ActionsSet::Init),
                        construction_tool_select_construction_option
                            .after(TextTree::Input)
                            .in_set(UpdateSet::TextTreeInputSelection),
                        send_constructable_items.after(construction_tool_actions),
                        mouse_click_input
                            .before(EditTileSet::Add)
                            .in_set(EditTileSet::Remove),
                    ),
                );
        } else {
            app.add_systems(
                Update,
                (
                    update_inventory_hud_add_item_to_slot::<ConstructionToolType>
                        .after(InventoryHudSet::UpdateSlot)
                        .in_set(InventoryHudSet::QueueUpdate)
                        .after(ClientBuildInventoryLabel::Net),
                    construction_tool_enable_select_cell_in_front_camera
                        .run_if(resource_exists::<GridmapConstructionState>)
                        .in_set(YPlaneSet::Show),
                ),
            )
            .add_systems(
                PreUpdate,
                (
                    load_entity::<ConstructionToolType>
                        .before(BuildingSet::NormalBuild)
                        .in_set(BuildingSet::TriggerBuild),
                    link_base_mesh::<ConstructionToolType>.in_set(BuildingSet::NormalBuild),
                ),
            )
            .add_systems(
                PostUpdate,
                active_item_display_camera::<ConstructionToolType>
                    .after(ClientActiveCameraItem::ActivateItem),
            );
        }
        register_entity_type::<ConstructionToolType>(app);
        register_basic_console_commands_for_type::<ConstructionToolType>(app);
        register_basic_console_commands_for_inventory_item_type::<ConstructionToolType>(app);
        app.add_systems(
            PreUpdate,
            (
                build_construction_tools::<ConstructionToolType>.after(BuildingSet::NormalBuild),
                (build_rigid_bodies::<ConstructionToolType>).after(BuildingSet::NormalBuild),
                (build_base_entities::<ConstructionToolType>).after(BuildingSet::NormalBuild),
                (build_inventory_items::<ConstructionToolType>)
                    .after(BuildingSet::NormalBuild)
                    .in_set(SpawnItemSet::AddingComponent),
            ),
        )
        .add_event::<ShowYLevelPlane>();
    }
}
