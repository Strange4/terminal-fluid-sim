use ratatui::{layout::Constraint::*, prelude::*, widgets::Block};

use crate::app::{App, AppState};

use super::{
    editor::render_editor_info,
    simulation::{render_sim, render_sim_info},
    theme::THEME,
};

pub fn render_app(app: &mut App, area: Rect, buf: &mut Buffer) {
    let [info_area, sim_area] = set_layout(area, buf);

    match app.state {
        AppState::Running => {
            render_sim_info(&app.info, info_area, buf);
            render_sim(&mut app.fluid_sim, sim_area, buf);
        }
        AppState::Editing => {
            render_editor_info(info_area, buf);
        }
        _ => {}
    }
}

pub fn set_layout(area: Rect, buf: &mut Buffer) -> [Rect; 2] {
    // make the bottom controls have height of 1 and the fill the rest
    let [main_area, controls_area] = Layout::vertical([Fill(1), Length(1)]).areas(area);

    let [left_area, sim_area] = Layout::horizontal([Length(30), Fill(1)]).areas(main_area);

    // background color
    Block::new().style(THEME.background).render(main_area, buf);

    let info_area = render_title(left_area, buf);

    controls_layout(controls_area, buf);

    [info_area, sim_area]
}

fn render_title(area: Rect, buf: &mut Buffer) -> Rect {
    // the title area should be legnth 3 since it has a border
    let [title_area, info_area] = Layout::vertical([Length(3), Fill(1)]).areas(area);

    // setup the title
    let title_border = Block::bordered().border_style(THEME.borders);
    let inner_title_area = title_border.inner(title_area);
    Text::from("Fluid Simulator")
        .centered()
        .style(THEME.title)
        .render(inner_title_area, buf);

    title_border.render(title_area, buf);
    info_area
}

fn controls_layout(area: Rect, buf: &mut Buffer) {
    let controls = [("q", "quit")];

    let spans: Vec<_> = controls
        .iter()
        .map(|(control, desc)| Span::raw(format!(" {control} / {desc} ")))
        .collect();

    Line::from(spans)
        .centered()
        .style(THEME.controls)
        .render(area, buf);
}
