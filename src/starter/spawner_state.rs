use crate::starter::{
    cluster::Cluster,
    spawner::Spawner,
    toml_list::TomlList,
    counter::Counter,
    state::State};
use color_eyre::eyre::Result;

const MAX_INFO_COUNTER: u32 = 6;

#[derive(Debug, PartialEq)]
pub struct SpawnerState {
    pub list_counter: Counter,
    pub info_counter: Counter,
    entries: TomlList<Spawner>,
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

}

impl SpawnerState {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new_empty() -> SpawnerState {
        let entries: TomlList<Spawner> = TomlList::new();
        SpawnerState {
            list_counter: Counter::new(1),
            info_counter: Counter::new(MAX_INFO_COUNTER),
            entries: entries,
        }
    }

    pub fn new_load(cluster: &Cluster) -> Result<SpawnerState> {
        let entries: TomlList<Spawner> = TomlList::load(cluster.name.as_str())?;
        let list_length = entries.len() as u32;
        Ok(SpawnerState {
            list_counter: Counter::new(list_length+1),
            info_counter: Counter::new(MAX_INFO_COUNTER),
            entries: entries,
        })
    }

    // =======================================================================
    //  GETTERS AND SETTERS
    // =======================================================================

    pub fn get_input_buffer(&self) -> &str {
        let spawner = self.get_entry().unwrap();
        match self.info_counter.get_value() {
            0 => spawner.preset_name.as_str(),
            1 => spawner.account.as_str(),
            2 => spawner.partition.as_str(),
            3 => spawner.time.as_str(),
            4 => spawner.working_directory.as_str(),
            5 => spawner.other_options.as_str(),
            _ => "",
        }
    }

    pub fn set_input_buffer(&mut self, text: &str, cluster: &Cluster) {
        let info_counter = self.info_counter.get_value() as usize;
        let new_name = if info_counter == 0 {
            self.check_entry_name(text)
        } else {
            text.to_string()
        };
        let spawner = self.get_entry_mut().unwrap();
        match info_counter {

            0 => spawner.preset_name = new_name,
            1 => spawner.account = new_name,
            2 => spawner.partition = new_name,
            3 => spawner.time = new_name,
            4 => spawner.working_directory = new_name,
            5 => spawner.other_options = new_name,
            _ => {}
        }
        self.save_entries(cluster).unwrap();
    }


    // =======================================================================
    //  SPAWN OPERATIONS
    // =======================================================================

    pub fn spawn(&self, cluster: &Cluster) -> Result<()> {
        let spawner = self.get_entry()?;
        let mut session = cluster.create_session()?;
        spawner.spawn(&mut session, cluster)?;
        Ok(())
    }

    // =======================================================================
    //            FILE OPERATIONS
    // =======================================================================

    pub fn save_entries(&self, cluster: &Cluster) -> Result<()> {
        self.entries.save(cluster.name.as_str())
    }

    pub fn load_entries(&mut self, cluster: &Cluster) -> Result<()> {
        let name = cluster.name.as_str();
        self.entries = TomlList::load(name)?;
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
