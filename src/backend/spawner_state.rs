use crate::backend::{
    cluster::Cluster,
    spawner::Spawner,
    toml_list::TomlList};
use color_eyre::eyre::Result;

#[derive(Debug, PartialEq)]
pub struct SpawnerState<'a> {
    counter: u32,
    cluster: &'a Cluster,
    spawner_list: TomlList<Spawner>,
    _new_spawner: Spawner,
}

impl SpawnerState<'_> {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new_empty(cluster: &Cluster) -> SpawnerState {
        let spawner_list: TomlList<Spawner> = TomlList::new();
        SpawnerState {
            counter: 0,
            cluster: cluster,
            spawner_list: spawner_list,
            _new_spawner: Spawner::new(),
        }
    }

    pub fn new_load(cluster: &Cluster) -> Result<SpawnerState> {
        let spawner_list: TomlList<Spawner> = TomlList::load(cluster.name.as_str())?;
        Ok(SpawnerState {
            counter: 0,
            cluster: cluster,
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

    // =======================================================================
    //  SPAWN OPERATIONS
    // =======================================================================

    pub fn spawn(self) -> Result<()> {
        let spawner = self.get_spawner()?;
        let mut session = self.cluster.create_session()?;
        spawner.spawn(&mut session, self.cluster)?;
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

    pub fn save_spawner_list(&self) -> Result<()> {
        self.spawner_list.save(self.cluster.name.as_str())
    }

    pub fn load_spawner_list(&mut self) -> Result<()> {
        let name = self.cluster.name.as_str();
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
        let cluster = Cluster::new_empty();
        let spawner_state = SpawnerState::new_empty(&cluster);
        assert_eq!(spawner_state.counter, 0);
        assert_eq!(spawner_state.cluster, &cluster);
    }

    #[test]
    fn test_add_spawner() {
        let cluster = Cluster::new_empty();
        let mut spawner_state = SpawnerState::new_empty(&cluster);
        let spawner = Spawner::new();
        spawner_state.add_spawner(spawner);
        assert_eq!(spawner_state.spawner_list.len(), 1);
    }

    #[test]
    fn test_save_spawner_list() {
        let mut cluster = Cluster::new_empty();
        cluster.name = "test_cluster".to_string();
        let mut spawner_state = SpawnerState::new_empty(&cluster);
        let spawner = Spawner::new();
        spawner_state.add_spawner(spawner);
        spawner_state.save_spawner_list().unwrap();
        let home = std::env::var("HOME").unwrap();
        let file = format!("{}/.config/code-remote/test_cluster.toml", home);
        assert!(std::path::Path::new(&file).exists());
    }
}
