use std::time::{Duration, Instant};

use ratatui::layout::Constraint::{self, *};
use ratatui::layout::{Flex, Layout};
use ratatui::prelude::{Buffer, Rect};
use ratatui::text::Text;
use ratatui::widgets::Widget;

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

    pub fn get_rendering_time(&self) -> Duration {
        self.rendering_duration
    }

    pub fn get_simulation_time(&self) -> Duration {
        self.simulation_step_duration
    }

    pub fn get_fps(&self) -> f32 {
        self.fps
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub(super) fn can_update(&self) -> bool {
        self.last_update.elapsed() > Duration::from_secs(1)
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
        let infos = info_as_text(self);
        let info_areas = Layout::vertical(vec![Length(1); infos.len()]).split(area);

        // set each info on the left and right
        infos
            .into_iter()
            .enumerate()
            .for_each(|(i, (info, info_len, name, name_len))| {
                let [left, right] = Layout::horizontal([info_len, name_len])
                    .flex(Flex::SpaceBetween)
                    .areas(info_areas[i]);
                info.render(left, buf);
                name.render(right, buf);
            });
    }
}

fn info_as_text(info: &AppInfo) -> Vec<(Text, Constraint, Text, Constraint)> {
    let simulation_time = info.get_simulation_time();
    let rendering_time = info.get_rendering_time();
    let fps = info.get_fps();
    let (width, height) = info.get_size();

    [
        (format_duration(simulation_time), "Simulation time"),
        (format_duration(rendering_time), "Rendering time"),
        (format!("{fps:.1} fps"), "Frames"),
        (format!("x: {width}, y: {height}"), "Grid Size"),
    ]
    .into_iter()
    .map(|(info, name)| {
        let info_length = info.len();
        let info_text = Text::raw(info);
        let name_text = Text::raw(name);
        (
            info_text,
            Length(info_length as u16),
            name_text,
            Length(name.len() as u16),
        )
    })
    .collect::<Vec<_>>()
}

fn format_duration(duration: Duration) -> String {
    format!(
        "{}.{} ms",
        duration.subsec_millis(),
        duration.subsec_micros() as u8
    )
}
