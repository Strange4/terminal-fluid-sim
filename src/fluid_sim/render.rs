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

        for (x_index, x_pos) in (area.left()..area.right()).enumerate() {
            for (mut y_index, y_pos) in (area.top()..area.bottom()).enumerate() {
                // reversing the y index
                // (0,0) is bottom left in simulation but top left on buffer to render
                y_index = sim_height - 1 - y_index;

                let is_block = block_grid[x_index][y_index];
                let pressure = pressure_grid[x_index][y_index];
                if is_block {
                    buf.get_mut(x_pos, y_pos).set_char('█').set_fg(Color::Black);
                } else if pressure != 0.0 {
                    let (ch, color) = get_linear_gradient(pressure, min_pressure, max_pressure);
                    buf.get_mut(x_pos, y_pos).set_char(ch).set_fg(color);
                }
            }
        }
    }
}

fn get_linear_gradient(value: f32, min: f32, max: f32) -> (char, Color) {
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

    let color = Color::Rgb(r, g, b);
    ('█', color)
}
