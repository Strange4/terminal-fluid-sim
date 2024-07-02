use color_eyre::owo_colors::OwoColorize;
use crossterm::{
    event::{MouseButton, MouseEvent, MouseEventKind},
    style::Stylize,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{
    app::{App, AppState},
    fluid_sim::simulator::FluidSim,
};

use super::THEME;

pub fn render_editor_info(area: Rect, buf: &mut Buffer) {
    let info = r#"Use the mouse and keyboard to edit your craft!
        
(hint: controls at the bottom)"#;
    Paragraph::new(info)
        .style(THEME.text)
        .block(
            Block::bordered()
                .style(THEME.borders)
                .title("Editor Info")
                .title_style(THEME.text),
        )
        .wrap(Wrap { trim: false })
        .render(area, buf);
}

pub struct Editor<'a> {
    mouse_pos: Option<(u16, u16)>,
    blocks: &'a Vec<bool>,
}

impl Widget for Editor<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // the rendering height is half the sim height
        let height = (area.height * 2) as usize;

        for (x_index, x_pos) in (area.left()..area.right()).enumerate() {
            let mut y_index = height;
            for y_pos in area.top()..area.bottom() {
                y_index -= 1;
                let up_index = FluidSim::calculate_index_with_height(height, x_index, y_index);

                y_index -= 1;
                let down_index = FluidSim::calculate_index_with_height(height, x_index, y_index);

                // if there are not blocks the skip
                if !(self.blocks[up_index] || self.blocks[down_index]) {
                    continue;
                }

                // the top block is the foreground and the background is the bottom
                let cell = buf.get_mut(x_pos, y_pos).set_char('▀');

                let block_color = THEME.sim_blocks;

                if self.blocks[up_index] {
                    cell.set_fg(block_color);
                } else {
                    // set the color to the background to make it as if there was no block
                    cell.set_fg(THEME.background.bg.unwrap());
                }

                if self.blocks[down_index] {
                    cell.set_bg(block_color);
                }
            }
        }

        hover_mouse(self.mouse_pos, buf);
    }
}

pub fn render_editor(app: &mut App, area: Rect, buf: &mut Buffer) {
    // editor
    Editor {
        blocks: app.fluid_sim.get_block_grid(),
        mouse_pos: app.editor_info.last_mouse_pos,
    }
    .render(area, buf);
}

fn hover_mouse(mouse_pos: Option<(u16, u16)>, buf: &mut Buffer) {
    if let Some(mouse_pos) = mouse_pos {
        let (x, y) = (mouse_pos.0, mouse_pos.1);
        let cell = buf.get_mut(x, y).set_fg(THEME.sim_blocks);

        cell.modifier = Modifier::DIM;

        let ch = cell.symbol();

        // first set the up block then the down block
        match ch {
            " " => {
                cell.set_char('▀');
            }

            "▀" => {
                cell.set_bg(THEME.sim_blocks);
            }

            _ => {}
        }
    }
}
