pub struct HumanoidHealth {

    pub head_brute : f32,
    pub head_burn : f32,
    pub head_toxin : f32,

    pub torso_brute : f32,
    pub torso_burn : f32,
    pub torso_toxin : f32,

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

impl Default for HumanoidHealth {
    fn default() -> Self {
        Self {
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
        }
    }
}
