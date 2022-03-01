use bevy_ecs::{prelude::Changed, system::Query};

use crate::space::core::{
    data_link::components::{DataLink, DataLinkType},
    pawn::components::{Senser, SensingAbility},
};

pub fn atmospherics_sensing_ability(
    mut data_linked: Query<(&DataLink, &mut Senser), Changed<DataLink>>,
) {
    for (data_link_component, mut senser_component) in data_linked.iter_mut() {
        if data_link_component
            .links
            .contains(&DataLinkType::FullAtmospherics)
            && senser_component
                .sensing_abilities
                .contains(&SensingAbility::Atmospherics)
                == false
        {
            senser_component
                .sensing_abilities
                .push(SensingAbility::Atmospherics);
        } else if data_link_component
            .links
            .contains(&DataLinkType::FullAtmospherics)
            == false
            && senser_component
                .sensing_abilities
                .contains(&SensingAbility::Atmospherics)
        {
            let index = senser_component
                .sensing_abilities
                .iter()
                .position(|r| r == &SensingAbility::Atmospherics)
                .unwrap();

            senser_component.sensing_abilities.remove(index);
        }
    }
}
