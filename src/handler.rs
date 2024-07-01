use std::time::Duration;

use crate::app::AppState;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

/// Handle any events that have occurred since the last time the app was rendered.
///
/// Currently, this only handles the q key to quit the app.
pub fn handle_events(app_state: &mut AppState) -> Result<()> {
    // Ensure that the app only blocks for a period that allows the app to render at
    // approximately 60 FPS (this doesn't account for the time to render the frame, and will
    // also update the app immediately any time an event occurs)
    let timeout = Duration::from_secs_f32(1.0 / 60.0);
    if event::poll(timeout)? {
        let event = event::read()?;
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }
            if key.code == KeyCode::Char('q') {
                *app_state = AppState::Quit;
            } else if key.code == KeyCode::Tab {
                *app_state = match app_state {
                    AppState::Running => AppState::Editing,
                    _ => AppState::Running,
                };
            };
        }
    }
    Ok(())
}
