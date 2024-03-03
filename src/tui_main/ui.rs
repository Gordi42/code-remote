use ratatui::{
    prelude::{Alignment, Frame, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui::{prelude::*, widgets::*, layout::Flex};

use crate::tui_main::app::{App, Focus, Menu, InputMode};

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
            if app.cluster_state.is_new_cluster() {
                render_create_new_dialog(f, &layout[1]);
            }
            else {
                // Render the right section.
                render_cluster_info(app, f, &layout[1]);
            }

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

    match app.input_mode {
        InputMode::Editing => {
            render_text_area(app, f);
        }
        InputMode::Remove => {
            render_remove_dialog(f);
        }
        _ => {}
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
    // Draw a border around the area.
    let mut border = Block::default()
        .title("Info: ").borders(Borders::ALL)
        .border_type(BorderType::Rounded);
        
    // modify the border based on the focus
    border = match app.focus {
        Focus::List => {
            border
                .title(block::Title::from("<tab> to toggle")
                       .alignment(Alignment::Right))}
        Focus::Info => {
            border.fg(Color::Blue)
        }
    };

    // create a layout for the inner area
    let layout = Layout::default().
        direction(Direction::Horizontal).constraints(
            [Constraint::Length(15),Constraint::Min(8)].as_ref())
        .split(border.inner(*area));


    let highlight_style = match app.focus {
        Focus::Info => Style::default().add_modifier(Modifier::BOLD)
                                       .bg(Color::Blue).fg(Color::Black),
        _           => Style::default().fg(Color::White),};

    let mut state = ListState::default();
    state.select(Some(app.cluster_state.info_counter as usize));

    let cluster = app.cluster_state.get_cluster().unwrap();

    let entry_names = vec!["Name: ", "Host: ", "User: ", "IdentityFile: "];
    let entry_values = vec![
        cluster.name.clone(),
        cluster.host.clone(),
        cluster.user.clone(),
        cluster.identity_file.clone(),
    ];

    let entry_names = List::new(entry_names)
        .style(Style::default().fg(Color::White))
        .highlight_style(highlight_style)
        .highlight_symbol(" ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let entry_values = List::new(entry_values)
        .style(Style::default().fg(Color::White))
        .highlight_style(highlight_style)
        .highlight_symbol(" ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    // render the list
    f.render_widget(border, *area);
    f.render_stateful_widget(entry_names, layout[0], &mut state);
    f.render_stateful_widget(entry_values, layout[1], &mut state);

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

pub fn render_text_area(app: &mut App, f: &mut Frame) {
    let window_width = f.size().width;
    let text_area_width = (0.8 * (window_width as f32)) as u16;

    let rect = centered_rect(f.size(), text_area_width, 3);

    f.render_widget(Clear, rect); //this clears out the background

    app.text_area.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("Edit entry: "),
    );

    app.text_area.set_style(
        Style::default().fg(Color::Green));

    f.render_widget(app.text_area.widget(), rect);
}

pub fn render_remove_dialog(f: &mut Frame) {
    let window_width = f.size().width;
    let text_area_width = (0.8 * (window_width as f32)) as u16;

    let rect = centered_rect(f.size(), text_area_width, 3);
    f.render_widget(Clear, rect); //this clears out the background
    let text = "Are you sure you want to remove this entry? (y/n)";
    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL)
            .border_type(BorderType::Rounded))
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center),
        rect);
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([width]).flex(Flex::Center);
    let vertical = Layout::vertical([height]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn render_create_new_dialog(f: &mut Frame, area: &Rect) {
    let text = "Press `Enter` to create a new entry.";
    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL)
            .border_type(BorderType::Rounded))
            .style(Style::default())
            .alignment(Alignment::Center),
        *area);
}
