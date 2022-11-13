use bevy::{
    hierarchy::Children,
    prelude::{warn, Commands, Component, Entity, Query, Res, Transform, With},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use gridmap::grid::GridmapMain;
use math::grid::world_to_cell_id;
use rigid_body::rigid_body::RigidBodyData;

/// Manage zero gravity for rigid bodies.
#[cfg(feature = "server")]
pub(crate) fn zero_gravity(
    mut rigid_bodies: Query<(
        Entity,
        &Transform,
        Option<&ZeroGravity>,
        &Children,
        &RigidBodyData,
    )>,
    mut colliders: Query<&mut Friction, With<Collider>>,
    gridmap_main: Res<GridmapMain>,
    mut commands: Commands,
) {
    for (
        rigidbody_entity,
        rigidbody_position_component,
        zero_gravity_component_option,
        children,
        rigidbody_data_component,
    ) in rigid_bodies.iter_mut()
    {
        let mut collider_child_entity_option = None;

        for child in children.iter() {
            match colliders.get(*child) {
                Ok(_friction_component) => {
                    collider_child_entity_option = Some(child);
                    break;
                }
                Err(_rr) => {}
            }
        }

        let mut collider_material_component;

        match collider_child_entity_option {
            Some(ent) => {
                collider_material_component = colliders.get_mut(*ent).unwrap();
            }
            None => {
                warn!("Couldnt find collider child!");
                continue;
            }
        }

        let mut cell_id = world_to_cell_id(rigidbody_position_component.translation.into());

        cell_id.y = -1;

        match gridmap_main.grid_data.get(&cell_id) {
            Some(_) => {
                if zero_gravity_component_option.is_some() {
                    collider_material_component.coefficient = rigidbody_data_component.friction;
                    collider_material_component.combine_rule =
                        rigidbody_data_component.friction_combine_rule;
                    commands.entity(rigidbody_entity).remove::<ZeroGravity>();
                }
            }
            None => {
                if zero_gravity_component_option.is_none() {
                    collider_material_component.coefficient = 0.;
                    collider_material_component.combine_rule = CoefficientCombineRule::Min;
                    commands.entity(rigidbody_entity).insert(ZeroGravity);
                }
            }
        }
    }
}

/// Component for entities with zero gravity.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct ZeroGravity;
