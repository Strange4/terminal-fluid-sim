use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub background: Style,
    pub text: Style,
    pub borders: Style,
    pub title: Style,
    pub controls: Style,
    pub sim_blocks: Color,
    pub tab_text: Style,
    pub highlight_config: Style,
}

pub const THEME: Theme = Theme {
    background: Style::new().bg(Color::Black),
    text: Style::new().fg(Color::White),
    borders: Style::new().fg(Color::Gray),
    title: Style::new().add_modifier(Modifier::BOLD),
    controls: Style::new().bg(Color::White).fg(Color::Black),
    sim_blocks: Color::White,
    tab_text: Style::new().fg(Color::White),
    highlight_config: Style::new().fg(Color::Black).bg(Color::White),
};
