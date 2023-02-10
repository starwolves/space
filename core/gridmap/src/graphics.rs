use std::f32::consts::PI;

use bevy::prelude::Commands;
use bevy::prelude::EventReader;

use bevy::prelude::Quat;
use bevy::scene::SceneBundle;
use math::grid::cell_id_to_world;

use crate::set_cell::SetCell;
use bevy::prelude::warn;
use bevy::prelude::Res;
use bevy::prelude::Transform;

use crate::grid::Gridmap;

pub(crate) fn set_cell_graphics(
    mut events: EventReader<SetCell>,
    gridmap_main: Res<Gridmap>,
    mut commands: Commands,
) {
    for set_cell in events.iter() {
        match gridmap_main
            .main_cell_properties
            .get(&set_cell.data.item_0.id)
        {
            Some(properties) => {
                let strict = gridmap_main.get_strict_cell(set_cell.id, set_cell.face.clone());

                let mut transform = Transform::from_translation(cell_id_to_world(strict.id));
                match strict.face {
                    crate::grid::StrictCellFace::FrontWall => {
                        transform.translation.z += 0.6;
                        transform.rotation = Quat::from_rotation_y(1. * PI);
                    }
                    crate::grid::StrictCellFace::RightWall => {
                        transform.translation.x += 0.6;
                        transform.rotation = Quat::from_rotation_y(0.5 * PI);
                    }
                    crate::grid::StrictCellFace::Floor => {}
                }

                match &set_cell.data.orientation {
                    Some(rotation) => match rotation {
                        crate::grid::Orientation::FrontFacing => {
                            transform.rotation = Quat::from_rotation_y(0.);
                        }
                        crate::grid::Orientation::BackFacing => {
                            transform.rotation = Quat::from_rotation_y(PI);
                        }
                        crate::grid::Orientation::RightFacing => {
                            transform.rotation = Quat::from_rotation_y(0.5 * PI);
                        }
                        crate::grid::Orientation::LeftFacing => {
                            transform.rotation = Quat::from_rotation_y(1.5 * PI);
                        }
                    },
                    None => {}
                }

                commands.spawn(SceneBundle {
                    scene: properties.mesh_option.clone().unwrap(),
                    transform,
                    ..Default::default()
                });
            }
            None => {
                warn!("Couldnt find maincellproperties!");
            }
        }
    }
}
