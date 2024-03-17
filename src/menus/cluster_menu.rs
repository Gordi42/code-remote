use tui_textarea::{TextArea};

use crate::menus::{cluster::Cluster};
use crate::double_column_menu::{
    counter::Counter,
    toml_list::TomlList,
    double_column_menu::{DoubleColumnMenu, Focus, InputMode}};

use crate::app::{Action};

const CLUSTER_FILE: &str = "clusters";
const MAX_INFO_COUNTER: u32 = 4;

#[derive(Debug)]
pub struct ClusterMenu {
    pub list_counter: Counter,
    pub info_counter: Counter,
    entries: TomlList<Cluster>,
    focus: Focus,
    input_mode: InputMode,
    text_area: TextArea<'static>,
}

impl Default for ClusterMenu {
    fn default() -> Self {
        let entries: TomlList<Cluster> = TomlList::new();
        ClusterMenu {
            list_counter: Counter::new(1),
            info_counter: Counter::new(MAX_INFO_COUNTER),
            entries: entries,
            focus: Focus::default(),
            input_mode: InputMode::default(),
            text_area: TextArea::default(),
        }
    }
}

impl DoubleColumnMenu<Cluster> for ClusterMenu {
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

    fn get_entries(&self) -> &TomlList<Cluster> {
        &self.entries
    }

    fn get_entries_mut(&mut self) -> &mut TomlList<Cluster> {
        &mut self.entries
    }

    fn get_filename(&self) -> &str {
        CLUSTER_FILE
    }

    fn get_titlename(&self) -> &str {
        "Clusters: "
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
        *action = Action::OpenSpawnerMenu;
    }

    fn action_left(&mut self, _action: &mut Action) {
        // do nothing
        return;
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::Result;

    fn create_dummy_cluster_menu() -> Result<ClusterMenu> {
        let mut cluster_menu = ClusterMenu::default();
        let cluster = Cluster::new(
            "levante", "levante.dkrz.de", "u301533", "/home/silvano/.ssh/levante_key");
        cluster_menu.add_entry(cluster);
        let cluster = Cluster::new("cluster2", "host2", "user2", "identity_file2");
        cluster_menu.add_entry(cluster);
        Ok(cluster_menu)
    }

    #[test]
    fn test_get_entry() {
        let mut cluster_menu = create_dummy_cluster_menu().unwrap();
        let cluster = cluster_menu.get_entry().unwrap();
        assert_eq!(cluster.name, "levante");
        cluster_menu.list_counter.increment();
        let cluster = cluster_menu.get_entry().unwrap();
        assert_eq!(cluster.name, "cluster2");
    }

    #[test]
    fn test_is_new_entry() {
        let mut cluster_menu = create_dummy_cluster_menu().unwrap();
        assert!(! cluster_menu.is_new_entry());
        cluster_menu.list_counter.increment();
        assert!(! cluster_menu.is_new_entry());
        cluster_menu.list_counter.increment();
        assert!(cluster_menu.is_new_entry());
    }
}
