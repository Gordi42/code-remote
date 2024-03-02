use ratatui::{
    prelude::{Alignment, Frame, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui::{prelude::*, widgets::*};

use crate::tui_main::app::{App, Focus, Menu};

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
        Paragraph::new("Press `Esc`, `Ctrl-C` or `q` to stop running.")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center),
        outer_layout[1],
    );

    // Create a layout for the frame with two vertical sections.
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Constraint::Percentage(30),
                     Constraint::Percentage(70),
        ])
        .split(outer_layout[0]);

    match app.menu {
        Menu::Cluster => {
            // Render the right section.
            render_cluster_info(app, f, &layout[1]);

            // Render the left section.
            render_cluster_list(app, f, &layout[0]);
        }
        Menu::Spawner => {
            // Render the right section.
            render_spawner_info(app, f, &layout[1]);
            // Render the right section.
            render_spawner_list(app, f, &layout[0]);
        }
    }

}


pub fn render_cluster_list(app: &mut App, f: &mut Frame, area: &Rect) {
    // select the border color based on the focus
    let border_color = match app.focus {
        Focus::List => Color::Blue,
        _ => Color::White,};
    // create a list with the cluster names
    let mut state = ListState::default();
    let counter: usize = app.cluster_state.counter as usize;
    state.select(Some(counter));
    let items = app.cluster_state.get_cluster_names();
    let list = List::new(items)
        .block(Block::default().title("Clusters:").borders(Borders::ALL)
        .border_type(BorderType::Rounded).fg(border_color))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD)
                         .bg(Color::Blue).fg(Color::Black))
        .highlight_symbol(" > ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    // render the list
    f.render_stateful_widget(list, *area, &mut state);
}

pub fn render_spawner_list(app: &mut App, f: &mut Frame, area: &Rect) {
    // select the border color based on the focus
    let border_color = match app.focus {
        Focus::List => Color::Blue,
        _ => Color::White,};
    // create a list with the cluster names
    let mut state = ListState::default();
    let counter = app.spawner_state.as_ref().map_or(0, |state| state.counter as usize);
    state.select(Some(counter));
    let items = app.spawner_state.as_ref().map_or(
        vec!["No spawners".to_string()],
        |state| state.get_spawner_names());
    let list = List::new(items)
        .block(Block::default().title("Spawners:").borders(Borders::ALL)
        .border_type(BorderType::Rounded).fg(border_color))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD)
                         .bg(Color::Blue).fg(Color::Black))
        .highlight_symbol(" > ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    // render the list
    f.render_stateful_widget(list, *area, &mut state);
}

pub fn render_cluster_info(app: &mut App, f: &mut Frame, area: &Rect) {
    // select the border color based on the focus
    let border_color = match app.focus {
        Focus::Info => Color::Blue,
        _ => Color::White,};

    let cluster = app.cluster_state.get_cluster().unwrap();
    let text = format!(
        "Name: {}\n\
        Host: {}\n\
        User: {}\n\
        IdentityFile: {}
        ",
        cluster.name, cluster.host, cluster.user, cluster.identity_file);
    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().title("Info:").borders(Borders::ALL)
            .border_type(BorderType::Rounded).fg(border_color))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left),
        *area);
}

pub fn render_spawner_info(app: &mut App, f: &mut Frame, area: &Rect) {
    // select the border color based on the focus
    let border_color = match app.focus {
        Focus::Info => Color::Blue,
        _ => Color::White,};

    let spawner = app.spawner_state.as_ref().unwrap().get_spawner().unwrap();
    let text = format!(
        "Preset Name: {}\n\
        Account: {}\n\
        Partition: {}\n\
        Time: {}\n\
        Workdir: {}\n\
        Other: {}
        ",
        spawner.preset_name, spawner.account, spawner.partition,
        spawner.time, spawner.working_directory, spawner.other_options);
    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().title("Info:").borders(Borders::ALL)
            .border_type(BorderType::Rounded).fg(border_color))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left),
        *area);
}
        
