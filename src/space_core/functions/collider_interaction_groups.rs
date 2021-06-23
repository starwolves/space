pub fn get_bit_masks(
    group : ColliderGroup
) -> (u32, u32) {



    match group {
        ColliderGroup::Standard => {
            (0b00000000000000000000000000000001, 0b00000000000000000000000000000001)
        },
        ColliderGroup::FOV => {
            (0b00000000000000000000000000000010, 0b00000000000000000000000000000010)
        },
        ColliderGroup::StandardFOV => {
            (0b00000000000000000000000000000011, 0b00000000000000000000000000000011)
        },
    }

}


#[allow(dead_code)]
pub enum ColliderGroup {
    Standard,
    FOV,
    StandardFOV,
}
