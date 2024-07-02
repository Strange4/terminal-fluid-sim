use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use crate::ui::THEME;

use super::simulator::FluidSim;

impl Widget for &FluidSim {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut max_pressure = f32::MIN;
        let mut min_pressure = f32::MAX;

        let pressure_grid = self.get_pressure_grid();
        let block_grid = self.get_block_grid();
        let smoke_grid = self.get_smoke_grid();

        pressure_grid.iter().for_each(|&pressure_value| {
            if pressure_value < min_pressure {
                min_pressure = pressure_value;
            } else if pressure_value > max_pressure {
                max_pressure = pressure_value
            }
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
                let index = self.calculate_index(x_index, y_index);

                // the top block is the foreground and the background is the bottom
                render_cell(
                    index,
                    x_pos,
                    y_pos,
                    block_grid,
                    smoke_grid,
                    pressure_grid,
                    max_pressure,
                    min_pressure,
                    buf,
                    true,
                );
                y_index -= 1;
                let index = self.calculate_index(x_index, y_index);

                render_cell(
                    index,
                    x_pos,
                    y_pos,
                    block_grid,
                    smoke_grid,
                    pressure_grid,
                    max_pressure,
                    min_pressure,
                    buf,
                    false,
                );
            }
        }
    }
}

fn render_cell(
    sim_index: usize,
    x_pos: u16,
    y_pos: u16,
    block_grid: &[bool],
    smoke_grid: &[f32],
    pressure_grid: &[f32],
    max_pressure: f32,
    min_pressure: f32,
    buf: &mut Buffer,
    as_fg: bool,
) {
    let is_block = block_grid[sim_index];

    let cell = buf.get_mut(x_pos, y_pos).set_char('▀');

    let color = if is_block {
        THEME.sim_blocks
    } else {
        let smoke = smoke_grid[sim_index];
        let pressure = pressure_grid[sim_index];
        get_color(smoke, pressure, min_pressure, max_pressure)
    };

    if as_fg {
        cell.set_fg(color);
    } else {
        cell.set_bg(color);
    }
}

fn get_linear_gradient(value: f32, min: f32, max: f32) -> (u8, u8, u8) {
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

    (r, g, b)
}

fn get_color(smoke: f32, pressure: f32, min_pressure: f32, max_pressure: f32) -> Color {
    let (r, g, b) = get_linear_gradient(pressure, min_pressure, max_pressure);
    let smoke_reducer = (255.0 * smoke) as u8;

    Color::Rgb(
        r.saturating_sub(smoke_reducer),
        g.saturating_sub(smoke_reducer),
        b.saturating_sub(smoke_reducer),
    )
}
