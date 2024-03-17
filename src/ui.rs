use ratatui::{
    prelude::{Alignment, Frame, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Paragraph},
};
use crate::app::{App, Menu, Popup};
use crate::double_column_menu::double_column_menu::DoubleColumnMenu;
use crate::double_column_menu::render_helper_functions::render_info_dialog;

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
            app.cluster_menu.render(f, &outer_layout[0]);
        }
        Menu::Spawner => {
            app.spawner_menu.render(f, &outer_layout[0]);
        }
    }

    match &app.popup {
        Popup::Error(error_text) => {
            render_info_dialog(f, &error_text, Color::Red, 2);
        }
        _ => {}
    }

}

