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
            render_cluster_menu(app, f, &outer_layout[0]);
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

// =======================================================================
//  UI Helper Functions
// =======================================================================

/// Render the borders of a widget and return the area inside the borders.
pub fn render_border(f: &mut Frame, area: &Rect, title: &str,
                     is_focused: bool) -> Rect {
    let mut block = Block::default().title(title).borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    if is_focused {
        block = block.fg(Color::Blue);
    }
    // add a <tab> to the title if the widget is not focused
    let block = match is_focused {
        true => block,
        false => block.title(block::Title::from("<tab>")
                            .alignment(Alignment::Right)),
    };
    f.render_widget(block.clone(), *area);
    block.inner(*area)
}

pub fn render_list(f: &mut Frame, area: &Rect, items: Vec<String>,
                   enable_highlight: bool, counter: usize, 
                   highlight_symbol: &str) {
    let highlight_style = match enable_highlight {
        true => Style::default().add_modifier(Modifier::BOLD)
            .bg(Color::Blue).fg(Color::Black),
        false => Style::default(),
    };
    // create the list state
    let mut state = ListState::default();
    state.select(Some(counter));
    // create the list
    let list = List::new(items)
        .style(Style::default())
        .highlight_style(highlight_style)
        .highlight_symbol(highlight_symbol)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    // render the list
    f.render_stateful_widget(list, *area, &mut state);
}

pub fn horizontal_split(area: &Rect, percentage: u16) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(percentage),
                Constraint::Percentage(100 - percentage),
            ]
            .as_ref(),
        )
        .split(*area);
    layout.to_vec()
}

pub fn horizontal_split_fixed(area: &Rect, fixed: u16) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(fixed),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(*area);
    layout.to_vec()
}

// =======================================================================
//  Render Cluster Menu
// =======================================================================

pub fn render_cluster_menu(app: &mut App, f: &mut Frame, area: &Rect) {
    let layout = horizontal_split(area, 30);
    // Render the list section.
    render_cluster_list(app, f, &layout[0]);

    // Render the info section.
    if app.cluster_state.is_new_cluster() {
        render_create_new_dialog(f, &layout[1]);
    } else {
        render_cluster_info(app, f, &layout[1]);
    }
}

pub fn render_cluster_list(app: &mut App, f: &mut Frame, area: &Rect) {
    let inner_area = render_border(
        f, area, "Clusters: ", app.focus == Focus::List);
    // create a list with the cluster names
    render_list(f, &inner_area, 
                app.cluster_state.get_cluster_names(), true, 
                app.cluster_state.counter as usize, " > ");
}

pub fn render_cluster_info(app: &mut App, f: &mut Frame, area: &Rect) {
    let inner_area = render_border(
        f, area, "Info: ", app.focus == Focus::Info);

    // create a layout for the inner area
    let layout = horizontal_split_fixed(&inner_area, 15);

    let enable_highlight = app.focus == Focus::Info;

    let cluster = app.cluster_state.get_cluster().unwrap();

    render_list(f, &layout[0], cluster.get_entry_names(), enable_highlight, 
                app.cluster_state.info_counter as usize, "  ");
    render_list(f, &layout[1], cluster.get_entry_values(), enable_highlight, 
                app.cluster_state.info_counter as usize, "  ");

}

// =======================================================================
//  Render Spawner Menu
// =======================================================================

pub fn render_spawner_menu(app: &mut App, f: &mut Frame, area: &Rect) {
    let layout = horizontal_split(area, 30);
    // Render the list section.
    render_spawner_list(app, f, &layout[0]);

    // Render the info section.
    // let spawner_state = app.spawner_state.as_ref().unwrap();
    // if spawner_state.is_new_option() {
    //     render_create_new_dialog(f, &layout[1]);
    // } else {
    //     render_spawner_info(app, f, &layout[1]);
    // }
}

pub fn render_spawner_list(app: &mut App, f: &mut Frame, area: &Rect) {
    let inner_area = render_border(
        f, area, "Spawners: ", app.focus == Focus::List);
    let spawner_state = app.spawner_state.as_ref().unwrap();
    // create a list with the spawner names
    render_list(f, &inner_area, spawner_state.get_spawner_names(),
                true, spawner_state.counter as usize, " > ");
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
