use crate::starter::{
    cluster::Cluster,
    spawner::Spawner,
    toml_list::TomlList};
use color_eyre::eyre::Result;

#[derive(Debug, PartialEq)]
pub struct SpawnerState {
    pub counter: u32,
    spawner_list: TomlList<Spawner>,
    _new_spawner: Spawner,
}

impl SpawnerState {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new_empty() -> SpawnerState {
        let spawner_list: TomlList<Spawner> = TomlList::new();
        SpawnerState {
            counter: 0,
            spawner_list: spawner_list,
            _new_spawner: Spawner::new(),
        }
    }

    pub fn new_load(cluster: &Cluster) -> Result<SpawnerState> {
        let spawner_list: TomlList<Spawner> = TomlList::load(cluster.name.as_str())?;
        Ok(SpawnerState {
            counter: 0,
            spawner_list: spawner_list,
            _new_spawner: Spawner::new(),
        })
    }

    // =======================================================================
    //  GETTERS AND SETTERS
    // =======================================================================

    pub fn add_spawner(&mut self, spawner: Spawner) {
        self.spawner_list.push(spawner);
    }

    pub fn get_spawner(&self) -> Result<&Spawner> {
        let index = self.counter as usize;
        if self.is_new_spawner() {
            return Ok(&self._new_spawner);
        }
        self.spawner_list.get(index)
    }

    pub fn get_spawner_names(&self) -> Vec<String> {
        let mut spawn_list: Vec<String> = self.spawner_list.entry
            .iter().map(|c| c.preset_name.clone()).collect();
        spawn_list.push("New Preset".to_string());
        spawn_list
    }

    // =======================================================================
    //  SPAWN OPERATIONS
    // =======================================================================

    pub fn spawn(&self, cluster: &Cluster) -> Result<()> {
        let spawner = self.get_spawner()?;
        let mut session = cluster.create_session()?;
        spawner.spawn(&mut session, cluster)?;
        Ok(())
    }

    // =======================================================================
    //  CHECKERS
    // =======================================================================

    pub fn is_new_spawner(&self) -> bool {
        // if the counter is at the end of the list, the new spawner is selected
        self.counter == self.spawner_list.len() as u32
    }

    // =======================================================================
    //            FILE OPERATIONS
    // =======================================================================

    pub fn save_spawner_list(&self, cluster: &Cluster) -> Result<()> {
        self.spawner_list.save(cluster.name.as_str())
    }

    pub fn load_spawner_list(&mut self, cluster: &Cluster) -> Result<()> {
        let name = cluster.name.as_str();
        self.spawner_list = TomlList::load(name)?;
        Ok(())
    }

    // =======================================================================
    //            CONTROLS
    // =======================================================================

    pub fn next(&mut self) {
        let max = self.spawner_list.len() as u32;
        self.counter += 1;
        if self.counter > max {
            self.counter = 0;
        }
    }

    pub fn previous(&mut self) {
        let max = self.spawner_list.len() as u32;
        if self.counter == 0 {
            self.counter = max+1;
        }
        self.counter -= 1;
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
    fn test_add_spawner() {
        let mut spawner_state = SpawnerState::new_empty();
        let spawner = Spawner::new();
        spawner_state.add_spawner(spawner);
        assert_eq!(spawner_state.spawner_list.len(), 1);
    }

    #[test]
    fn test_save_spawner_list() {
        let mut cluster = Cluster::new_empty();
        cluster.name = "test_cluster".to_string();
        let mut spawner_state = SpawnerState::new_empty();
        let spawner = Spawner::new();
        spawner_state.add_spawner(spawner);
        spawner_state.save_spawner_list(&cluster).unwrap();
        let home = std::env::var("HOME").unwrap();
        let file = format!("{}/.config/code-remote/test_cluster.toml", home);
        assert!(std::path::Path::new(&file).exists());
    }
}
