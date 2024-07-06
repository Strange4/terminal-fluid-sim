use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::Widget;

use crate::ui::{render_border_with_title, render_left_right_layout};

pub struct AppConfig {
    /// gravity of the simulation, set to 0 for no gravity
    gravity: f32,

    /// wind speed, must be above 0
    wind_speed: f32,

    /// some height percentage of the screen [0,1] inclusive
    smoke_size: f32,

    /// density of the sim
    density: f32,

    /// the current selection to be changed
    current_selection: u8,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            gravity: 0.0,
            wind_speed: 50.0,
            smoke_size: 0.25,
            density: 1000.0,
            current_selection: 0,
        }
    }
}

impl Widget for &AppConfig {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let gravity = self.gravity;
        let wind_speed = self.wind_speed;
        let smoke_size = self.smoke_size * 100.0; // this is a precentage
        let density = self.density;

        let infos = [
            (format!("{gravity:.1} m/s²"), "Gravity".to_string()),
            (format!("{wind_speed:.0} m/s"), "Wind Speed".to_string()),
            (format!("{smoke_size:.0} %"), "Smoke Size".to_string()),
            (format!("{density:.0}"), "Density".to_string()),
        ];

        render_left_right_layout(&infos, area, buf);
    }
}
