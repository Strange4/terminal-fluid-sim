use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;

use crate::{fluid_sim::simulator::FluidSim, fps::FpsWidget, ui::render_app};

#[derive(Debug, Default)]
pub struct App {
    /// The current state of the app (running or quit)
    state: AppState,

    /// A widget that displays the current frames per second
    pub fps_widget: FpsWidget,

    /// the actual sim
    pub fluid_sim: FluidSim,
}

#[derive(Debug, Default, PartialEq, Eq)]
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
        // set the terminal size to the whole terminal so we can render it at least once

        while self.is_running() {
            // make sure to handle the events
            self.handle_events()?;

            terminal.draw(|frame| {
                self.fluid_sim.next_step();
                render_app(frame, self);
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
            } else if let Event::Resize(width, height) = event {
                resize_sim_if_necessary(&mut self.fluid_sim, width, height)
            }
        }
        Ok(())
    }
}

fn resize_sim_if_necessary(fluid_sim: &mut FluidSim, width: u16, height: u16) {
    let (width, height) = (width as usize, height as usize);
    let (sim_width, sim_height) = fluid_sim.get_size();

    if width != sim_width || height != sim_height {
        fluid_sim.resize(width, height);
    }
}
