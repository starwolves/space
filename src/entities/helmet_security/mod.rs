use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::core::{
    entity::{
        functions::initialize_entity_data::initialize_entity_data,
        resources::{EntityDataProperties, EntityDataResource},
        spawn::{summon_base_entity, SpawnEvent},
    },
    inventory_item::spawn::summon_inventory_item,
    rigid_body::spawn::summon_rigid_body,
    StartupLabels, SummoningLabels,
};

use self::spawn::{
    default_summon_helmet_security, summon_helmet, summon_raw_helmet, HelmetSummoner,
};

pub mod components;
pub mod spawn;

pub struct HelmetsPlugin;

impl Plugin for HelmetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(summon_helmet.after(SummoningLabels::TriggerSummon))
            .add_system(
                (summon_base_entity::<HelmetSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_rigid_body::<HelmetSummoner>).after(SummoningLabels::TriggerSummon))
            .add_system(
                (summon_inventory_item::<HelmetSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_raw_helmet).after(SummoningLabels::TriggerSummon))
            .add_event::<SpawnEvent<HelmetSummoner>>()
            .add_system((default_summon_helmet_security).after(SummoningLabels::DefaultSummon));
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: "helmetSecurity".to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
