use ratatui::{prelude::*, widgets::Block};

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
    info.render(inner_info_area, buf);
}

pub fn render_sim(sim: &mut FluidSim, area: Rect, buf: &mut Buffer) {
    // resize_sim(sim, area.width, area.height);

    sim.render(area, buf);
}
