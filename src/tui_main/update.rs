use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui_main::app::{App, InputMode, Focus, Menu};
use crate::starter::state::State;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => normal_mode(app, key_event),
        InputMode::Editing => editing_mode(app, key_event),
        InputMode::Remove => remove_mode(app, key_event),
    }
}

pub fn normal_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Char('i') => app.toggle_editing(),
        // KeyCode::Char('d') => app.open_remove_mode(),
        _ => {}
    };
    match app.menu {
        Menu::Cluster => app.cluster_state.input(key_event),
        Menu::Spawner => app.spawner_state.input(key_event),
    };
    match app.focus {
        Focus::List => match key_event.code {
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter
                => app.pressed_right(),
            KeyCode::Left | KeyCode::Char('h') => app.pressed_left(),
            _ => {}
        }
        Focus::Info => match key_event.code {
            KeyCode::Enter
                => app.toggle_editing(),
            _ => {}
        }
    };
}

pub fn editing_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Enter => app.save_input_buffer(),
        KeyCode::Esc => app.toggle_editing(),
        _ => {
            app.text_area.input(key_event);
        }
    };
}

pub fn remove_mode(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Enter | KeyCode::Char('y')
            => app.remove_selected(),
        KeyCode::Esc | KeyCode::Char('n')
            => app.input_mode = InputMode::Normal,
        _ => {}
    };
}
