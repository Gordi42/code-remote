use crate::starter::{
    cluster::Cluster,
    counter::Counter,
    spawner_state::SpawnerState,
    toml_list::TomlList};
use color_eyre::eyre::Result;

const CLUSTER_FILE: &str = "clusters";
const MAX_INFO_COUNTER: u32 = 4;

#[derive(Debug, PartialEq, Default)]
pub struct ClusterState {
    pub list_counter: Counter,
    pub info_counter: Counter,
    entries: TomlList<Cluster>,
}

impl ClusterState {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new_empty() -> Result<ClusterState> {
        let entries: TomlList<Cluster> = TomlList::new();
        Ok(ClusterState {
            list_counter: Counter::new(1),
            info_counter: Counter::new(MAX_INFO_COUNTER),
            entries: entries,
        })
    }
    pub fn new_load() -> Result<ClusterState> {
        let entries: TomlList<Cluster> = TomlList::load(CLUSTER_FILE)?;
        let list_length = entries.len() as u32;
        Ok(ClusterState {
            list_counter: Counter::new(list_length+1),
            info_counter: Counter::new(MAX_INFO_COUNTER),
            entries: entries,
        })
    }

    // =======================================================================
    //  GETTERS AND SETTERS
    // =======================================================================

    pub fn add_entry(&mut self, cluster: Cluster) {
        self.entries.push(cluster);
        let list_length = self.entries.len() as u32;
        self.list_counter.update_length(list_length+1);
    }

    pub fn add_new_entry(&mut self) {
        let mut cluster = Cluster::new_empty();
        cluster.name = self.check_entry_name("New Cluster");
        self.add_entry(cluster);
    }

    // Check if a new cluster name is valid. E.g. it is not empty and 
    // it does not exist in the list of clusters.
    // If the name is not valid, it returns a modified name:
    // new name = name + "(i)"  where i is the smallest integer such that
    // the new name does not exist in the list of clusters.
    pub fn check_entry_name(&self, name: &str) -> String {
        let mut new_name = name.to_string();
        if new_name.is_empty() {
            new_name = "New Cluster".to_string();
        }
        let mut i = 1;
        let mut name_list = self.get_entry_names();
        // remove the current cluster name from the list
        if !self.is_new_entry() {
            let old_name = self.get_entry().unwrap().name.clone();
            name_list.retain(|n| n != &old_name);
        } else {
            name_list.pop();
        }

        // find a new name that does not exist in the list
        while name_list.contains(&new_name) {
            new_name = format!("{}({})", name, i);
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

    pub fn get_entry(&self) -> Result<&Cluster> {
        let index = self.list_counter.get_value();
        self.entries.get(index as usize)
    }

    pub fn get_entry_mut(&mut self) -> Result<&mut Cluster> {
        let index = self.list_counter.get_value();
        self.entries.get_mut(index as usize)
    }

    pub fn get_input_buffer(&self) -> &str {
        let cluster = self.get_entry().unwrap();
        match self.info_counter.get_value() {
            0 => &cluster.name,
            1 => &cluster.host,
            2 => &cluster.user,
            3 => &cluster.identity_file,
            _ => &cluster.name,
        }
    }

    pub fn set_input_buffer(&mut self, value: &str) {
        let info_counter = self.info_counter.get_value() as usize;
        let new_name = if info_counter == 0 {
            self.check_entry_name(value)
        } else {
            value.to_string()
        };
        let cluster = self.get_entry_mut().unwrap();
        match info_counter {
            0 => cluster.name = new_name,
            1 => cluster.host = new_name,
            2 => cluster.user = new_name,
            3 => cluster.identity_file = new_name,
            _ => {},
        }
        self.save_entries().unwrap();
    }

    pub fn get_spawner_state(&self) -> Result<SpawnerState> {
        let cluster = self.get_entry()?;
        SpawnerState::new_load(cluster)
    }

    pub fn get_entry_names(&self) -> Vec<String> {
        let mut clust_list: Vec<String> = self.entries.entry
            .iter().map(|c| c.name.clone()).collect();
        clust_list.push("Create New".to_string());
        clust_list
    }

    // =======================================================================
    //  CHECKERS
    // =======================================================================

    pub fn is_new_entry(&self) -> bool {
        // if the counter is at the end of the list, the new cluster is selected
        let index = self.list_counter.get_value();
        index == self.entries.len() as u32
    }

    // pub fn cluster_is_valid(self) -> eyre::Result<()> {
    //     let cluster = self.get_entry()?;
    //     cluster.check_if_cluster_is_valid()
    // }

    // =======================================================================
    //            FILE OPERATIONS
    // =======================================================================

    pub fn save_entries(&self) -> Result<()> {
        self.entries.save(CLUSTER_FILE)
    }

    pub fn load_entries(&mut self) -> Result<()> {
        let loaded_list: TomlList<Cluster> = TomlList::load(CLUSTER_FILE)?;
        self.entries = loaded_list;
        Ok(())
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
