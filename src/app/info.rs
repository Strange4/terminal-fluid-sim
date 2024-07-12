use std::time::{Duration, Instant};

use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::Widget;

use crate::ui::render_left_right_text;

pub struct AppInfo {
    rendering_duration: Duration,
    simulation_step_duration: Duration,
    last_update: Instant,
    frame_count: usize,
    fps: f32,
    width: usize,
    height: usize,
}

impl Default for AppInfo {
    fn default() -> Self {
        AppInfo {
            rendering_duration: Duration::default(),
            simulation_step_duration: Duration::default(),
            last_update: Instant::now(),
            frame_count: 0,
            fps: 0.0,
            width: 0,
            height: 0,
        }
    }
}

impl AppInfo {
    pub(super) fn add_frame(&mut self) {
        self.frame_count += 1;
    }

    pub(super) fn can_update(&self) -> bool {
        self.last_update.elapsed() > Duration::from_secs_f32(0.5)
    }

    fn calculate_fps(&mut self) {
        let elapsed = self.last_update.elapsed();
        self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
    }

    pub(super) fn update(
        &mut self,
        simulation_time: Duration,
        rendering_time: Duration,
        width: usize,
        height: usize,
    ) {
        self.simulation_step_duration = simulation_time;
        self.rendering_duration = rendering_time;
        self.calculate_fps();
        self.last_update = Instant::now();
        self.frame_count = 0;
        self.width = width;
        self.height = height;
    }
}

impl Widget for &AppInfo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = (self.width, self.height);

        let infos = [
            (
                format_duration(self.simulation_step_duration),
                "Simulation time".to_string(),
            ),
            (
                format_duration(self.rendering_duration),
                "Rendering time".to_string(),
            ),
            (format!("{:.1} fps", self.fps), "Frames".to_string()),
            (format!("x: {width}, y: {height}"), "Grid Size".to_string()),
        ];
        render_left_right_text(&infos, area, buf);
    }
}

fn format_duration(duration: Duration) -> String {
    format!(
        "{}.{} ms",
        duration.subsec_millis(),
        duration.subsec_micros() as u8
    )
}
