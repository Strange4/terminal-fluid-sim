use std::{str::Chars, string, time::Instant};

use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

use super::simulator::FluidSim;

impl Widget for &mut FluidSim {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.width != area.width as usize || self.height != area.height as usize {
            self.resize(area.width as usize, area.height as usize)
        }
        self.step_through(self.last_instant.elapsed());

        let mut max_pressure = f32::MIN;
        let mut min_pressure = f32::MAX;

        self.pressure_grid.iter().for_each(|column| {
            column.iter().for_each(|&pressure_value| {
                if pressure_value < min_pressure {
                    min_pressure = pressure_value;
                } else if pressure_value > max_pressure {
                    max_pressure = pressure_value
                }
            });
        });
        let max_y = self.pressure_grid[0].len() - 1;
        for (x_index, x_pos) in (area.left()..area.right()).enumerate() {
            for (y_index, y_pos) in (area.top()..area.bottom()).enumerate() {
                let is_block = self.block_grid[x_index][max_y - y_index];
                let pressure = self.pressure_grid[x_index][max_y - y_index];
                if is_block {
                    buf.get_mut(x_pos, y_pos).set_char('█').set_fg(Color::White);
                } else if pressure != 0.0 {
                    let (ch, color) = get_linear_gradient(pressure, min_pressure, max_pressure);
                    buf.get_mut(x_pos, y_pos).set_char(ch).set_fg(color);
                }
            }
        }
        self.last_instant = Instant::now()
    }
}

// gets the linear gradient of a value from 0..1 inclusive where 1 is the brightest value
fn get_linear_gradient(value: f32, min: f32, max: f32) -> (char, Color) {
    // const CHARACTER_GRADIENT: &[u8] =
    //     r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#.as_bytes();
    // const LENGTH: usize = CHARACTER_GRADIENT.len();

    let value = value.max(min).min(max - 1e-4);
    let difference = max - min;

    let normalized = if difference == 0.0 {
        0.5
    } else {
        (value - min) / difference
    };

    let color_type = (normalized / 0.25).floor();

    let color_value = (normalized - color_type * 0.25) / 0.25;

    let (r, g, b) = match color_type as u8 {
        0 => (0.0, color_value, 1.0),
        1 => (0.0, 1.0, 1.0 - color_value),
        2 => (color_value, 1.0, 0.0),
        3 => (1.0, 1.0 - color_value, 0.0),
        _ => (1.0, 1.0, 1.0),
    };

    let (r, g, b) = ((255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8);

    let color = Color::Rgb(r, g, b);

    // let character_index = ((LENGTH - 1) as f32 * (1.0 - normalized)).floor() as usize;
    // (CHARACTER_GRADIENT[character_index] as char, color)
    ('█', color)
}
