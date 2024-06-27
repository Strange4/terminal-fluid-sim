use ratatui::{
    layout::{Constraint::*, Layout},
    text::Text,
    Frame,
};

use crate::app::App;

pub fn render_app(frame: &mut Frame, app: &mut App) {
    let main_area = frame.size();
    let [top, sim] = Layout::vertical([Length(1), Min(0)]).areas(main_area);
    let [title, fps] = Layout::horizontal([Min(0), Length(8)]).areas(top);
    let text = Text::from("fluid simulation. Press q to quit").centered();

    frame.render_widget(text, title);

    frame.render_widget(&mut app.fps_widget, fps);

    frame.render_widget(&app.fluid_sim, sim);
}
