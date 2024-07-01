use std::time::{Duration, Instant};

use color_eyre::Result;
use ratatui::prelude::*;

use crate::{fluid_sim::simulator::FluidSim, handler::handle_events, ui::render_app};

#[derive(Default)]
pub struct App {
    /// The current state of the app (running or quit)
    pub state: AppState,

    /// the actual sim
    pub fluid_sim: FluidSim,

    pub info: AppInfo,
}

#[derive(Default, PartialEq, Eq)]
pub enum AppState {
    /// The app is running
    #[default]
    Running,

    /// Editing Mode
    Editing,

    /// The user has requested the app to quit
    Quit,
}

impl App {
    /// Run the app
    ///
    /// This is the main event loop for the app.
    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.is_running() {
            handle_events(&mut self.state)?;

            terminal.draw(|frame| {
                self.update(frame);
            })?;
        }
        Ok(())
    }

    const fn is_running(&self) -> bool {
        matches!(self.state, AppState::Running | AppState::Editing)
    }

    fn update(&mut self, frame: &mut Frame) {
        self.info.frame_count += 1;

        match self.state {
            AppState::Running => {
                if self.info.can_update() {
                    // measure the simulation time and save the info
                    let start = Instant::now();
                    self.fluid_sim.next_step();
                    let sim_duration = start.elapsed();

                    // measure rendering time
                    let start = Instant::now();
                    render_app(self, frame.size(), frame.buffer_mut());
                    let render_duration = start.elapsed();

                    let (width, height) = self.fluid_sim.get_size();
                    self.info
                        .update(sim_duration, render_duration, width, height);
                } else {
                    self.fluid_sim.next_step();
                    render_app(self, frame.size(), frame.buffer_mut());
                }
            }
            AppState::Editing => {
                render_app(self, frame.size(), frame.buffer_mut());
            }
            _ => {}
        }
    }
}

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

    fn can_update(&self) -> bool {
        self.last_update.elapsed() > Duration::from_secs(1)
    }

    fn calculate_fps(&mut self) {
        let elapsed = self.last_update.elapsed();
        self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
    }

    fn update(
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
