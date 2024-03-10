use color_eyre::eyre::Result;
use tui_textarea::{TextArea};

use crate::starter::{
    cluster::Cluster,
    counter::Counter,
    toml_list::TomlList,
    state::{State, Focus, InputMode}};

const CLUSTER_FILE: &str = "clusters";
const MAX_INFO_COUNTER: u32 = 4;

#[derive(Debug)]
pub struct ClusterState<'a> {
    pub list_counter: Counter,
    pub info_counter: Counter,
    entries: TomlList<Cluster>,
    focus: Focus,
    input_mode: InputMode,
    text_area: TextArea<'a>,
}

impl Default for ClusterState<'_> {
    fn default() -> Self {
        let entries: TomlList<Cluster> = TomlList::new();
        ClusterState {
            list_counter: Counter::new(1),
            info_counter: Counter::new(MAX_INFO_COUNTER),
            entries: entries,
            focus: Focus::default(),
            input_mode: InputMode::default(),
            text_area: TextArea::default(),
        }
    }
}

impl State<Cluster> for ClusterState<'_> {
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

    fn get_text_area(&mut self) -> &mut TextArea {
        &mut self.text_area
    }

}

impl ClusterState<'_> {

    // =====================================================================
    //  CHECKERS
    // =====================================================================

    pub fn cluster_is_valid(self) -> Result<()> {
        let cluster = self.get_entry()?;
        cluster.cluster_is_valid()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_dummy_cluster_state() -> Result<ClusterState> {
        let mut cluster_state = ClusterState::new_empty()?;
        let cluster = Cluster::new(
            "levante", "levante.dkrz.de", "u301533", "/home/silvano/.ssh/levante_key");
        cluster_state.add_entry(cluster);
        let cluster = Cluster::new("cluster2", "host2", "user2", "identity_file2");
        cluster_state.add_entry(cluster);
        Ok(cluster_state)
    }

    #[test]
    fn test_get_entry() {
        let mut cluster_state = create_dummy_cluster_state().unwrap();
        let cluster = cluster_state.get_entry().unwrap();
        assert_eq!(cluster.name, "levante");
        cluster_state.list_counter.increment();
        let cluster = cluster_state.get_entry().unwrap();
        assert_eq!(cluster.name, "cluster2");
    }

    #[test]
    fn test_save_entries() {
        let cluster_state = create_dummy_cluster_state().unwrap();
        cluster_state.save_entries().unwrap();
        let home = std::env::var("HOME").unwrap();
        let file = format!("{}/.config/code-remote/clusters.toml", home);
        assert!(std::path::Path::new(&file).exists());
    }

    #[test]
    fn test_load_entries() {
        let dummy_cluster_state = create_dummy_cluster_state().unwrap();
        dummy_cluster_state.save_entries().unwrap();

        let mut cluster_state = ClusterState::new_empty().unwrap();
        cluster_state.load_entries().unwrap();
        assert_eq!(cluster_state.entries.len(), 2);
        assert_eq!(cluster_state.get_entry().unwrap().name, "levante");
    }

    #[test]
    fn test_is_new_entry() {
        let mut cluster_state = create_dummy_cluster_state().unwrap();
        assert!(! cluster_state.is_new_entry());
        cluster_state.list_counter.increment();
        assert!(! cluster_state.is_new_entry());
        cluster_state.list_counter.increment();
        assert!(cluster_state.is_new_entry());
    }
}
