use tui_textarea::{TextArea};

use crate::menus::spawner::Spawner;
use crate::double_column_menu::{
    toml_list::TomlList,
    counter::Counter,
    double_column_menu::{DoubleColumnMenu, Focus, InputMode}};

use crate::app::{Action};

const MAX_INFO_COUNTER: u32 = 6;

#[derive(Debug)]
pub struct SpawnerMenu {
    pub cluster_name: String,
    pub list_counter: Counter,
    pub info_counter: Counter,
    entries: TomlList<Spawner>,
    focus: Focus,
    input_mode: InputMode,
    text_area: TextArea<'static>,
}

impl Default for SpawnerMenu {
    fn default() -> Self {
        let entries: TomlList<Spawner> = TomlList::new();
        SpawnerMenu {
            cluster_name: "cluster".to_string(),
            list_counter: Counter::new(1),
            info_counter: Counter::new(MAX_INFO_COUNTER),
            entries: entries,
            focus: Focus::default(),
            input_mode: InputMode::default(),
            text_area: TextArea::default(),
        }
    }
}

impl DoubleColumnMenu<Spawner> for SpawnerMenu {
    fn get_list_counter(&self) -> &Counter {
        &self.list_counter
    }

    fn get_list_counter_mut(&mut self) -> &mut Counter {
        &mut self.list_counter
    }

    fn get_info_counter(&self) -> &Counter {
        &self.info_counter
    }

    fn get_info_counter_mut(&mut self) -> &mut Counter {
        &mut self.info_counter
    }

    fn get_entries(&self) -> &TomlList<Spawner> {
        &self.entries
    }

    fn get_entries_mut(&mut self) -> &mut TomlList<Spawner> {
        &mut self.entries
    }

    fn get_filename(&self) -> &str {
        self.cluster_name.as_str()
    }

    fn get_titlename(&self) -> &str {
        "Spawners: "
    }

    fn get_focus(&self) -> &Focus {
        &self.focus
    }

    fn get_focus_mut(&mut self) -> &mut Focus {
        &mut self.focus
    }

    fn get_input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    fn get_input_mode_mut(&mut self) -> &mut InputMode {
        &mut self.input_mode
    }

    fn get_text_area(&mut self) -> &mut TextArea<'static> {
        &mut self.text_area
    }

    fn action_right(&mut self, action: &mut Action) {
        *action = Action::StartSpawner;
    }

    fn action_left(&mut self, action: &mut Action) {
        *action = Action::OpenClusterMenu;
    }
}

// =======================================================================
//           TESTS
// =======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_entry() {
        let mut spawner_menu = SpawnerMenu::default();
        let spawner = Spawner::default();
        spawner_menu.add_entry(spawner);
        assert_eq!(spawner_menu.entries.len(), 1);
    }

}
