use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use super::simulator::FluidSim;

impl Widget for &FluidSim {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut max_pressure = f32::MIN;
        let mut min_pressure = f32::MAX;

        let pressure_grid = self.get_pressure_grid();
        let block_grid = self.get_block_grid();

        pressure_grid.iter().for_each(|column| {
            column.iter().for_each(|&pressure_value| {
                if pressure_value < min_pressure {
                    min_pressure = pressure_value;
                } else if pressure_value > max_pressure {
                    max_pressure = pressure_value
                }
            });
        });

        let (_, sim_height) = self.get_size();

        // assume that the rendering height is half the simulation height
        // this makes it so we can have double the "pixels" vertically
        // by using fg and bg colors

        for (x_index, x_pos) in (area.left()..area.right()).enumerate() {
            // reversing the y index since the sim origin is bottom left
            // and the rendering orign is top left
            let mut y_index = sim_height;
            for y_pos in area.top()..area.bottom() {
                y_index -= 1;

                render_cell(
                    x_index,
                    y_index,
                    x_pos,
                    y_pos,
                    block_grid,
                    pressure_grid,
                    max_pressure,
                    min_pressure,
                    '▄',
                    buf,
                    true,
                );
                y_index -= 1;

                render_cell(
                    x_index,
                    y_index,
                    x_pos,
                    y_pos,
                    block_grid,
                    pressure_grid,
                    max_pressure,
                    min_pressure,
                    '▀',
                    buf,
                    false,
                );
            }
        }
    }
}

fn render_cell(
    x_index: usize,
    y_index: usize,
    x_pos: u16,
    y_pos: u16,
    block_grid: &Vec<Vec<bool>>,
    pressure_grid: &Vec<Vec<f32>>,
    max_pressure: f32,
    min_pressure: f32,
    ch: char,
    buf: &mut Buffer,
    as_fg: bool,
) {
    let is_block = block_grid[x_index][y_index];
    let pressure = pressure_grid[x_index][y_index];

    let cell = buf.get_mut(x_pos, y_pos).set_char(ch);

    let color = if is_block {
        Color::DarkGray
    } else {
        get_linear_gradient(pressure, min_pressure, max_pressure)
    };

    if as_fg {
        cell.set_fg(color);
    } else {
        cell.set_bg(color);
    }
}

fn get_linear_gradient(value: f32, min: f32, max: f32) -> Color {
    let difference = max - min;

    let normalized = if difference == 0.0 {
        0.5
    } else {
        (value - min) / difference
    };

    let color_type = (normalized * 4.0).floor();

    let color_value = (normalized - color_type * 0.25) / 0.25;

    let (r, g, b) = match color_type as u8 {
        0 => (0.0, color_value, 1.0),
        1 => (0.0, 1.0, 1.0 - color_value),
        2 => (color_value, 1.0, 0.0),
        3 => (1.0, 1.0 - color_value, 0.0),
        _ => (1.0, 0.0, 0.0),
    };

    let (r, g, b) = ((255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8);

    Color::Rgb(r, g, b)
}
