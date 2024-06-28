use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;

use crate::{fluid_sim::simulator::FluidSim, fps::FpsWidget, ui::render_app};

#[derive(Default)]
pub struct App {
    /// The current state of the app (running or quit)
    state: AppState,

    /// A widget that displays the current frames per second
    pub fps_widget: FpsWidget,

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
                // measure the simulation time and save the info
                let start = Instant::now();
                self.fluid_sim.next_step();
                self.info.simulation_step_duration = start.elapsed();

                let start = Instant::now();
                let area = frame.size();
                let buffer = frame.buffer_mut();
                render_app(self, buffer, area);
                self.info.rendering_duration = start.elapsed();
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

#[derive(Default)]
pub struct AppInfo {
    rendering_duration: Duration,
    simulation_step_duration: Duration,
}

impl AppInfo {
    pub fn get_rendering_time(&self) -> Duration {
        self.rendering_duration
    }

    pub fn get_simulation_time(&self) -> Duration {
        self.simulation_step_duration
    }
}
