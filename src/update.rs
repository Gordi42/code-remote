use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Menu, Popup};
use crate::double_column_menu::double_column_menu::DoubleColumnMenu;

pub fn update(app: &mut App, key_event: KeyEvent) {
    // any input should reset the error popup
    app.popup = Popup::None;
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
