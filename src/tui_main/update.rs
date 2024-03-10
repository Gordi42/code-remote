use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui_main::app::{App, Menu};
use crate::double_column_menu::state::DoubleColumnMenu;

pub fn update(app: &mut App, key_event: KeyEvent) {
    // Ctrl + C should always quit, regardless of the input mode
    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        _ => {}
    };
    match app.menu {
        Menu::Cluster => app.cluster_menu.input(&mut app.action, key_event),
        Menu::Spawner => app.spawner_menu.input(&mut app.action, key_event),
    };
    app.handle_action();
}
