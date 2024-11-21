use std::collections::BTreeMap;

use bevy::{
    math::Vec3,
    prelude::{App, IntoSystemConfigs, Plugin, Transform},
};
use entity::{
    entity_types::register_entity_type,
    examine::{Examinable, RichName},
    loading::load_entity,
    spawn::{build_base_entities, BaseEntityBuilder, BaseEntityBundle, EntityBuildData, NoData},
};
use resources::{
    modes::is_server_mode,
    ordering::{BuildingSet, PreUpdate},
};

use crate::spawn::{build_point_lights, PointLightType};

use entity::entity_types::EntityType;

impl BaseEntityBuilder<NoData> for PointLightType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, "A point light.".to_string());
        BaseEntityBundle {
            default_transform: Transform::from_translation(Vec3::ZERO),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "Point light".to_string(),
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(PointLightType::new()),
            ..Default::default()
        }
    }
}

pub struct PointLightPlugin;

impl Plugin for PointLightPlugin {
    fn build(&self, app: &mut App) {
        if !is_server_mode(app) {
            app.add_systems(
                PreUpdate,
                (load_entity::<PointLightType>
                    .before(BuildingSet::NormalBuild)
                    .in_set(BuildingSet::TriggerBuild),),
            );
        }
        app.add_systems(
            PreUpdate,
            (
                build_point_lights::<PointLightType>,
                build_base_entities::<PointLightType>,
            )
                .in_set(BuildingSet::NormalBuild),
        );
        register_entity_type::<PointLightType>(app);
    }
}
