use crate::{FovAlgorithm, MapData};

pub struct FovRestrictive {
    start_angle: Vec<f64>,
    end_angle: Vec<f64>,
    allocated: usize,

    /// width x height vector of field of view information
    pub fov: Vec<bool>,
    /// width of the map in cells
    pub width: usize,
    /// height of the map in cells
    pub height: usize,
}

// Mingos' Restrictive Precise Angle Shadowcasting (MRPAS) v1.2

impl FovRestrictive {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            start_angle: Vec::new(),
            end_angle: Vec::new(),
            allocated: 0,
            width,
            height,
            fov: vec![false; width * height],
        }
    }
    fn quadrant(
        &mut self,
        map: &mut MapData,
        player_x: i32,
        player_y: i32,
        max_radius: usize,
        light_walls: bool,
        dx: i32,
        dy: i32,
    ) {
        // octant: vertical edge
        {
            let mut iteration = 1; /* iteration of the algo for this octant */
            let mut done = false;
            let mut total_obstacles = 0;
            let mut obstacles_in_last_line = 0;
            let mut min_angle = 0.0;

            /* do while there are unblocked slopes left and the algo is within the map's boundaries
            scan progressive lines/columns from the PC outwards */
            let mut y = player_y + dy; // the outer slope's coordinates (first processed line)
            if y < 0 || y >= map.height as i32 {
                done = true;
            }
            while !done {
                // process cells in the line
                let slopes_per_cell = 1.0 / f64::from(iteration);
                let half_slopes = slopes_per_cell * 0.5;
                let mut processed_cell = ((min_angle + half_slopes) / slopes_per_cell) as i32;
                let minx = (player_x - iteration).max(0);
                let maxx = (player_x + iteration).min(map.width as i32 - 1);
                done = true;
                let mut x = player_x + processed_cell * dx;
                while x >= minx && x <= maxx {
                    // calculate slopes per cell
                    let c = (x + y * map.width as i32) as usize;
                    let mut visible = true;
                    let mut extended = false;
                    let centre_slope = f64::from(processed_cell) * slopes_per_cell;
                    let start_slope = centre_slope - half_slopes;
                    let end_slope = centre_slope + half_slopes;
                    let off1 = (c as i32 - dy * map.width as i32) as usize;
                    let off2 = (off1 as i32 - dx) as usize;
                    if obstacles_in_last_line > 0 {
                        if !(self.fov[off1] && map.transparent[off1]
                            || self.fov[off2] && map.transparent[off2])
                        {
                            visible = false;
                        } else {
                            for idx in 0..obstacles_in_last_line {
                                if !visible {
                                    break;
                                }
                                if start_slope <= self.end_angle[idx]
                                    && end_slope >= self.start_angle[idx]
                                {
                                    if map.transparent[c] {
                                        if centre_slope > self.start_angle[idx]
                                            && centre_slope < self.end_angle[idx]
                                        {
                                            visible = false;
                                        }
                                    } else if start_slope >= self.start_angle[idx]
                                        && end_slope <= self.end_angle[idx]
                                    {
                                        visible = false;
                                    } else {
                                        self.start_angle[idx] =
                                            self.start_angle[idx].min(start_slope);
                                        self.end_angle[idx] = self.end_angle[idx].max(end_slope);
                                        extended = true;
                                    }
                                }
                            }
                        }
                    }
                    if visible {
                        done = false;
                        self.fov[c] = true;
                        // if the cell is opaque, block the adjacent slopes
                        if !map.transparent[c] {
                            if min_angle >= start_slope {
                                min_angle = end_slope;
                                /* if min_angle is applied to the last cell in line, nothing more
                                needs to be checked. */
                                if processed_cell == iteration {
                                    done = true;
                                }
                            } else if !extended {
                                self.start_angle[total_obstacles] = start_slope;
                                self.end_angle[total_obstacles] = end_slope;
                                total_obstacles += 1;
                            }
                            if !light_walls {
                                self.fov[c] = false;
                            }
                        }
                    }
                    processed_cell += 1;
                    x += dx;
                }
                if iteration == max_radius as i32 {
                    done = true;
                }
                iteration += 1;
                obstacles_in_last_line = total_obstacles;
                y += dy;
                if y < 0 || y >= map.height as i32 {
                    done = true;
                }
            }
        }
        // octant: horizontal edge
        {
            let mut iteration = 1; // iteration of the algo for this octant
            let mut done = false;
            let mut total_obstacles = 0;
            let mut obstacles_in_last_line = 0;
            let mut min_angle = 0.0;

            /* do while there are unblocked slopes left and the algo is within the map's boundaries
            scan progressive lines/columns from the PC outwards */
            let mut x = player_x + dx; // the outer slope's coordinates (first processed line)
            if x < 0 || x >= map.width as i32 {
                done = true;
            }
            while !done {
                // process cells in the line
                let slopes_per_cell = 1.0 / f64::from(iteration);
                let half_slopes = slopes_per_cell * 0.5;
                let mut processed_cell = ((min_angle + half_slopes) / slopes_per_cell) as i32;
                let miny = (player_y - iteration).max(0);
                let maxy = (player_y + iteration).min(map.height as i32 - 1);
                done = true;
                let mut y = player_y + processed_cell * dy;
                while y >= miny && y <= maxy {
                    let c = (x + y * map.width as i32) as usize;
                    // calculate slopes per cell
                    let mut visible = true;
                    let mut extended = false;
                    let centre_slope = f64::from(processed_cell) * slopes_per_cell;
                    let start_slope = centre_slope - half_slopes;
                    let end_slope = centre_slope + half_slopes;
                    let off1 = (c as i32 - dx) as usize;
                    let off2 = (off1 as i32 - dy * map.width as i32) as usize;
                    if obstacles_in_last_line > 0 {
                        if !(self.fov[off1] && map.transparent[off1]
                            || self.fov[off2] && map.transparent[off2])
                        {
                            visible = false;
                        } else {
                            for idx in 0..obstacles_in_last_line {
                                if !visible {
                                    break;
                                }
                                if start_slope <= self.end_angle[idx]
                                    && end_slope >= self.start_angle[idx]
                                {
                                    if map.transparent[c] {
                                        if centre_slope > self.start_angle[idx]
                                            && centre_slope < self.end_angle[idx]
                                        {
                                            visible = false;
                                        }
                                    } else if start_slope >= self.start_angle[idx]
                                        && end_slope <= self.end_angle[idx]
                                    {
                                        visible = false;
                                    } else {
                                        self.start_angle[idx] =
                                            self.start_angle[idx].min(start_slope);
                                        self.end_angle[idx] = self.end_angle[idx].max(end_slope);
                                        extended = true;
                                    }
                                }
                            }
                        }
                    }
                    if visible {
                        done = false;
                        self.fov[c] = true;
                        // if the cell is opaque, block the adjacent slopes
                        if !map.transparent[c] {
                            if min_angle >= start_slope {
                                min_angle = end_slope;
                                /* if min_angle is applied to the last cell in line, nothing more
                                needs to be checked. */
                                if processed_cell == iteration {
                                    done = true;
                                }
                            } else if !extended {
                                self.start_angle[total_obstacles] = start_slope;
                                self.end_angle[total_obstacles] = end_slope;
                                total_obstacles += 1;
                            }
                            if !light_walls {
                                self.fov[c] = false;
                            }
                        }
                    }
                    processed_cell += 1;
                    y += dy;
                }
                if iteration == max_radius as i32 {
                    done = true;
                }
                iteration += 1;
                obstacles_in_last_line = total_obstacles;
                x += dx;
                if x < 0 || x >= map.width as i32 {
                    done = true;
                }
            }
        }
    }
}

