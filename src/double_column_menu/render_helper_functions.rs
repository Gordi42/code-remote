use ratatui::{prelude::*, widgets::*, layout::Flex};

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

pub fn render_create_new_dialog(f: &mut Frame, area: &Rect) {
    let text = "Press `Enter` to create a new entry.";
    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL)
            .border_type(BorderType::Rounded))
            .style(Style::default())
            .alignment(Alignment::Center),
        *area);
}

pub fn render_remove_dialog(f: &mut Frame) {
    let text = "Are you sure you want to remove this entry? (y/n)";
    render_info_dialog(f, text, Color::Red, 1);
}

pub fn render_info_dialog(f: &mut Frame, text: &str, color: Color, vsize: u16) {
    let window_width = f.size().width;
    let text_area_width = (0.8 * (window_width as f32)) as u16;

    let rect = centered_rect(f.size(), text_area_width, vsize+2);
    f.render_widget(Clear, rect); //this clears out the background
    f.render_widget(
        Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL)
               .border_type(BorderType::Rounded))
        .style(Style::default().fg(color))
        .alignment(Alignment::Center),
        rect);
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

pub fn vertical_split_fixed(area: &Rect, fixed: u16) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(fixed),
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

pub fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let horizontal = Layout::horizontal([width]).flex(Flex::Center);
    let vertical = Layout::vertical([height]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
