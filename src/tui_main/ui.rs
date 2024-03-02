use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::tui_main::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
    Paragraph::new(format!(
      "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {} \n\
        Cluster: {}
      ",
      app.cluster_state.counter,
      app.cluster_state.get_cluster().unwrap().name
    ))
    .block(
      Block::default()
        .title("Counter App")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Yellow))
    .alignment(Alignment::Center),
    f.size(),
  )
}
