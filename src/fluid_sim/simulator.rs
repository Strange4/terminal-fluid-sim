use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FluidSim {
    // grid aranged like cartesian, bottom left is (0,0)
    horizontal_values: Vec<Vec<f32>>,
    vertical_values: Vec<Vec<f32>>,
    pressure_grid: Vec<Vec<f32>>,
    smoke_grid: Vec<Vec<f32>>,
    block_grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    density: f32,
    last_instant: Instant,
}

impl Default for FluidSim {
    fn default() -> Self {
        FluidSim::new(2, 2, 1000.0)
    }
}

impl FluidSim {
    pub fn new(width: usize, height: usize, density: f32) -> Self {
        Self {
            // plus one since we use a staggered grid
            // grid: Self::make_grid(width, height),
            horizontal_values: vec![vec![0.0; height]; width],
            vertical_values: vec![vec![0.0; height]; width],
            pressure_grid: vec![vec![0.0; height]; width],
            smoke_grid: vec![vec![1.0; height]; width],
            block_grid: Self::make_block_grid(width, height),
            width,
            height,
            density,
            last_instant: Instant::now(),
        }
    }

    #[inline]
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[inline]
    pub fn get_pressure_grid(&self) -> &Vec<Vec<f32>> {
        &self.pressure_grid
    }

    #[inline]
    pub fn get_block_grid(&self) -> &Vec<Vec<bool>> {
        &self.block_grid
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.horizontal_values = vec![vec![0.0; height]; width];
        self.vertical_values = vec![vec![0.0; height]; width];
        self.pressure_grid = vec![vec![0.0; height]; width];
        self.smoke_grid = vec![vec![1.0; height]; width];
        self.block_grid = Self::make_block_grid(width, height);
        self.width = width;
        self.height = height;
    }

    pub fn next_step(&mut self) {
        self.step_through(self.last_instant.elapsed());
        self.last_instant = Instant::now();
    }

    fn step_through(&mut self, delta: Duration) {
        self.update_velocities(delta);
        self.make_incompressible(delta);
        self.move_velocity(delta);
    }

    fn make_block_grid(width: usize, height: usize) -> Vec<Vec<bool>> {
        let mut grid = vec![vec![false; height]; width];
        let left_border = &mut grid[0];
        left_border.fill(true);
        let right_border = &mut grid[width - 1];
        right_border.fill(true);
        for i in 0..width {
            let bottom_border = &mut grid[i][0];
            *bottom_border = true;
        }
        grid
    }

    fn update_velocities(&mut self, delta: Duration) {
        const GRAVITY: f32 = -9.81; // in m/s
        for i in 1..self.width {
            for j in 1..self.height {
                if self.block_grid[i][j] || self.block_grid[i][j - 1] {
                    continue;
                }
                let secs = delta.as_secs_f32();
                self.vertical_values[i][j] += GRAVITY * secs;
            }
        }
    }

    fn make_incompressible(&mut self, delta: Duration) {
        const OVERLAX: f32 = 1.9;
        const ITERATIONS: usize = 100;
        self.pressure_grid.fill(vec![0.0; self.height]);
        let pressure_constant = self.density / delta.as_secs_f32();
        for x in 0..ITERATIONS {
            // avoid borders
            for i in 1..self.width - 1 {
                for j in 1..self.height - 1 {
                    if self.block_grid[i][j] {
                        continue;
                    }
                    let right_is_block = !self.block_grid[i + 1][j] as u8 as f32;
                    let top_is_block = !self.block_grid[i][j + 1] as u8 as f32;
                    let left_is_block = !self.block_grid[i - 1][j] as u8 as f32;
                    let bottom_is_block = !self.block_grid[i][j - 1] as u8 as f32;
                    let number_of_fluids =
                        right_is_block + top_is_block + left_is_block + bottom_is_block;

                    if number_of_fluids == 0.0 {
                        continue;
                    }

                    let divergence = self.horizontal_values[i + 1][j]
                        - self.horizontal_values[i][j]
                        + self.vertical_values[i][j + 1]
                        - self.vertical_values[i][j];

                    let correction = OVERLAX * (-divergence / number_of_fluids);
                    self.horizontal_values[i][j] -= correction * left_is_block;
                    self.horizontal_values[i + 1][j] += correction * right_is_block;
                    self.vertical_values[i][j] -= correction * bottom_is_block;
                    self.vertical_values[i][j + 1] += correction * top_is_block;
                    let old_pressure = self.pressure_grid[i][j];
                    let new_pressure = old_pressure + (pressure_constant * correction);
                    if new_pressure.is_infinite() {
                        panic!("What the hell dude: i: {} j: {} x:{}", i, j, x);
                    }
                    self.pressure_grid[i][j] = new_pressure;
                }
            }
        }
    }

