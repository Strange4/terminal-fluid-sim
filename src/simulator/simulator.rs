use rayon::prelude::*;
use std::time::{Duration, Instant};

use crate::SimConfig;

pub struct FluidSim {
    /// all the values are indexed by x * height + y
    horizontal_speeds: Vec<f32>,
    vertical_values: Vec<f32>,
    pressure_grid: Vec<f32>,
    smoke_grid: Vec<f32>,
    block_grid: Vec<bool>,
    width: usize,
    height: usize,

    // the last time the smoke has been calculated
    last_instant: Instant,

    config: SimConfig,
}

impl Default for FluidSim {
    fn default() -> Self {
        FluidSim::new(2, 2, SimConfig::default())
    }
}

impl FluidSim {
    pub fn new(width: usize, height: usize, config: SimConfig) -> Self {
        Self {
            horizontal_speeds: Self::create_horizontal_speed(width, height, config.wind_speed),
            vertical_values: vec![0.0; height * width],
            pressure_grid: vec![0.0; height * width],
            smoke_grid: Self::create_smoke_pipe(width, height, config.smoke_size),
            block_grid: Self::make_block_grid(width, height),
            width,
            height,
            config,
            last_instant: Instant::now(),
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.horizontal_speeds =
            Self::create_horizontal_speed(width, height, self.config.wind_speed);
        self.vertical_values = vec![0.0; height * width];
        self.pressure_grid = vec![0.0; height * width];
        self.smoke_grid = Self::create_smoke_pipe(width, height, self.config.smoke_size);
        self.resize_block_grid(width, height);
        self.width = width;
        self.height = height;
        self.last_instant = Instant::now();
    }

    pub fn restart_sim(&mut self) {
        self.resize(self.width, self.height);
    }

    pub fn next_step(&mut self) {
        let delta = self.last_instant.elapsed();
        self.add_gravity(delta);
        self.make_incompressible(delta);
        self.move_velocity(delta);
        self.last_instant = Instant::now();
    }

    pub fn set_config(&mut self, config: SimConfig) {
        Self::set_horizontal_speed(&mut self.horizontal_speeds, self.height, config.wind_speed);
        Self::set_smoke_pipe(&mut self.smoke_grid, self.height, config.smoke_size);
        self.config = config;
    }

    fn create_horizontal_speed(width: usize, height: usize, wind_speed: f32) -> Vec<f32> {
        let mut values = vec![0.0; width * height];
        Self::set_horizontal_speed(&mut values, height, wind_speed);
        values
    }

    fn set_horizontal_speed(horizontal_speeds: &mut [f32], height: usize, wind_speed: f32) {
        horizontal_speeds
            .iter_mut()
            .take(height * 2)
            .skip(height)
            .for_each(|speed| *speed = wind_speed);
    }

    fn create_smoke_pipe(width: usize, height: usize, smoke_size: f32) -> Vec<f32> {
        let mut values = vec![1.0; width * height];
        Self::set_smoke_pipe(&mut values, height, smoke_size);

        values
    }

    fn set_smoke_pipe(smoke_grid: &mut [f32], height: usize, smoke_size: f32) {
        // get rid of the previous smoke if any
        smoke_grid
            .iter_mut()
            .take(height)
            .for_each(|smoke_cell| *smoke_cell = 1.0);

        let pipe_height = height as f32 * smoke_size;
        let middle = height as f32 * 0.5;
        let min_index = (middle - pipe_height * 0.5) as usize;
        let max_index = (middle + pipe_height * 0.5) as usize;

        smoke_grid
            .iter_mut()
            .take(max_index)
            .skip(min_index)
            .for_each(|smoke_cell| *smoke_cell = 0.0);
    }

    fn make_block_grid(width: usize, height: usize) -> Vec<bool> {
        let mut grid = vec![false; height * width];

        // filling the left border
        grid.iter_mut().take(height).for_each(|cell| *cell = true);
        grid
    }

    fn resize_block_grid(&mut self, width: usize, height: usize) {
        // remove the left border
        self.block_grid
            .iter_mut()
            .take(self.height)
            .for_each(|cell| *cell = false);

        self.block_grid.resize(width * height, false);

        // set the left border for the new height
        self.block_grid
            .iter_mut()
            .take(height)
            .for_each(|cell| *cell = true);
    }

    fn add_gravity(&mut self, delta: Duration) {
        self.vertical_values
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, vertical_value)| {
                if self.block_grid[index]
                    || Self::index_is_border_with_size(index, self.width, self.height)
                {
                    return;
                }
                *vertical_value += self.config.gravity * delta.as_secs_f32();
            });
    }

    fn make_incompressible(&mut self, delta: Duration) {
        const OVERLAX: f32 = 1.9;
        const ITERATIONS: usize = 50;
        self.pressure_grid.fill(0.0);
        let pressure_constant = self.config.density / delta.as_secs_f32();

        for _ in 0..ITERATIONS {
            for i in 0..self.width {
                for j in 0..self.height {
                    let index = self.calculate_index(i, j);
                    if self.block_grid[index] || self.index_is_border(index) {
                        continue;
                    }
                    let [top, right, bottom, left] = self.indexes_around(i, j);
                    let top_is_block = !self.block_grid[top] as u8 as f32;
                    let right_is_block = !self.block_grid[right] as u8 as f32;
                    let bottom_is_block = !self.block_grid[bottom] as u8 as f32;
                    let left_is_block = !self.block_grid[left] as u8 as f32;
                    let number_of_fluids =
                        right_is_block + top_is_block + left_is_block + bottom_is_block;

                    if number_of_fluids == 0.0 {
                        continue;
                    }

                    let divergence = self.horizontal_speeds[right] - self.horizontal_speeds[index]
                        + self.vertical_values[top]
                        - self.vertical_values[index];

                    let correction = OVERLAX * (-divergence / number_of_fluids);
                    self.horizontal_speeds[index] -= correction * left_is_block;
                    self.horizontal_speeds[right] += correction * right_is_block;

                    self.vertical_values[index] -= correction * bottom_is_block;
                    self.vertical_values[top] += correction * top_is_block;
                    self.pressure_grid[index] += pressure_constant * correction;
                }
            }
        }
    }

    fn move_velocity(&mut self, delta: Duration) {
        let mut new_horizontal = self.horizontal_speeds.clone();
        let mut new_vertical = self.vertical_values.clone();
        let mut new_smoke = self.smoke_grid.clone();

        let half_size = 0.5;
        (&mut new_horizontal, &mut new_vertical, &mut new_smoke)
            .into_par_iter()
            .enumerate()
            .for_each(|(index, (horizontal_value, vertical_value, smoke_value))| {
                if self.block_grid[index] || self.index_is_border(index) {
                    return;
                }
                // for horizontal
                let (i, j) = self.pos_from_index(index);
                let mut x_pos = i as f32;
                let mut y_pos = j as f32 + half_size;
                let average_vertical_value = self.avg_vertical(i, j);

                x_pos -= *horizontal_value * delta.as_secs_f32();
                y_pos -= average_vertical_value * delta.as_secs_f32();
                *horizontal_value = self.sample_vector(x_pos, y_pos, FieldType::Horizontal);

                // for vertical component
                x_pos = i as f32 + half_size;
                y_pos = j as f32;

                let average_horizontal_value = self.avg_horizontal(i, j);

                x_pos -= average_horizontal_value * delta.as_secs_f32();
                y_pos -= *vertical_value * delta.as_secs_f32();

                *vertical_value = self.sample_vector(x_pos, y_pos, FieldType::Vertical);

                // for smoke
                if self.calculate_index(i + 1, j) < self.horizontal_speeds.len() {
                    let cell_vertical_value = (self.vertical_values[index]
                        + self.vertical_values[self.calculate_index(i, j + 1)])
                        * 0.5;
                    let cell_horizontal_value = (self.horizontal_speeds[index]
                        + self.horizontal_speeds[self.calculate_index(i + 1, j)])
                        * 0.5;

                    x_pos = i as f32 + half_size - cell_horizontal_value * delta.as_secs_f32();
                    y_pos = j as f32 + half_size - cell_vertical_value * delta.as_secs_f32();
                    *smoke_value = self.sample_vector(x_pos, y_pos, FieldType::Smoke);
                }
            });
        self.horizontal_speeds = new_horizontal;
        self.vertical_values = new_vertical;
        self.smoke_grid = new_smoke;
    }

    fn avg_vertical(&self, i: usize, j: usize) -> f32 {
        let v = &self.vertical_values;
        let sum: f32 = [(i - 1, j), (i - 1, j + 1), (i, j + 1), (i, j)]
            .into_iter()
            .map(|(i, j)| {
                let index = self.calculate_index(i, j);
                v[index]
            })
            .sum();
        sum * 0.25
    }

    fn avg_horizontal(&self, i: usize, j: usize) -> f32 {
        let u = &self.horizontal_speeds;
        let sum: f32 = [(i, j - 1), (i + 1, j - 1), (i + 1, j), (i, j)]
            .into_iter()
            .map(|(i, j)| {
                let index = self.calculate_index(i, j);
                u[index]
            })
            .sum();
        sum * 0.25
    }

    fn sample_vector(&self, x: f32, y: f32, field: FieldType) -> f32 {
        let h = 1.0;
        let x = x.min((self.width) as f32).max(h);
        let y = y.min((self.height) as f32).max(h);

        let inverse_size = 1.0 / h;
        let half_size = 0.5 * h;

        let (field, dx, dy) = match field {
            FieldType::Horizontal => (&self.horizontal_speeds, 0.0, half_size),
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

        sx * sy * field[self.calculate_index(x_left_index, y_bottom_index)]
            + x_size_ratio * sy * field[self.calculate_index(x_right_index, y_bottom_index)]
            + x_size_ratio * y_size_ratio * field[self.calculate_index(x_right_index, y_top_index)]
            + sx * y_size_ratio * field[self.calculate_index(x_left_index, y_top_index)]
    }

    #[inline]
    pub fn calculate_index(&self, x_index: usize, y_index: usize) -> usize {
        Self::calculate_index_with_height(self.height, x_index, y_index)
    }

    #[inline]
    pub fn calculate_index_with_height(height: usize, x_index: usize, y_index: usize) -> usize {
        (x_index * height) + y_index
    }

    #[inline]
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[inline]
    pub fn get_pressure_grid(&self) -> &Vec<f32> {
        &self.pressure_grid
    }

    #[inline]
    pub fn get_block_grid(&self) -> &Vec<bool> {
        &self.block_grid
    }

    #[inline]
    pub fn get_smoke_grid(&self) -> &Vec<f32> {
        &self.smoke_grid
    }

    #[inline]
    pub fn set_block(&mut self, x: usize, y: usize) {
        let index = self.calculate_index(x, y);
        self.block_grid[index] = true;
    }

    #[inline]
    pub fn unset_block(&mut self, x: usize, y: usize) {
        let index = self.calculate_index(x, y);
        self.block_grid[index] = false;
    }

    /// calculates the indexes and returns the in the top, right, bottom, left order
    fn indexes_around(&self, x_index: usize, y_index: usize) -> [usize; 4] {
        [
            (x_index, y_index + 1),
            (x_index + 1, y_index),
            (x_index, y_index - 1),
            (x_index - 1, y_index),
        ]
        .into_iter()
        .map(|(x, y)| self.calculate_index(x, y))
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
    }

    #[inline]
    fn pos_from_index(&self, index: usize) -> (usize, usize) {
        let x = index / self.height;
        let y = index % self.height;
        (x, y)
    }

    #[inline]
    fn index_is_border(&self, index: usize) -> bool {
        Self::index_is_border_with_size(index, self.width, self.height)
    }

    #[inline]
    fn index_is_border_with_size(index: usize, width: usize, height: usize) -> bool {
        let is_left_border = index < height;
        let is_right_border = index >= (width - 1) * height;

        let remainder = index % height;
        let is_top_border = remainder == height - 1;
        let is_bottom_border = remainder == 0;

        // return true if any are true
        is_left_border || is_right_border || is_top_border || is_bottom_border
    }
}

enum FieldType {
    Horizontal,
    Vertical,
    Smoke,
}