impl FovAlgorithm for FovRestrictive {
    fn compute_fov(
        &mut self,
        map: &mut MapData,
        player_x: usize,
        player_y: usize,
        max_radius: usize,
        light_walls: bool,
    ) {
        // calculate an approximated (excessive, just in case) maximum number of obstacles per octant
        let max_obstacles = map.width * map.height / 7;

        // check memory for angles
        if max_obstacles > self.allocated {
            self.allocated = max_obstacles;
            self.start_angle = vec![0.0; max_obstacles];
            self.end_angle = vec![0.0; max_obstacles];
        }

        // set PC's position as visible
        self.fov[player_x + player_y * map.width] = true;

        // compute the 4 quadrants of the map
        self.quadrant(
            map,
            player_x as i32,
            player_y as i32,
            max_radius,
            light_walls,
            1,
            1,
        );
        self.quadrant(
            map,
            player_x as i32,
            player_y as i32,
            max_radius,
            light_walls,
            1,
            -1,
        );
        self.quadrant(
            map,
            player_x as i32,
            player_y as i32,
            max_radius,
            light_walls,
            -1,
            1,
        );
        self.quadrant(
            map,
            player_x as i32,
            player_y as i32,
            max_radius,
            light_walls,
            -1,
            -1,
        );
    }
}
