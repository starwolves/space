use std::collections::HashMap;

pub struct Health {

    pub health_container : HealthContainer,
    pub health_flags : HashMap<u32, HealthFlag>,
    pub raegent_container : RaegentContainer,

}

pub enum HealthFlag {
    TorsoBruteArmor(f32),
}

pub enum HealthContainer {
    Humanoid(HumanoidHealth)
}

pub struct RaegentContainer {
    raegents : HashMap<String, f32>,
}

pub struct HumanoidHealth {

    pub head_brute : f32,
    pub head_burn: f32,
    pub head_toxin: f32,

    pub torso_brute: f32,
    pub torso_burn: f32,
    pub torso_toxin: f32,

    pub left_arm_brute : f32,
    pub left_arm_burn : f32,
    pub left_arm_toxin : f32,

    pub right_arm_brute : f32,
    pub right_arm_burn : f32,
    pub right_arm_toxin : f32,

    pub right_leg_brute : f32,
    pub right_leg_burn : f32,
    pub right_leg_toxin : f32,

    pub left_leg_brute : f32,
    pub left_leg_burn : f32,
    pub left_leg_toxin : f32,

}


pub struct DamageModel {
    pub brute : f32,
    pub burn : f32,
    pub toxin : f32,
    pub damage_flags : HashMap<u32, DamageFlag>,
}

impl Default for DamageModel {
    fn default() -> Self {
        Self {
            brute: 0.,
            burn: 0.,
            toxin: 0.,
            damage_flags: HashMap::new(),
        }

    }
}

pub enum DamageFlag {
    Stun(f32),
    Floor(f32),
}

impl Health {

    pub fn apply_damage(&mut self, body_part : &str, damage_model : &DamageModel) {



    }

    pub fn tick(&mut self) {
        
    }

}

impl Default for Health {
    fn default() -> Self {
        Self {
            health_container : HealthContainer::Humanoid(HumanoidHealth {
                head_brute : 0.,
                head_burn: 0.,
                head_toxin: 0.,

                torso_brute: 0.,
                torso_burn: 0.,
                torso_toxin: 0.,

                left_arm_brute : 0.,
                left_arm_burn : 0.,
                left_arm_toxin : 0.,

                right_arm_brute : 0.,
                right_arm_burn : 0.,
                right_arm_toxin : 0.,

                right_leg_brute : 0.,
                right_leg_burn : 0.,
                right_leg_toxin : 0.,

                left_leg_brute : 0.,
                left_leg_burn : 0.,
                left_leg_toxin : 0.,
            }),
            health_flags: HashMap::new(),
            raegent_container : RaegentContainer {
                raegents: HashMap::new(),
            }
        }
    }
}
