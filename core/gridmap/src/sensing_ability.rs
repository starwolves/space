use bevy::prelude::{Changed, Query};
use api::{
    data_link::{DataLink, DataLinkType},
    senser::{Senser, SensingAbility},
};

pub fn gridmap_sensing_ability(
    mut data_linked: Query<(&DataLink, &mut Senser), Changed<DataLink>>,
) {
    for (data_link_component, mut senser_component) in data_linked.iter_mut() {
        if data_link_component
            .links
            .contains(&DataLinkType::ShipEngineeringKnowledge)
            && senser_component
                .sensing_abilities
                .contains(&SensingAbility::ShipEngineerKnowledge)
                == false
        {
            senser_component
                .sensing_abilities
                .push(SensingAbility::ShipEngineerKnowledge);
        } else if data_link_component
            .links
            .contains(&DataLinkType::ShipEngineeringKnowledge)
            == false
            && senser_component
                .sensing_abilities
                .contains(&SensingAbility::ShipEngineerKnowledge)
        {
            let index = senser_component
                .sensing_abilities
                .iter()
                .position(|r| r == &SensingAbility::ShipEngineerKnowledge)
                .unwrap();

            senser_component.sensing_abilities.remove(index);
        }
    }
}
