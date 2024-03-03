use crate::starter::{
    cluster::Cluster,
    spawner::Spawner,
    toml_list::TomlList,
    counter::Counter};
use color_eyre::eyre::Result;

const MAX_INFO_COUNTER: u32 = 6;

#[derive(Debug, PartialEq)]
pub struct SpawnerState {
    pub list_counter: Counter,
    pub info_counter: Counter,
    entries: TomlList<Spawner>,
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

    pub fn add_entry(&mut self, spawner: Spawner) {
        self.entries.push(spawner);
    }

    pub fn add_new_entry(&mut self) {
        let mut spawner = Spawner::new();
        spawner.preset_name = self.check_entry_name("New Preset");
        self.add_entry(spawner);
    }

    // Check if a new preset name is valid. E.g. it is not empty and 
    // it does not exist in the list of spawners.
    // If the name is not valid, it returns a modified name:
    // new name = name + "(i)"  where i is the smallest integer such that
    // the new name does not exist in the list of spawners.
    pub fn check_entry_name(&self, name: &str) -> String {
        let mut new_name = name.to_string();
        let mut i = 1;
        let mut name_list = self.get_entry_names();
        if !self.is_new_entry() {
            let old_name = self.get_entry().unwrap().preset_name.clone();
            name_list.retain(|c| c != &old_name);
        } else {
            name_list.pop();
        }
        while name_list.contains(&new_name) {
            new_name = format!("{} ({})", name, i);
            i += 1;
        }
        new_name
    }

    pub fn remove_selected(&mut self) {
        let index = self.list_counter.get_value() as usize;
        self.entries.entry.remove(index);
        let list_length = self.entries.len() as u32;
        self.list_counter.update_length(list_length+1);
    }

    pub fn get_entry(&self) -> Result<&Spawner> {
        let index = self.list_counter.get_value();
        self.entries.get(index as usize)
    }

    pub fn get_entry_mut(&mut self) -> Result<&mut Spawner> {
        let index = self.list_counter.get_value();
        self.entries.get_mut(index as usize)
    }

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

    pub fn get_entry_names(&self) -> Vec<String> {
        let mut spawn_list: Vec<String> = self.entries.entry
            .iter().map(|c| c.preset_name.clone()).collect();
        spawn_list.push("New Preset".to_string());
        spawn_list
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
    //  CHECKERS
    // =======================================================================

    pub fn is_new_entry(&self) -> bool {
        // if the counter is at the end of the list, the new spawner is selected
        let index = self.list_counter.get_value();
        index == self.entries.len() as u32
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
    fn test_new() {
        let spawner_state = SpawnerState::new_empty();
        assert_eq!(spawner_state.counter, 0);
    }

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
