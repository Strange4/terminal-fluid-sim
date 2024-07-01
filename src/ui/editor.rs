use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget, Wrap},
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