    fn move_velocity(&mut self, delta: Duration) {
        let mut new_horizontal = self.horizontal_values.clone();
        let mut new_vertical = self.vertical_values.clone();
        let mut new_smoke = self.smoke_grid.clone();
        let half_size = 0.5;
        for i in 1..self.width - 1 {
            for j in 1..self.height - 1 {
                if self.block_grid[i][j] {
                    continue;
                }
                // for horizontal
                let mut x_pos = i as f32;
                let mut y_pos = j as f32 + half_size;
                let horizontal_value = self.horizontal_values[i][j];
                let average_vertical_value = self.avg_vertical(i, j);

                x_pos -= horizontal_value * delta.as_secs_f32();
                y_pos -= average_vertical_value * delta.as_secs_f32();
                new_horizontal[i][j] = self.sample_vector(x_pos, y_pos, FieldType::Horizontal);

                // for vertical component
                let mut x_pos = i as f32 + half_size;
                let mut y_pos = j as f32;

                let vertical_value = self.vertical_values[i][j];
                let average_horizontal_value = self.avg_horizontal(i, j);

                x_pos -= average_horizontal_value * delta.as_secs_f32();
                y_pos -= vertical_value * delta.as_secs_f32();

                new_vertical[i][j] = self.sample_vector(x_pos, y_pos, FieldType::Vertical);

                // for smoke
                let cell_vertical_value =
                    (self.vertical_values[i][j] + self.vertical_values[i][j + 1]) * 0.5;
                let cell_horizontal_value =
                    (self.horizontal_values[i][j] + self.horizontal_values[i + 1][j]) * 0.5;

                let x_pos = i as f32 + half_size - cell_horizontal_value * delta.as_secs_f32();
                let y_pos = j as f32 + half_size - cell_vertical_value * delta.as_secs_f32();
                new_smoke[i][j] = self.sample_vector(x_pos, y_pos, FieldType::Smoke);
            }
        }
        self.horizontal_values = new_horizontal;
        self.vertical_values = new_vertical;
        self.smoke_grid = new_smoke;
    }

    fn avg_vertical(&self, i: usize, j: usize) -> f32 {
        let v = &self.vertical_values;
        let sum = v[i - 1][j] + v[i - 1][j + 1] + v[i][j + 1] + v[i][j];
        sum * 0.25
    }

    fn avg_horizontal(&self, i: usize, j: usize) -> f32 {
        let u = &self.horizontal_values;
        let sum = u[i][j - 1] + u[i][j] + u[i + 1][j - 1] + u[i + 1][j];
        sum * 0.25
    }

    fn sample_vector(&self, x: f32, y: f32, field: FieldType) -> f32 {
        let h = 1.0;
        let x = x.min((self.width) as f32).max(0.0);
        let y = y.min((self.height) as f32).max(0.0);

        let inverse_size = 1.0 / h;
        let half_size = 0.5 * h;

        let (field, dx, dy) = match field {
            FieldType::Horizontal => (&self.horizontal_values, 0.0, half_size),
            FieldType::Vertical => (&self.vertical_values, half_size, 0.0),
            FieldType::Smoke => (&self.smoke_grid, half_size, half_size),
        };

        let x_left_index = (((x - dx) * inverse_size).floor() as usize).min(self.width - 1);
        let x_right_index = (x_left_index + 1).min(self.width - 1);
        let x_size_ratio = ((x - dx) - x_left_index as f32 * h) * inverse_size;

        let y_bottom_index = (((y - dy) * inverse_size).trunc() as usize).min(self.height - 1);
        let y_top_index = (y_bottom_index + 1).min(self.height - 1);
        let y_size_ratio = ((y - dy) - y_bottom_index as f32 * h) * inverse_size;

        let sx = 1.0 - x_size_ratio;
        let sy = 1.0 - y_size_ratio;

        let sampled_value = sx * sy * field[x_left_index][y_bottom_index]
            + x_size_ratio * sy * field[x_right_index][y_bottom_index]
            + x_size_ratio * y_size_ratio * field[x_right_index][y_top_index]
            + sx * y_size_ratio * field[x_left_index][y_top_index];
        return sampled_value;
    }
}

enum FieldType {
    Horizontal,
    Vertical,
    Smoke,
}
