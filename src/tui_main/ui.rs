use ratatui::{
    prelude::{Alignment, Frame, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Paragraph},
};
use crate::tui_main::app::{App, Menu};
use crate::starter::state::State;

pub fn render(app: &mut App, f: &mut Frame) {

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // make a info text at the bottom
    f.render_widget(
        Paragraph::new("Press Ctrl-C` or `q` to stop running.")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center),
        outer_layout[1],
    );

    match app.menu {
        Menu::Cluster => {
            app.cluster_state.render(f, &outer_layout[0]);
        }
        Menu::Spawner => {
            app.spawner_state.render(f, &outer_layout[0]);
        }
    }

}

