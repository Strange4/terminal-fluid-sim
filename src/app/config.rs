use ratatui::layout::Constraint;
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget};

use crate::ui::THEME;

use terminal_fluid_sim::SimConfig;

#[derive(Default)]
pub struct AppConfig {
    /// the configuration of the sim
    config: SimConfig,

    /// the current selection to be changed
    current_selection: TableState,
}

impl AppConfig {
    #[inline]
    fn add_gravity(&mut self) {
        self.config.gravity += 0.1;
    }

    #[inline]
    fn reduce_gravity(&mut self) {
        self.config.gravity -= 0.1;
    }

    #[inline]
    fn add_wind_speed(&mut self) {
        self.config.wind_speed += 1.0;
    }

    #[inline]
    fn reduce_wind_speed(&mut self) {
        self.config.wind_speed = (self.config.wind_speed - 1.0).max(0.0);
    }

    #[inline]
    fn add_smoke_size(&mut self) {
        self.config.smoke_size = (self.config.smoke_size + 0.05).min(1.0);
    }

    #[inline]
    fn reduce_smoke_size(&mut self) {
        self.config.smoke_size = (self.config.smoke_size - 0.05).max(0.0);
    }

    #[inline]
    fn add_density(&mut self) {
        self.config.density += 25.0;
    }

    #[inline]
    fn reduce_density(&mut self) {
        self.config.density -= 25.0;
    }

    #[inline]
    pub fn get_gravity(&self) -> f32 {
        self.config.gravity
    }

    #[inline]
    pub fn get_wind_speed(&self) -> f32 {
        self.config.wind_speed
    }

    #[inline]
    pub fn get_smoke_size(&self) -> f32 {
        self.config.smoke_size
    }

    #[inline]
    pub fn get_density(&self) -> f32 {
        self.config.density
    }

    pub fn get_config(&self) -> SimConfig {
        self.config.clone()
    }

    pub fn reduce_selection(&mut self) {
        if let Some(selection) = self.current_selection.selected() {
            match selection {
                0 => self.reduce_gravity(),
                1 => self.reduce_wind_speed(),
                2 => self.reduce_smoke_size(),
                3 => self.reduce_density(),
                _ => {}
            }
        }
    }

    pub fn increase_selection(&mut self) {
        if let Some(selection) = self.current_selection.selected() {
            match selection {
                0 => self.add_gravity(),
                1 => self.add_wind_speed(),
                2 => self.add_smoke_size(),
                3 => self.add_density(),
                _ => {}
            }
        }
    }

    pub fn down_select(&mut self) {
        let i = match self.current_selection.selected() {
            Some(i) => {
                if i >= 3 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.current_selection.select(Some(i));
    }

    pub fn up_select(&mut self) {
        let i = match self.current_selection.selected() {
            Some(i) => {
                if i == 0 {
                    3
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.current_selection.select(Some(i));
    }
}

impl Widget for &mut AppConfig {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let gravity = self.config.gravity;
        let wind_speed = self.config.wind_speed;
        let smoke_size = self.config.smoke_size * 100.0; // this is a precentage
        let density = self.config.density;

        let rows = [
            Row::new(vec![format!("{gravity:.1} m/s²"), "Gravity".to_string()]),
            Row::new(vec![
                format!("{wind_speed:.0} m/s"),
                "Wind Speed".to_string(),
            ]),
            Row::new(vec![format!("{smoke_size:.0} %"), "Smoke Size".to_string()]),
            Row::new(vec![format!("{density:.0}"), "Density".to_string()]),
        ];

        let table = Table::new(rows, [Constraint::Fill(1), Constraint::Length(10)])
            .highlight_style(THEME.highlight_config)
            .highlight_symbol(">>")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(table, area, buf, &mut self.current_selection);
    }
}
