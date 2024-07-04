use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    widgets::{Block, Paragraph, Widget, Wrap},
};

use crate::{app::App, fluid_sim::simulator::FluidSim};

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
                let cell = buf.get_mut(x_pos, y_pos).set_char('▄');

                let block_color = THEME.sim_blocks;

                if self.blocks[up_index] {
                    cell.set_bg(block_color);
                } else {
                    // set the color to the background to make it as if there was no block
                    // cell.set_fg(THEME.background.bg.unwrap());
                }

                if self.blocks[down_index] {
                    cell.set_fg(block_color);
                } else {
                    cell.set_fg(THEME.background.bg.unwrap());
                }
            }
        }

        if let Some(mouse_pos) = self.mouse_pos {
            // get x and y in simulator coordinates
            let (x, y) = editor_area_to_sim_coordinates(mouse_pos, &area);
            let down_index = FluidSim::calculate_index_with_height(height, x, y);
            let down_is_block = self.blocks[down_index];
            hover_mouse(mouse_pos, down_is_block, buf);
        }
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

fn hover_mouse(mouse_pos: (u16, u16), down_is_block: bool, buf: &mut Buffer) {
    let (x, y) = (mouse_pos.0, mouse_pos.1);

    let cell = buf.get_mut(x, y).set_fg(THEME.sim_blocks);
    cell.modifier = Modifier::DIM;

    if down_is_block {
        cell.set_char('▀').set_bg(THEME.sim_blocks);
    } else {
        cell.set_char('▄');
    }
}

/// returns the block that the mouse coordinate is pointing to
/// in simulation coordinates according to the area of the editor
/// returns the BOTTOM block
/// ex:
///
/// x is the position of the mouse starting from top left as the origin
/// EO is the origin of the editor increasing in y towards the bottom
/// SO is the origin of the simulation increasing in y towards the top
///
///       _________________
///      |     (EO)_______|
///      |        |      ||
///      |        |  x   ||
///      |    (SO)‾‾‾‾‾‾‾ |
///      ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
pub fn editor_area_to_sim_coordinates(
    mouse_coordinates: (u16, u16),
    editor_area: &Rect,
) -> (usize, usize) {
    (
        (mouse_coordinates.0 - editor_area.x) as usize,
        2 * (editor_area.height - mouse_coordinates.1) as usize,
    )
}
