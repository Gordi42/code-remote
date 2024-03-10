use ratatui::{
    prelude::{Alignment, Frame, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui::{prelude::*, widgets::*, layout::Flex};
use crate::tui_main::app::{App, Menu, InputMode};
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
        Paragraph::new("Press `Esc`, `Ctrl-C` or `q` to stop running.")
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
