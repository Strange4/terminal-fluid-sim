use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;

use crate::{fluid_sim::simulator::FluidSim, ui::render_app};

#[derive(Default)]
pub struct App {
    /// The current state of the app (running or quit)
    state: AppState,

    /// the actual sim
    pub fluid_sim: FluidSim,

    pub info: AppInfo,
}

#[derive(Default, PartialEq, Eq)]
enum AppState {
    /// The app is running
    #[default]
    Running,

    /// The user has requested the app to quit
    Quit,
}

impl App {
    /// Run the app
    ///
    /// This is the main event loop for the app.
    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.is_running() {
            self.handle_events()?;

            terminal.draw(|frame| {
                self.info.frame_count += 1;
                if self.info.can_update() {
                    // measure the simulation time and save the info
                    let start = Instant::now();
                    self.fluid_sim.next_step();
                    let sim_duration = start.elapsed();

                    // measure rendering time
                    let start = Instant::now();
                    let area = frame.size();
                    let buffer = frame.buffer_mut();
                    render_app(self, area, buffer);
                    let render_duration = start.elapsed();

                    self.info.update(sim_duration, render_duration);
                } else {
                    self.fluid_sim.next_step();

                    let area = frame.size();
                    let buffer: &mut Buffer = frame.buffer_mut();
                    render_app(self, area, buffer);
                }
            })?;
        }
        Ok(())
    }

    const fn is_running(&self) -> bool {
        matches!(self.state, AppState::Running)
    }

    /// Handle any events that have occurred since the last time the app was rendered.
    ///
    /// Currently, this only handles the q key to quit the app.
    fn handle_events(&mut self) -> Result<()> {
        // Ensure that the app only blocks for a period that allows the app to render at
        // approximately 60 FPS (this doesn't account for the time to render the frame, and will
        // also update the app immediately any time an event occurs)
        let timeout = Duration::from_secs_f32(1.0 / 60.0);
        if event::poll(timeout)? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.state = AppState::Quit;
                };
            }
        }
        Ok(())
    }
}

pub struct AppInfo {
    rendering_duration: Duration,
    simulation_step_duration: Duration,
    last_update: Instant,
    frame_count: usize,
    fps: f32,
}

impl Default for AppInfo {
    fn default() -> Self {
        AppInfo {
            rendering_duration: Duration::default(),
            simulation_step_duration: Duration::default(),
            last_update: Instant::now(),
            frame_count: 0,
            fps: 0.0,
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

    fn can_update(&self) -> bool {
        self.last_update.elapsed() > Duration::from_secs(1)
    }

    fn calculate_fps(&mut self) {
        let elapsed = self.last_update.elapsed();
        self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
    }

    fn update(&mut self, simulation_time: Duration, rendering_time: Duration) {
        self.simulation_step_duration = simulation_time;
        self.rendering_duration = rendering_time;
        self.calculate_fps();
        self.last_update = Instant::now();
        self.frame_count = 0;
    }
}
