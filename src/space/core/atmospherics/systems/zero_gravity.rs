use bevy_internal::prelude::{Query, Entity, Res, Commands};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, ColliderMaterialComponent, RigidBodyPositionComponent,
};

use crate::space::core::{
    atmospherics::components::ZeroGravity,
    gridmap::{functions::gridmap_functions::world_to_cell_id, resources::GridmapMain},
    rigid_body::components::RigidBodyData,
};

pub fn zero_gravity(
    mut rigid_bodies: Query<(
        Entity,
        &RigidBodyPositionComponent,
        Option<&ZeroGravity>,
        &mut ColliderMaterialComponent,
        &RigidBodyData,
    )>,
    gridmap_main: Res<GridmapMain>,
    mut commands: Commands,
) {
    for (
        rigidbody_entity,
        rigidbody_position_component,
        zero_gravity_component_option,
        mut collider_material_component,
        rigidbody_data_component,
    ) in rigid_bodies.iter_mut()
    {
        let mut cell_id =
            world_to_cell_id(rigidbody_position_component.position.translation.into());

        cell_id.y = -1;

        match gridmap_main.grid_data.get(&cell_id) {
            Some(_) => {
                if zero_gravity_component_option.is_some() {
                    collider_material_component.friction = rigidbody_data_component.friction;
                    collider_material_component.friction_combine_rule =
                        rigidbody_data_component.friction_combine_rule;
                    commands.entity(rigidbody_entity).remove::<ZeroGravity>();
                }
            }
            None => {
                if zero_gravity_component_option.is_none() {
                    collider_material_component.friction = 0.;
                    collider_material_component.friction_combine_rule = CoefficientCombineRule::Min;
                    commands.entity(rigidbody_entity).insert(ZeroGravity);
                }
            }
        }
    }
}
