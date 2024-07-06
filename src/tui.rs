use std::{io::stdout, panic};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};

use crate::Result;

/// Install `color_eyre` panic and error hooks
///
/// The hooks restore the terminal to a usable state before printing the error message.
pub fn install_error_hooks() -> Result<()> {
    // let (panic, error) = HookBuilder::default().into_hooks();
    // let panic = panic.into_panic_hook();
    // let error = error.into_eyre_hook();
    // eyre::set_hook(Box::new(move |e| {
    //     let _ = restore_terminal();
    //     error(e)
    // }))?;
    let panic_hook = std::panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic_hook(info);
    }));
    Ok(())
}

pub fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

pub fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}
