use std::time::Instant;

// use color_eyre::Result;
use ratatui::prelude::*;

use terminal_fluid_sim::FluidSim;

use crate::{handler::handle_events, ui::render_app, Result};

use super::{config::AppConfig, info::AppInfo};

pub struct App {
    /// The current state of the app (running or quit)
    pub state: AppState,

    /// the actual sim
    pub fluid_sim: FluidSim,

    /// relevant information about the app
    pub info: AppInfo,

    /// information needed by the editor
    pub editor_info: EditorInfo,

    /// configuration settings for the app
    pub config: AppConfig,
}

#[derive(Default, Clone, PartialEq)]
pub enum AppState {
    /// The app is running
    #[default]
    Running,

    /// Editing Mode
    Editing,

    // a non rendering state
    Resizing,

    /// The user has requested the app to quit
    Quit,
}

impl Default for App {
    fn default() -> Self {
        let config = AppConfig::default();
        Self {
            state: Default::default(),
            fluid_sim: FluidSim::new(10, 10, 1000.0),
            info: Default::default(),
            editor_info: Default::default(),
            config,
        }
    }
}

impl App {
    /// Run the app
    ///
    /// This is the main event loop for the app.
    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        let initial_size = terminal.size().unwrap();
        self.fluid_sim.resize(
            initial_size.width as usize,
            (initial_size.height * 2) as usize,
        );
        while self.is_running() {
            handle_events(self)?;
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
        self.info.add_frame();

        match self.state {
            AppState::Running => {
                if self.info.can_update() {
                    self.measure_and_update(frame);
                } else {
                    self.fluid_sim.next_step();
                    self.editor_info.editor_area =
                        render_app(self, frame.size(), frame.buffer_mut());
                }
            }
            AppState::Editing => {
                self.editor_info.editor_area = render_app(self, frame.size(), frame.buffer_mut());
            }
            _ => {}
        }
    }

    fn measure_and_update(&mut self, frame: &mut Frame) {
        // measure the simulation time and save the info
        let start = Instant::now();
        self.fluid_sim.next_step();
        let sim_duration = start.elapsed();

        // measure rendering time
        let start = Instant::now();
        self.editor_info.editor_area = render_app(self, frame.size(), frame.buffer_mut());
        let render_duration = start.elapsed();

        let (width, height) = self.fluid_sim.get_size();
        self.info
            .update(sim_duration, render_duration, width, height);
    }
}

#[derive(Default)]
pub struct EditorInfo {
    pub last_mouse_pos: Option<(u16, u16)>,
    pub editor_area: Rect,
}
