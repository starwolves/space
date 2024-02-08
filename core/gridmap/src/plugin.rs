use std::time::Duration;

use bevy::{
    prelude::{resource_exists, App, IntoSystemConfigs, Plugin, Startup},
    time::common_conditions::on_timer,
};
use entity::despawn::DespawnEntitySet;
use networking::messaging::{register_reliable_message, MessageSender, MessagingSet};
use player::{connections::process_response, plugin::ConfigurationLabel};
use resources::{
    modes::{is_correction_mode, is_server_mode},
    ordering::{ActionsSet, BuildingSet, PreUpdate, StartupSet, Update},
};

use crate::{
    connections::configure,
    construction::{
        apply_ghost_rotation, change_ghost_tile_request, client_mouse_click_input,
        create_select_cell_cam_state, input_yplane_position, move_ylevel_plane, register_input,
        select_cell_in_front_camera, set_yplane_position, show_ylevel_plane, update_ghost_cell,
        ConstructionCellSelectionChanged, ConstructionSelection, GridmapConstructionState,
        NewGhostBuffer, SetYPlanePosition, YPlaneSet,
    },
    examine::{
        examine_grid, examine_map, examine_map_abilities, examine_map_health, finalize_examine_map,
        finalize_grid_examine_input, incoming_messages, set_action_header_name,
        GridmapExamineMessages, InputExamineMap,
    },
    fov::ProjectileFOV,
    graphics::{set_cell_graphics, CellGraphicsBuffer},
    grid::{
        add_cell_client, add_tile, add_tile_collision, add_tile_net, export_debug_map,
        remove_cell_client, remove_tile, remove_tile_net, removed_tile, spawn_group, AddGroup,
        AddTile, EditTileSet, Gridmap, RemoveTile,
    },
    init::{
        init_tile_groups, init_tile_properties, load_ron_gridmap, InitTileGroups,
        InitTileProperties,
    },
    items::{
        bridge_floor::{
            init_bridge_floor_material, init_corner2_bridge_floor, init_corner_bridge_floor,
            init_filled_bridge_floor, init_half_bridge_floor, BridgeFloorMaterial,
        },
        bridge_half_diagonal_ceiling::{
            init_bridge_half_diagonal_ceiling_group, init_bridge_half_diagonal_ceiling_high,
            init_bridge_half_diagonal_ceiling_low, init_bridge_half_diagonal_ceiling_material,
            BridgeHalfDiagonalCeilingMaterial,
        },
        bridge_wall::{
            init_bridge_wall, init_bridge_wall_group, init_bridge_wall_material, BridgeWallMaterial,
        },
        generic_assets::{
            init_default_materials, init_generic_meshes, GenericMaterials, GenericMeshes,
        },
        generic_diagonal_ceiling::init_generic_diagonal_ceiling,
        generic_diagonal_floor::init_generic_diagonal_floor,
        generic_floor::{init_generic_floor, init_generic_floor_material, GenericFloorMaterial},
        generic_half_diagonal_ceiling::{
            init_generic_half_diagonal_ceiling_group, init_generic_half_diagonal_ceiling_high,
            init_generic_half_diagonal_ceiling_low, init_generic_half_diagonal_ceiling_material,
            GenericHalfDiagonalCeilingMaterial,
        },
        generic_half_diagonal_floor::{
            init_generic_half_diagonal_floor_group, init_generic_half_diagonal_floor_high,
            init_generic_half_diagonal_floor_low, init_generic_half_diagonal_floor_material,
            GenericHalfDiagonalFloorMaterial,
        },
        generic_wall::{
            init_generic_wall, init_generic_wall_group, init_generic_wall_material,
            GenericWallMaterial,
        },
        reinforced_glass_floor::{
            init_reinforced_glass_floor, init_reinforced_glass_floor_material,
            ReinforcedGlassFloorMaterial,
        },
        reinforced_glass_half_diagonal::{
            init_reinforced_glass_half_diagonal_ceiling_group,
            init_reinforced_glass_half_diagonal_ceiling_high,
            init_reinforced_glass_half_diagonal_ceiling_low,
            init_reinforced_glass_half_diagonal_ceiling_material,
            HalfDiagonalReinforcedGlassMaterial,
        },
        reinforced_glass_wall::{
            init_reinforced_glass_wall, init_reinforced_glass_wall_material,
            ReinforcedGlassWallMaterial,
        },
    },
    net::{GridmapClientMessage, GridmapServerMessage},
};

