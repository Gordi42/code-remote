use crate::starter::{
    cluster::Cluster,
    spawner::Spawner,
    toml_list::TomlList,
    counter::Counter,
    state::{State, Focus}};
use color_eyre::eyre::Result;

const MAX_INFO_COUNTER: u32 = 6;

#[derive(Debug, PartialEq)]
pub struct SpawnerState {
    pub cluster_name: String,
    pub list_counter: Counter,
    pub info_counter: Counter,
    entries: TomlList<Spawner>,
    focus: Focus,
}

impl Default for SpawnerState {
    fn default() -> Self {
        let entries: TomlList<Spawner> = TomlList::new();
        SpawnerState {
            cluster_name: "cluster".to_string(),
            list_counter: Counter::new(1),
            info_counter: Counter::new(MAX_INFO_COUNTER),
            entries: entries,
            focus: Focus::default(),
        }
    }
}

impl State<Spawner> for SpawnerState {
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
}

impl SpawnerState {

    // ======================================================================
    //  SPAWN OPERATIONS
    // ======================================================================

    pub fn spawn(&self, cluster: &Cluster) -> Result<()> {
        let spawner = self.get_entry()?;
        let mut session = cluster.create_session()?;
        spawner.spawn(&mut session, cluster)?;
        Ok(())
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
        let mut spawner_state = SpawnerState::new_empty();
        let spawner = Spawner::new();
        spawner_state.add_entry(spawner);
        assert_eq!(spawner_state.entries.len(), 1);
    }

    #[test]
    fn test_save_entries() {
        let mut cluster = Cluster::new_empty();
        cluster.name = "test_cluster".to_string();
        let mut spawner_state = SpawnerState::new_empty();
        let spawner = Spawner::new();
        spawner_state.add_entry(spawner);
        spawner_state.save_entries(&cluster).unwrap();
        let home = std::env::var("HOME").unwrap();
        let file = format!("{}/.config/code-remote/test_cluster.toml", home);
        assert!(std::path::Path::new(&file).exists());
    }
}
