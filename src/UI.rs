use ratatui::{
    layout::{Constraint::*, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::block,
    text::{Line, Span, Text},
    widgets::Block,
    Frame,
};

use crate::{app::App, fluid_sim::simulator::FluidSim, fps::FpsWidget};

pub fn render_app(frame: &mut Frame, app: &mut App) {
    let [main_area, controls_area] = Layout::vertical([Fill(1), Length(1)]).areas(frame.size());

    let [info_area, sim_area] = Layout::horizontal([Ratio(1, 5), Fill(1)]).areas(main_area);

    info_box_layout(frame, &mut app.fps_widget, info_area);
    sim_layout(frame, &app.fluid_sim, sim_area);
    controls_layout(frame, controls_area);
}

fn controls_layout(frame: &mut Frame, area: Rect) {
    let controls = [("q", "quit")];

    let spans: Vec<_> = controls
        .iter()
        .map(|(control, desc)| Span::raw(format!(" {control} / {desc} ")))
        .collect();

    let line = Line::from(spans).centered().style(THEME.controls);

    frame.render_widget(line, area);
}

fn info_box_layout(frame: &mut Frame, fps_widget: &mut FpsWidget, area: Rect) {
    let [title_area, info_area] = Layout::vertical([Length(3), Fill(1)]).areas(area);
    let title_border = Block::bordered().style(THEME.borders);
    let inner_title_area = title_border.inner(title_area);

    let title = Text::from("Fluid Simulator").centered().style(THEME.title);

    let info_border = Block::bordered().title("Sim Info").style(THEME.borders);

    let inner_info_area = info_border.inner(info_area);

    frame.render_widget(title_border, title_area);
    frame.render_widget(title, inner_title_area);
    frame.render_widget(info_border, info_area);
    frame.render_widget(fps_widget, inner_info_area);
}

fn sim_layout(frame: &mut Frame, sim: &FluidSim, area: Rect) {
    let block = Block::bordered().style(THEME.borders);

    let sim_area = block.inner(area);

    frame.render_widget(block, area);

    frame.render_widget(sim, sim_area);
}

struct Theme {
    background: Style,
    text: Style,
    borders: Style,
    title: Style,
    controls: Style,
}

const THEME: Theme = Theme {
    background: Style::new().bg(Color::Black),
    text: Style::new().fg(Color::White),
    borders: Style::new().fg(Color::Gray),
    title: Style::new()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC),
    controls: Style::new().bg(Color::White).fg(Color::Black),
};
