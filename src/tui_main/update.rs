use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui_main::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Char('e') | KeyCode::Char('E') => app.enter_info(),
        KeyCode::Char('s') | KeyCode::Char('S') => app.enter_list(),
        KeyCode::Down | KeyCode::Char('j') => app.increment_counter(),
        KeyCode::Up | KeyCode::Char('k') => app.decrement_counter(),
        _ => {}
    };
}
