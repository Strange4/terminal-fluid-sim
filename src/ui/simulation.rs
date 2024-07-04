use std::time::Duration;

use layout::Flex;
use ratatui::{layout::Constraint::*, prelude::*, widgets::Block};

use crate::{app::AppInfo, fluid_sim::simulator::FluidSim};

use super::theme::THEME;

pub fn render_sim_info(info: &AppInfo, area: Rect, buf: &mut Buffer) {
    // setup the info
    let info_border = Block::bordered()
        .title("Sim Info")
        .title_style(THEME.text)
        .style(THEME.borders);
    let inner_info_area = info_border.inner(area);
    info_border.render(area, buf);

    // rendering the rest of the info
    let infos = info_as_text(info);

    let info_areas = Layout::vertical(vec![Length(1); infos.len()]).split(inner_info_area);

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

fn info_as_text(info: &AppInfo) -> Vec<(Text, Constraint, Text, Constraint)> {
    let simulation_time = info.get_simulation_time();
    let rendering_time = info.get_rendering_time();
    let fps = info.get_fps();
    let (width, height) = info.get_size();

    [
        (format_duration(simulation_time), "Simulation time"),
        (format_duration(rendering_time), "Rendering time"),
        (format!("{fps:.1} fps"), "Frames"),
        (format!("x: {width}, y: {height}"), "Grid Size"),
    ]
    .into_iter()
    .map(|(info, name)| {
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
    .collect::<Vec<_>>()
}

pub fn render_sim(sim: &mut FluidSim, area: Rect, buf: &mut Buffer) {
    // resize_sim(sim, area.width, area.height);

    sim.render(area, buf);
}

fn format_duration(duration: Duration) -> String {
    format!(
        "{}.{} ms",
        duration.subsec_millis(),
        duration.subsec_micros() as u8
    )
}
