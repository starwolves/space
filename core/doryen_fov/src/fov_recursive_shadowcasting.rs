use crate::{FovAlgorithm, MapData};

const MULT0: [i32; 8] = [1, 0, 0, -1, -1, 0, 0, 1];
const MULT1: [i32; 8] = [0, 1, -1, 0, 0, -1, 1, 0];
const MULT2: [i32; 8] = [0, 1, 1, 0, 0, -1, -1, 0];
const MULT3: [i32; 8] = [1, 0, 0, 1, -1, 0, 0, -1];

pub struct FovRecursiveShadowCasting {
    /// width x height vector of field of view information
    pub fov: Vec<bool>,
    /// width of the map in cells
    pub width: usize,
    /// height of the map in cells
    pub height: usize,
}

impl FovRecursiveShadowCasting {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            fov: vec![false; width * height],
        }
    }
    fn cast_light(
        &mut self,
        map: &mut MapData,
        cx: i32,
        cy: i32,
        row: i32,
        start_p: f32,
        end: f32,
        radius: i32,
        r2: i32,
        xx: i32,
        xy: i32,
        yx: i32,
        yy: i32,
        id: i32,
        light_walls: bool,
    ) {
        if start_p < end {
            return;
        }
        let mut start = start_p;
        let mut new_start = 0.0;
        for j in row..=radius {
            let mut dx = -j - 1;
            let dy = -j;
            let mut blocked = false;
            while dx <= 0 {
                dx += 1;
                let cur_x = cx + dx * xx + dy * xy;
                let cur_y = cy + dx * yx + dy * yy;
                if cur_x >= 0 && cur_x < map.width as i32 && cur_y >= 0 && cur_y < map.height as i32
                {
                    let off = cur_x as usize + cur_y as usize * map.width;
                    let l_slope = (dx as f32 - 0.5) / (dy as f32 + 0.5);
                    let r_slope = (dx as f32 + 0.5) / (dy as f32 - 0.5);
                    if start < r_slope {
                        continue;
                    } else if end > l_slope {
                        break;
                    }
                    if dx * dx + dy * dy <= r2 && (light_walls || map.transparent[off]) {
                        self.fov[off] = true;
                    }
                    if blocked {
                        if !map.transparent[off] {
                            new_start = r_slope;
                            continue;
                        } else {
                            blocked = false;
                            start = new_start;
                        }
                    } else if !map.transparent[off] && j < radius {
                        blocked = true;
                        self.cast_light(
                            map,
                            cx,
                            cy,
                            j + 1,
                            start,
                            l_slope,
                            radius,
                            r2,
                            xx,
                            xy,
                            yx,
                            yy,
                            id + 1,
                            light_walls,
                        );
                        new_start = r_slope;
                    }
                }
            }
            if blocked {
                break;
            }
        }
    }
    /// reset the fov information to false
    pub fn clear_fov(&mut self) {
        for off in 0..self.width * self.height {
            self.fov[off] = false;
        }
    }
    pub fn is_in_fov(&self, x: usize, y: usize) -> bool {
        let i = x + y * self.width;
        if i >= self.fov.len() {
            false
        } else {
            self.fov[i]
        }
    }

    pub fn set_fov(&mut self, x: usize, y: usize, in_fov: bool) {
        let mut i = x + y * self.width;

        i = i.clamp(0, self.fov.len());

        self.fov[i] = in_fov;
    }
}

impl FovAlgorithm for FovRecursiveShadowCasting {
    fn compute_fov(
        &mut self,
        map: &mut MapData,
        x: usize,
        y: usize,
        max_radius_p: usize,
        light_walls: bool,
    ) {
        let max_radius = if max_radius_p == 0 {
            let max_radius_x = (map.width - x).max(x);
            let max_radius_y = (map.height - y).max(y);
            ((max_radius_x * max_radius_x + max_radius_y * max_radius_y) as f32).sqrt() as usize + 1
        } else {
            max_radius_p
        };
        let r2 = max_radius * max_radius;
        for oct in 0..8 {
            self.cast_light(
                map,
                x as i32,
                y as i32,
                1,
                1.0,
                0.0,
                max_radius as i32,
                r2 as i32,
                MULT0[oct],
                MULT1[oct],
                MULT2[oct],
                MULT3[oct],
                0,
                light_walls,
            );
        }
        self.fov[x + y * map.width] = true;
    }
}
