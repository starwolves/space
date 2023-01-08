//! For fast (but we will need faster) gridmap based FOV calculation.

mod fov_recursive_shadowcasting;

mod fov_restrictive;

pub use fov_recursive_shadowcasting::*;
pub use fov_restrictive::*;

/// Some basic structure to store map cells' transparency and fov computation result

pub struct MapData {
    /// width of the map in cells
    pub width: usize,
    /// height of the map in cells
    pub height: usize,
    /// width x height vector of transparency information
    pub transparent: Vec<bool>,
}

impl MapData {
    /// create a new empty map : no walls and empty field of view
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            transparent: vec![true; width * height],
        }
    }
    pub fn is_transparent(&self, x: usize, y: usize) -> bool {
        self.transparent[x + y * self.width]
    }
    pub fn set_transparent(&mut self, x: usize, y: usize, is_transparent: bool) {
        self.transparent[x + y * self.width] = is_transparent;
    }
}

/// Some algorithm to compute a field of view
/// x,y : observer position on the map
/// max_radius : max distance in cells where the observer can see. 0 = infinite
/// light_walls : are walls limiting the field of view inside the field of view ?

pub trait FovAlgorithm {
    fn compute_fov(
        &mut self,
        map: &mut MapData,
        x: usize,
        y: usize,
        max_radius: usize,
        light_walls: bool,
    );
}
