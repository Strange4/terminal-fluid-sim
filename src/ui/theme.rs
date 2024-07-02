use ratatui::style::{Color, Modifier, Style, Stylize};

pub struct Theme {
    pub background: Style,
    pub text: Style,
    pub borders: Style,
    pub title: Style,
    pub controls: Style,
    pub sim_blocks: Color,
    pub tab_text: Style,
}

pub const THEME: Theme = Theme {
    background: Style::new().bg(Color::Black),
    text: Style::new().fg(Color::White),
    borders: Style::new().fg(Color::Gray),
    title: Style::new().add_modifier(Modifier::BOLD),
    controls: Style::new().bg(Color::White).fg(Color::Black),
    sim_blocks: Color::Red,
    tab_text: Style::new().fg(Color::White),
};