use super::{
    fov::{senser_update_fov, DoryenMap},
    sensing_ability::gridmap_sensing_ability,
};

pub struct GridmapPlugin;

impl Plugin for GridmapPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) && !is_correction_mode(app) {
            app.add_systems(
                Update,
                (
                    senser_update_fov,
                    gridmap_sensing_ability,
                    examine_map.after(ActionsSet::Action),
                    set_action_header_name
                        .after(ActionsSet::Build)
                        .before(ActionsSet::Approve),
                    examine_map.after(ActionsSet::Action),
                    examine_map_health.after(ActionsSet::Action),
                    examine_map_abilities.after(ActionsSet::Action),
                    examine_grid.after(ActionsSet::Action),
                    configure
                        .after(process_response)
                        .in_set(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                    add_tile_net.after(EditTileSet::Add),
                    remove_tile_net.after(EditTileSet::Remove),
                ),
            )
            .add_event::<ProjectileFOV>()
            .add_systems(
                PreUpdate,
                (
                    incoming_messages,
                    finalize_grid_examine_input,
                    finalize_examine_map,
                )
                    .after(MessagingSet::DeserializeIncoming),
            )
            .add_event::<InputExamineMap>()
            .init_resource::<GridmapExamineMessages>();
        }
        if !is_server_mode(app) {
            app.init_resource::<NewGhostBuffer>()
                .init_resource::<CellGraphicsBuffer>()
                .add_systems(Startup, export_debug_map)
                .add_systems(
                    Update,
                    (
                        add_cell_client.before(EditTileSet::Add),
                        remove_cell_client.in_set(EditTileSet::Remove),
                        removed_tile
                            .after(EditTileSet::Remove)
                            .before(DespawnEntitySet),
                        set_cell_graphics.after(EditTileSet::Add),
                        create_select_cell_cam_state,
                        set_yplane_position
                            .run_if(resource_exists::<GridmapConstructionState>())
                            .after(YPlaneSet::Input)
                            .after(YPlaneSet::Position),
                        show_ylevel_plane
                            .run_if(resource_exists::<GridmapConstructionState>())
                            .after(YPlaneSet::Show)
                            .in_set(YPlaneSet::Position),
                        input_yplane_position
                            .in_set(YPlaneSet::Input)
                            .in_set(YPlaneSet::Position)
                            .run_if(resource_exists::<GridmapConstructionState>()),
                        move_ylevel_plane.run_if(resource_exists::<GridmapConstructionState>()),
                        update_ghost_cell
                            .after(ConstructionSelection::Changed)
                            .run_if(resource_exists::<GridmapConstructionState>()),
                        change_ghost_tile_request
                            .in_set(ConstructionSelection::Changed)
                            .run_if(resource_exists::<GridmapConstructionState>()),
                        select_cell_in_front_camera
                            .in_set(ConstructionSelection::Changed)
                            .run_if(resource_exists::<GridmapConstructionState>())
                            .run_if(on_timer(Duration::from_secs_f32(1. / 8.))),
                        apply_ghost_rotation
                            .after(ConstructionSelection::Changed)
                            .run_if(resource_exists::<GridmapConstructionState>())
                            .before(update_ghost_cell),
                        (client_mouse_click_input
                            .after(update_ghost_cell)
                            .run_if(resource_exists::<GridmapConstructionState>()),),
                    ),
                )
                .add_event::<SetYPlanePosition>()
                .add_event::<ConstructionCellSelectionChanged>()
                .add_systems(
                    Startup,
                    (
                        init_generic_meshes,
                        register_input,
                        init_default_materials,
                        init_generic_wall_material.before(init_generic_wall),
                        init_bridge_wall_material.before(init_bridge_wall),
                        init_generic_floor_material.before(init_generic_floor),
                        init_generic_half_diagonal_floor_material
                            .before(init_generic_half_diagonal_floor_low)
                            .before(init_generic_half_diagonal_floor_high),
                        init_generic_half_diagonal_ceiling_material
                            .before(init_generic_half_diagonal_ceiling_low)
                            .before(init_generic_half_diagonal_ceiling_high),
                        init_bridge_half_diagonal_ceiling_material
                            .before(init_bridge_half_diagonal_ceiling_low)
                            .before(init_bridge_half_diagonal_ceiling_high),
                        init_bridge_floor_material
                            .before(init_filled_bridge_floor)
                            .before(init_half_bridge_floor)
                            .before(init_corner_bridge_floor)
                            .before(init_corner2_bridge_floor),
                        init_reinforced_glass_half_diagonal_ceiling_material
                            .before(init_reinforced_glass_half_diagonal_ceiling_low)
                            .before(init_reinforced_glass_half_diagonal_ceiling_high),
                        init_reinforced_glass_wall_material.before(init_reinforced_glass_wall),
                        init_reinforced_glass_floor_material.before(init_reinforced_glass_floor),
                    ),
                );
        }
        app.init_resource::<GenericMaterials>()
            .init_resource::<GenericMeshes>()
            .init_resource::<GenericWallMaterial>()
            .init_resource::<GenericHalfDiagonalFloorMaterial>()
            .init_resource::<GenericFloorMaterial>()
            .init_resource::<GenericHalfDiagonalCeilingMaterial>()
            .init_resource::<BridgeFloorMaterial>()
            .init_resource::<ReinforcedGlassWallMaterial>()
            .init_resource::<ReinforcedGlassFloorMaterial>()
            .init_resource::<HalfDiagonalReinforcedGlassMaterial>()
            .init_resource::<BridgeWallMaterial>()
            .init_resource::<BridgeHalfDiagonalCeilingMaterial>()
            .add_systems(
                Startup,
                load_ron_gridmap
                    .before(EditTileSet::Add)
                    .in_set(StartupSet::BuildGridmap)
                    .after(StartupSet::InitDefaultGridmapData),
            )
            .add_systems(
                Startup,
                (
                    (
                        init_tile_properties
                            .in_set(StartupSet::InitDefaultGridmapData)
                            .in_set(BuildingSet::TriggerBuild)
                            .after(StartupSet::MiscResources),
                        init_tile_groups.after(init_tile_properties),
                        init_generic_floor
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_generic_wall_group
                            .after(init_tile_properties)
                            .after(init_generic_meshes)
                            .before(init_tile_groups),
                        init_bridge_wall_group
                            .after(init_tile_properties)
                            .after(init_generic_meshes)
                            .before(init_tile_groups),
                        init_generic_half_diagonal_ceiling_group
                            .after(init_tile_properties)
                            .after(init_generic_meshes)
                            .before(init_tile_groups),
                        init_bridge_half_diagonal_ceiling_group
                            .after(init_tile_properties)
                            .after(init_generic_meshes)
                            .before(init_tile_groups),
                        init_reinforced_glass_half_diagonal_ceiling_group
                            .after(init_tile_properties)
                            .after(init_generic_meshes)
                            .before(init_tile_groups),
                        init_generic_half_diagonal_floor_group
                            .after(init_tile_properties)
                            .after(init_generic_meshes)
                            .before(init_tile_groups),
                        init_generic_wall
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_bridge_wall
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_reinforced_glass_wall
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_reinforced_glass_floor
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_generic_diagonal_floor
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_generic_diagonal_ceiling
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_generic_half_diagonal_ceiling_low
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_bridge_half_diagonal_ceiling_low
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_reinforced_glass_half_diagonal_ceiling_low
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_generic_half_diagonal_floor_low
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                    ),
                    (
                        init_generic_half_diagonal_ceiling_high
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_bridge_half_diagonal_ceiling_high
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_reinforced_glass_half_diagonal_ceiling_high
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_generic_half_diagonal_floor_high
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_filled_bridge_floor
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_half_bridge_floor
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_corner_bridge_floor
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                        init_corner2_bridge_floor
                            .before(init_tile_properties)
                            .after(init_generic_meshes),
                    ),
                ),
            )
            .init_resource::<Gridmap>()
            .init_resource::<DoryenMap>()
            .add_systems(
                Update,
                (
                    remove_tile
                        .after(EditTileSet::Remove)
                        .before(DespawnEntitySet),
                    add_tile_collision.after(EditTileSet::Add),
                    add_tile.after(EditTileSet::Add),
                    spawn_group.before(EditTileSet::Add),
                ),
            )
            .add_event::<AddTile>()
            .add_event::<AddGroup>()
            .add_event::<RemoveTile>()
            .init_resource::<InitTileProperties>()
            .init_resource::<InitTileGroups>();

        register_reliable_message::<GridmapClientMessage>(app, MessageSender::Client, true);
        register_reliable_message::<GridmapServerMessage>(app, MessageSender::Server, true);
    }
}
