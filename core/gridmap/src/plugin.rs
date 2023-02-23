use bevy::{
    prelude::{App, IntoSystemDescriptor, Plugin, SystemSet},
    time::FixedTimestep,
};
use entity::entity_data::INTERPOLATION_LABEL1;
use networking::messaging::{register_reliable_message, MessageSender};
use player::plugin::ConfigurationLabel;
use resources::{
    is_server::is_server,
    labels::{ActionsLabels, BuildingLabels, PostUpdateLabels, StartupLabels},
};

use crate::{
    connections::configure,
    examine::{
        examine_grid, examine_map, examine_map_abilities, examine_map_health, finalize_examine_map,
        finalize_grid_examine_input, incoming_messages, set_action_header_name,
        GridmapExamineMessages, InputExamineMap,
    },
    fov::ProjectileFOV,
    graphics::set_cell_graphics,
    grid::{add_tile, add_tile_collision, AddGroup, AddTile, Gridmap, RemoveCell},
    init::{load_ron_gridmap, startup_map_tile_properties, startup_misc_resources},
    net::{GridmapClientMessage, GridmapServerMessage},
    select_cell_yplane::{
        change_ghost_tile_request, create_select_cell_cam_state, input_ghost_rotation,
        input_yplane_position, move_ylevel_plane, select_cell_in_front_camera, set_yplane_position,
        show_ylevel_plane, update_ghost_cell, GhostTileLabel, SelectCellSelectionChanged,
        SetYPlanePosition,
    },
    wall::add_wall_group,
};
use bevy::app::CoreStage::{PostUpdate, PreUpdate};

use super::{
    fov::{senser_update_fov, DoryenMap},
    sensing_ability::gridmap_sensing_ability,
    updates::gridmap_updates_manager,
};

pub struct GridmapPlugin;

impl Plugin for GridmapPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(senser_update_fov)
                .add_event::<RemoveCell>()
                .add_system_set(
                    SystemSet::new()
                        .with_run_criteria(
                            FixedTimestep::step(1. / 4.).with_label(INTERPOLATION_LABEL1),
                        )
                        .with_system(gridmap_updates_manager),
                )
                .add_system(gridmap_sensing_ability)
                .add_system(examine_map.after(ActionsLabels::Action))
                .add_system(
                    set_action_header_name
                        .after(ActionsLabels::Build)
                        .before(ActionsLabels::Approve),
                )
                .add_system(examine_map.after(ActionsLabels::Action))
                .add_system(examine_map_health.after(ActionsLabels::Action))
                .add_system(examine_map_abilities.after(ActionsLabels::Action))
                .add_event::<ProjectileFOV>()
                .add_system_to_stage(PreUpdate, finalize_grid_examine_input)
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<InputExamineMap>()
                .init_resource::<GridmapExamineMessages>()
                .add_system_to_stage(
                    PostUpdate,
                    finalize_examine_map.before(PostUpdateLabels::EntityUpdate),
                )
                .add_system(examine_grid.after(ActionsLabels::Action))
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                );
        } else {
            app.add_system(set_cell_graphics)
                .add_startup_system(create_select_cell_cam_state)
                .add_event::<SetYPlanePosition>()
                .add_system(show_ylevel_plane)
                .add_system(set_yplane_position)
                .add_system(input_yplane_position)
                .add_system(move_ylevel_plane)
                .add_system_set(
                    SystemSet::new()
                        .with_run_criteria(FixedTimestep::step(1. / 8.))
                        .with_system(select_cell_in_front_camera),
                )
                .add_system(update_ghost_cell.label(GhostTileLabel::Update))
                .add_event::<SelectCellSelectionChanged>()
                .add_system(change_ghost_tile_request)
                .add_system(input_ghost_rotation.after(GhostTileLabel::Update));
        }

        app.add_startup_system(startup_misc_resources.label(StartupLabels::MiscResources))
            .add_startup_system(
                startup_map_tile_properties
                    .label(StartupLabels::InitDefaultGridmapData)
                    .label(BuildingLabels::TriggerBuild)
                    .after(StartupLabels::MiscResources),
            )
            .add_startup_system(
                load_ron_gridmap
                    .label(StartupLabels::BuildGridmap)
                    .after(StartupLabels::InitDefaultGridmapData),
            )
            .init_resource::<Gridmap>()
            .init_resource::<DoryenMap>()
            .add_system(add_tile)
            .add_event::<AddTile>()
            .add_event::<AddGroup>()
            .add_system(add_wall_group)
            .add_system(add_tile_collision);

        register_reliable_message::<GridmapClientMessage>(app, MessageSender::Client);
        register_reliable_message::<GridmapServerMessage>(app, MessageSender::Server);
    }
}
