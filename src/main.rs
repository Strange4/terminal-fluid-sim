mod app;
mod handler;
mod tui;
mod ui;

use app::App;
// use color_eyre::Result;
use tui::*;

/// error for the app
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    install_error_hooks()?;
    let terminal = init_terminal()?;
    App::default().run(terminal)?;
    restore_terminal()?;

    Ok(())
}
