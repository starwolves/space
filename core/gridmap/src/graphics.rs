use std::collections::HashMap;

use bevy::gltf::GltfMesh;
use bevy::prelude::Assets;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::EventReader;

use bevy::log::warn;
use bevy::prelude::PbrBundle;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Resource;
use resources::grid::CellFace;
use resources::grid::TargetCell;
use resources::math::Vec3Int;

use crate::grid::AddTile;
use crate::grid::Gridmap;
use crate::items::generic_assets::GenericMaterials;

#[derive(Component)]
pub struct GraphicsGridLink {
    pub id: Vec3Int,
}

#[derive(Default, Resource)]
pub(crate) struct CellGraphicsBuffer {
    pub buffer: HashMap<BufferId, AddTile>,
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct BufferId {
    pub id: Vec3Int,
    pub face: CellFace,
}

pub(crate) fn set_cell_graphics(
    mut events: EventReader<AddTile>,
    gridmap_main: Res<Gridmap>,
    mut commands: Commands,
    materials: Res<GenericMaterials>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    mut buffer: ResMut<CellGraphicsBuffer>,
) {
    for set_cell in events.read() {
        buffer.buffer.insert(
            BufferId {
                id: set_cell.id,
                face: set_cell.face.clone(),
            },
            set_cell.clone(),
        );
    }
    let mut remove_ids = vec![];
    for (id, add_tile) in buffer.buffer.iter() {
        match gridmap_main.tile_properties.get(&add_tile.tile_type) {
            Some(properties) => {
                let transform = gridmap_main.get_cell_transform(
                    TargetCell {
                        id: add_tile.id,
                        face: add_tile.face.clone(),
                    },
                    add_tile.orientation,
                );

                let mat;
                match &properties.material_option {
                    Some(m) => {
                        mat = m;
                    }
                    None => {
                        mat = &materials.gray_metallic;
                    }
                }
                match assets_gltfmesh.get(&properties.mesh_option.clone().unwrap()) {
                    Some(mesh) => {
                        commands.entity(add_tile.entity).insert((
                            PbrBundle {
                                mesh: mesh.primitives[0].mesh.clone(),
                                material: mat.clone(),
                                transform: transform.into(),
                                ..Default::default()
                            },
                            GraphicsGridLink {
                                id: add_tile.id.clone(),
                            },
                        ));
                        remove_ids.push(id.clone());
                    }
                    None => {}
                }
            }
            None => {
                warn!("Couldnt find maincellproperties!");
            }
        }
    }
    for id in remove_ids {
        buffer.buffer.remove(&id);
    }
}
