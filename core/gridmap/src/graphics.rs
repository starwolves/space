use bevy::gltf::GltfMesh;
use bevy::prelude::Assets;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;

use bevy::prelude::warn;
use bevy::prelude::PbrBundle;
use bevy::prelude::Res;
use resources::grid::TargetCell;

use crate::grid::AddTile;
use crate::grid::Gridmap;
use crate::init::DefaultGridMaterials;

pub(crate) fn set_cell_graphics(
    mut events: EventReader<AddTile>,
    gridmap_main: Res<Gridmap>,
    mut commands: Commands,
    materials: Res<DefaultGridMaterials>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
) {
    for set_cell in events.iter() {
        match gridmap_main.main_cell_properties.get(&set_cell.tile_type) {
            Some(properties) => {
                let transform = gridmap_main.get_cell_transform(
                    TargetCell {
                        id: set_cell.id,
                        face: set_cell.face.clone(),
                    },
                    set_cell.orientation,
                );
                match assets_gltfmesh.get(&properties.mesh_option.clone().unwrap()) {
                    Some(mesh) => {
                        commands.entity(set_cell.entity).insert(PbrBundle {
                            mesh: mesh.primitives[0].mesh.clone(),
                            material: materials.gray_metallic.clone(),
                            transform,
                            ..Default::default()
                        });
                    }
                    None => {
                        warn!("Couldnt find gltf mesh!");
                    }
                }
            }
            None => {
                warn!("Couldnt find maincellproperties!");
            }
        }
    }
}
