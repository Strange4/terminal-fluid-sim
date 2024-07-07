use ratatui::layout::Constraint::*;
use ratatui::prelude::*;

use crate::{
    app::{AppConfig, AppInfo},
    fluid_sim::simulator::FluidSim,
};

use super::render_border_with_title;

pub fn render_sim_info(info: &AppInfo, config: &mut AppConfig, area: Rect, buf: &mut Buffer) {
    let [up, down] = Layout::vertical([Fill(1), Fill(1)]).areas(area);

    let info_area = render_border_with_title("Sim Info", up, buf);
    let config_area = render_border_with_title("Settings", down, buf);

    info.render(info_area, buf);
    config.render(config_area, buf);
}

pub fn render_sim(sim: &mut FluidSim, area: Rect, buf: &mut Buffer) {
    sim.render(area, buf);
}
