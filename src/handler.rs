use std::time::Duration;

use crate::{
    app::{App, AppState},
    fluid_sim::simulator::FluidSim,
    ui::{editor::editor_area_to_sim_coordinates, render_app},
    Result,
};
// use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{
        self, Event, KeyCode, KeyEventKind, MouseButton, MouseEvent, MouseEventKind,
    },
    layout::Rect,
};

/// Handle any events that have occurred since the last time the app was rendered.
pub fn handle_events(app: &mut App) -> Result<()> {
    // Ensure that the app only blocks for a period that allows the app to render at
    // approximately 60 FPS (this doesn't account for the time to render the frame, and will
    // also update the app immediately any time an event occurs)
    let timeout = Duration::from_secs_f32(1.0 / 60.0);
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key) => {
                if key.kind != KeyEventKind::Press {
                    return Ok(());
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        app.state = AppState::Quit;
                    }
                    KeyCode::Tab => {
                        app.fluid_sim.reset_velocity_and_smoke();
                        app.state = match app.state {
                            AppState::Running => AppState::Editing,
                            AppState::Editing => AppState::Running,
                            _ => AppState::Running,
                        }
                    }
                    _ => {}
                }
            }
            Event::Mouse(mouse_event) => handle_mouse_event(mouse_event, app),
            Event::Resize(width, height) => {
                let previous_state = app.state.clone();
                app.state = AppState::Resizing;

                // renders once to see the size of the simulation
                let area = Rect {
                    width,
                    height,
                    ..Default::default()
                };
                let mut empty_buffer = Buffer::empty(area);
                let new_sim_area = render_app(app, area, &mut empty_buffer);

                resize_sim(&mut app.fluid_sim, new_sim_area.width, new_sim_area.height);

                app.state = previous_state;
            }
            _ => {}
        }
    }
    Ok(())
}

/// resizes the sim
/// note: the sim height is double the render height to use half blocks
fn resize_sim(fluid_sim: &mut FluidSim, render_width: u16, render_height: u16) {
    let (width, height) = (render_width as usize, (render_height * 2) as usize);
    let (sim_width, sim_height) = fluid_sim.get_size();

    if width != sim_width || height != sim_height {
        fluid_sim.resize(width, height);
    }
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
