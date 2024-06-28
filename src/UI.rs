use layout::Flex;
use ratatui::{layout::Constraint::*, prelude::*, widgets::Block};

use crate::{
    app::{App, AppInfo},
    fluid_sim::simulator::FluidSim,
    fps::FpsWidget,
};

pub fn render_app(app: &mut App, buf: &mut Buffer, area: Rect) {
    // make the bottom controls have height of 1 and the fill the rest
    let [main_area, controls_area] = Layout::vertical([Fill(1), Length(1)]).areas(area);

    // make the info area take a minimu of 10 of the screen and fill the rest
    let [info_area, sim_area] = Layout::horizontal([Length(30), Fill(1)]).areas(main_area);

    // background color
    Block::new().style(THEME.background).render(main_area, buf);

    info_box_layout(&app.info, &mut app.fps_widget, info_area, buf);
    controls_layout(controls_area, buf);
    sim_layout(&mut app.fluid_sim, sim_area, buf);
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

fn info_box_layout(info: &AppInfo, fps_widget: &mut FpsWidget, area: Rect, buf: &mut Buffer) {
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

    // setup the info
    let info_border = Block::bordered()
        .title("Sim Info")
        .title_style(THEME.text)
        .style(THEME.borders);
    let inner_info_area = info_border.inner(info_area);
    info_border.render(info_area, buf);

    // rendering fps
    let [fps_area, info_area] = Layout::vertical([Length(1), Fill(1)]).areas(inner_info_area);
    fps_widget.render(fps_area, buf);

    // rendering the rest of the info
    let infos = [
        (info.get_simulation_time(), "Simulation Time"),
        (info.get_rendering_time(), "Rendering time"),
    ]
    .into_iter()
    .map(|(info, name)| {
        let info = format!(
            " {}.{} ms ",
            info.subsec_millis(),
            info.subsec_micros() as u8
        );
        let info_length = info.len();
        let info_text = Text::raw(info);
        let name_text = Text::raw(name);
        (
            info_text,
            Length(info_length as u16),
            name_text,
            Length(name.len() as u16),
        )
    })
    .collect::<Vec<_>>();

    let info_areas = Layout::vertical(vec![Length(1); infos.len()]).split(info_area);

    infos
        .into_iter()
        .enumerate()
        .for_each(|(i, (info, info_len, name, name_len))| {
            let [left, right] = Layout::horizontal([info_len, name_len])
                .flex(Flex::SpaceBetween)
                .areas(info_areas[i]);
            info.render(left, buf);
            name.render(right, buf);
        });
}

fn sim_layout(sim: &mut FluidSim, area: Rect, buf: &mut Buffer) {
    let border = Block::bordered().style(THEME.borders);
    let sim_area = border.inner(area);
    border.render(area, buf);

    resize_sim(sim, sim_area.width, sim_area.height);

    sim.render(sim_area, buf);
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
    title: Style::new().add_modifier(Modifier::BOLD),
    controls: Style::new().bg(Color::White).fg(Color::Black),
};
