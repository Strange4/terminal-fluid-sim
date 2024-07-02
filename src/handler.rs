use std::time::Duration;

use crate::{
    app::{App, AppState},
    fluid_sim::simulator::FluidSim,
    ui::editor::editor_area_to_sim_coordinates,
};
use color_eyre::Result;
use crossterm::event::{
    self, Event, KeyCode, KeyEventKind, MouseButton, MouseEvent, MouseEventKind,
};
use ratatui::layout::Rect;

/// Handle any events that have occurred since the last time the app was rendered.
pub fn handle_events(app: &mut App) -> Result<()> {
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
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                app.state = AppState::Quit;
            } else if key.code == KeyCode::Tab {
                app.state = match app.state {
                    AppState::Running => AppState::Editing,
                    _ => AppState::Running,
                };
            };
        } else if let Event::Mouse(mouse_event) = event {
            handle_mouse_event(mouse_event, app);
        }
    }
    Ok(())
}

fn handle_mouse_event(mouse_event: MouseEvent, app: &mut App) {
    if app.state != AppState::Editing {
        return;
    }
    let sim_area = &app.editor_info.editor_area;

    // checking bounds
    if !is_within_bounding_box(mouse_event.column, mouse_event.row, sim_area) {
        app.editor_info.last_mouse_pos = None;
        return;
    }

    app.editor_info.last_mouse_pos = Some((mouse_event.column, mouse_event.row));

    if let MouseEventKind::Down(button) = mouse_event.kind {
        let (x, y) =
            editor_area_to_sim_coordinates((mouse_event.column, mouse_event.row), sim_area);

        match button {
            MouseButton::Left => {
                let down_index =
                    FluidSim::calculate_index_with_height((sim_area.height * 2) as usize, x, y);

                let down_is_block = app.fluid_sim.get_block_grid()[down_index];
                // set down block first
                if !down_is_block {
                    app.fluid_sim.set_block(x, y);
                } else {
                    app.fluid_sim.set_block(x, y + 1);
                }
            }
            MouseButton::Right => {
                let up_index =
                    FluidSim::calculate_index_with_height((sim_area.height * 2) as usize, x, y + 1);

                let up_is_block = app.fluid_sim.get_block_grid()[up_index];
                // unset top block first
                if up_is_block {
                    app.fluid_sim.unset_block(x, y + 1);
                } else {
                    app.fluid_sim.unset_block(x, y);
                }
            }
            _ => {}
        }
    }
}

fn is_within_bounding_box(x: u16, y: u16, bx: &Rect) -> bool {
    x >= bx.x && x < (bx.x + bx.width) && y >= bx.y && y < (bx.y + bx.height)
}
