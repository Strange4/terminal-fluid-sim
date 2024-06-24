//! # [Ratatui] `Colors_RGB` example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

// This example shows the full range of RGB colors that can be displayed in the terminal.
//
// Requires a terminal that supports 24-bit color (true color) and unicode.
//
// This example also demonstrates how implementing the Widget trait on a mutable reference
// allows the widget to update its state while it is being rendered. This allows the fps
// widget to update the fps calculation and the colors widget to update a cached version of
// the colors to render instead of recalculating them every frame.
//
// This is an alternative to using the `StatefulWidget` trait and a separate state struct. It
// is useful when the state is only used by the widget and doesn't need to be shared with
// other widgets.

mod fluid_sim;

use std::{
    io::stdout,
    panic,
    time::{Duration, Instant},
};

use color_eyre::{config::HookBuilder, eyre, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use fluid_sim::simulator::FluidSim;
use ratatui::prelude::*;

#[derive(Debug, Default)]
struct App {
    /// The current state of the app (running or quit)
    state: AppState,

    /// A widget that displays the current frames per second
    fps_widget: FpsWidget,

    fluid_sim: FluidSim,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum AppState {
    /// The app is running
    #[default]
    Running,

    /// The user has requested the app to quit
    Quit,
}

/// A widget that displays the current frames per second
#[derive(Debug)]
struct FpsWidget {
    /// The number of elapsed frames that have passed - used to calculate the fps
    frame_count: usize,

    /// The last instant that the fps was calculated
    last_instant: Instant,

    /// The current frames per second
    fps: Option<f32>,
}

fn main() -> Result<()> {
    install_error_hooks()?;
    let terminal = init_terminal()?;
    App::default().run(terminal)?;
    restore_terminal()?;
    Ok(())
}

impl App {
    /// Run the app
    ///
    /// This is the main event loop for the app.
    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.size()))?;
            self.handle_events()?;
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
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.state = AppState::Quit;
                };
            }
        }
        Ok(())
    }
}

/// Implement the Widget trait for &mut App so that it can be rendered
///
/// This is implemented on a mutable reference so that the app can update its state while it is
/// being rendered. This allows the fps widget to update the fps calculation and the colors widget
/// to update the colors to render.
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        #[allow(clippy::enum_glob_use)]
        use Constraint::*;
        let [top, colors] = Layout::vertical([Length(1), Min(0)]).areas(area);
        let [title, fps] = Layout::horizontal([Min(0), Length(8)]).areas(top);
        Text::from("colors_rgb example. Press q to quit")
            .centered()
            .render(title, buf);
        self.fps_widget.render(fps, buf);
        self.fluid_sim.render(colors, buf);
        // self.colors_widget.render(colors, buf);
    }
}

/// Default impl for `FpsWidget`
///
/// Manual impl is required because we need to initialize the `last_instant` field to the current
/// instant.
impl Default for FpsWidget {
    fn default() -> Self {
        Self {
            frame_count: 0,
            last_instant: Instant::now(),
            fps: None,
        }
    }
}

/// Widget impl for `FpsWidget`
///
/// This is implemented on a mutable reference so that we can update the frame count and fps
/// calculation while rendering.
impl Widget for &mut FpsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.calculate_fps();
        if let Some(fps) = self.fps {
            let text = format!("{fps:.1} fps");
            Text::from(text).render(area, buf);
        }
    }
}

impl FpsWidget {
    /// Update the fps calculation.
    ///
    /// This updates the fps once a second, but only if the widget has rendered at least 2 frames
    /// since the last calculation. This avoids noise in the fps calculation when rendering on slow
    /// machines that can't render at least 2 frames per second.
    #[allow(clippy::cast_precision_loss)]
    fn calculate_fps(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_instant.elapsed();
        if elapsed > Duration::from_secs(1) && self.frame_count > 2 {
            self.fps = Some(self.frame_count as f32 / elapsed.as_secs_f32());
            self.frame_count = 0;
            self.last_instant = Instant::now();
        }
    }
}

/// Install `color_eyre` panic and error hooks
///
/// The hooks restore the terminal to a usable state before printing the error message.
fn install_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info);
    }));
    Ok(())
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
