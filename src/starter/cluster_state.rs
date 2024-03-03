use crate::starter::{
    cluster::Cluster,
    spawner_state::SpawnerState,
    toml_list::TomlList};
use crate::tui_main::app::Focus;
use color_eyre::eyre::Result;
use serde::{Serialize, Deserialize};

const CLUSTER_FILE: &str = "clusters";
const MAX_INFO_COUNTER: u32 = 4;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterState {
    pub counter: u32,
    pub info_counter: u32,
    cluster_list: TomlList<Cluster>,
}

impl ClusterState {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new_empty() -> Result<ClusterState> {
        let cluster_list: TomlList<Cluster> = TomlList::new();
        Ok(ClusterState {
            counter: 0,
            info_counter: 0,
            cluster_list: cluster_list,
        })
    }
    pub fn new_load() -> Result<ClusterState> {
        let cluster_list: TomlList<Cluster> = TomlList::load(CLUSTER_FILE)?;
        Ok(ClusterState {
            counter: 0,
            info_counter: 0,
            cluster_list: cluster_list,
        })
    }

    // =======================================================================
    //  GETTERS AND SETTERS
    // =======================================================================

    pub fn add_cluster(&mut self, cluster: Cluster) {
        self.cluster_list.push(cluster);
    }

    pub fn add_new_cluster(&mut self) {
        let mut cluster = Cluster::new_empty();
        cluster.name = self.check_cluster_name("New Cluster");
        self.add_cluster(cluster);
    }

    // Check if a new cluster name is valid. E.g. it is not empty and 
    // it does not exist in the list of clusters.
    // If the name is not valid, it returns a modified name:
    // new name = name + "(i)"  where i is the smallest integer such that
    // the new name does not exist in the list of clusters.
    pub fn check_cluster_name(&self, name: &str) -> String {
        let mut new_name = name.to_string();
        if new_name.is_empty() {
            new_name = "New Cluster".to_string();
        }
        let mut i = 1;
        while self.cluster_list.entry.iter().any(|c| c.name == new_name) {
            new_name = format!("{}({})", name, i);
            i += 1;
        }
        new_name
    }

    pub fn remove_selected(&mut self) {
        let index = self.counter as usize;
        self.cluster_list.entry.remove(index);
    }

    pub fn get_cluster(&self) -> Result<&Cluster> {
        self.cluster_list.get(self.counter as usize)
    }

    pub fn get_cluster_mut(&mut self) -> Result<&mut Cluster> {
        self.cluster_list.get_mut(self.counter as usize)
    }

    pub fn get_input_buffer(&self) -> &str {
        let cluster = self.get_cluster().unwrap();
        match self.info_counter {
            0 => &cluster.name,
            1 => &cluster.host,
            2 => &cluster.user,
            3 => &cluster.identity_file,
            _ => &cluster.name,
        }
    }

    pub fn set_input_buffer(&mut self, value: &str) {
        let info_counter = self.info_counter as usize;
        let new_name = if info_counter == 0 {
            self.check_cluster_name(value)
        } else {
            value.to_string()
        };
        let cluster = self.get_cluster_mut().unwrap();
        match info_counter {
            0 => cluster.name = new_name,
            1 => cluster.host = new_name,
            2 => cluster.user = new_name,
            3 => cluster.identity_file = new_name,
            _ => {},
        }
        self.save_cluster_list().unwrap();
    }

    pub fn get_spawner_state(&self) -> Result<SpawnerState> {
        let cluster = self.get_cluster()?;
        SpawnerState::new_load(cluster)
    }

    pub fn get_cluster_names(&self) -> Vec<String> {
        let mut clust_list: Vec<String> = self.cluster_list.entry
            .iter().map(|c| c.name.clone()).collect();
        clust_list.push("Create New".to_string());
        clust_list
    }

    // =======================================================================
    //  CHECKERS
    // =======================================================================

    pub fn is_new_cluster(&self) -> bool {
        // if the counter is at the end of the list, the new cluster is selected
        self.counter == self.cluster_list.len() as u32
    }

    // pub fn cluster_is_valid(self) -> eyre::Result<()> {
    //     let cluster = self.get_cluster()?;
    //     cluster.check_if_cluster_is_valid()
    // }

    // =======================================================================
    //            FILE OPERATIONS
    // =======================================================================

    pub fn save_cluster_list(&self) -> Result<()> {
        self.cluster_list.save(CLUSTER_FILE)
    }

    pub fn load_cluster_list(&mut self) -> Result<()> {
        let loaded_list: TomlList<Cluster> = TomlList::load(CLUSTER_FILE)?;
        self.cluster_list = loaded_list;
        Ok(())
    }

    // =======================================================================
    //            CONTROLS
    // =======================================================================
    pub fn increment_counter(&mut self, focus: &Focus) {
        match focus {
            Focus::List => {
                self.next();
            }
            Focus::Info => {
                self.info_counter += 1;
                if self.info_counter >= MAX_INFO_COUNTER {
                    self.info_counter = 0;
                }
            }
        }
    }

    pub fn decrement_counter(&mut self, focus: &Focus) {
        match focus {
            Focus::List => {
                self.previous();
            }
            Focus::Info => {
                if self.info_counter == 0 {
                    self.info_counter = MAX_INFO_COUNTER;
                }
                self.info_counter -= 1;
            }
        }
    }
    

    pub fn next(&mut self) {
        let max = self.cluster_list.len() as u32;
        self.counter += 1;
        if self.counter > max {
            self.counter = 0;
        }
    }

    pub fn previous(&mut self) {
        let max = self.cluster_list.len() as u32;
        if self.counter == 0 {
            self.counter = max+1;
        }
        self.counter -= 1;
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_dummy_cluster_state() -> Result<ClusterState> {
        let mut cluster_state = ClusterState::new_empty()?;
        let cluster = Cluster::new(
            "levante", "levante.dkrz.de", "u301533", "/home/silvano/.ssh/levante_key");
        cluster_state.add_cluster(cluster);
        let cluster = Cluster::new("cluster2", "host2", "user2", "identity_file2");
        cluster_state.add_cluster(cluster);
        Ok(cluster_state)
    }

    #[test]
    fn test_save_cluster_list() {
        let cluster_state = create_dummy_cluster_state().unwrap();
        cluster_state.save_cluster_list().unwrap();
        let home = std::env::var("HOME").unwrap();
        let file = format!("{}/.config/code-remote/clusters.toml", home);
        assert!(std::path::Path::new(&file).exists());
    }

    #[test]
    fn test_load_cluster_list() {
        let dummy_cluster_state = create_dummy_cluster_state().unwrap();
        dummy_cluster_state.save_cluster_list().unwrap();

        let mut cluster_state = ClusterState::new_empty().unwrap();
        cluster_state.load_cluster_list().unwrap();
        assert_eq!(cluster_state.cluster_list.len(), 2);
        assert_eq!(cluster_state.get_cluster().unwrap().name, "levante");
        cluster_state.next();
        assert_eq!(cluster_state.get_cluster().unwrap().name, "cluster2");
    }

    #[test]
    fn test_next() {
        let mut cluster_state = create_dummy_cluster_state().unwrap();
        assert_eq!(cluster_state.counter, 0);
        cluster_state.next();
        assert_eq!(cluster_state.counter, 1);
        cluster_state.next();
        assert_eq!(cluster_state.counter, 2);
        cluster_state.next();
        assert_eq!(cluster_state.counter, 0);
    }

    #[test]
    fn test_previous() {
        let mut cluster_state = create_dummy_cluster_state().unwrap();
        cluster_state.previous();
        assert_eq!(cluster_state.counter, 2);
        cluster_state.previous();
        assert_eq!(cluster_state.counter, 1);
        cluster_state.previous();
        assert_eq!(cluster_state.counter, 0);
    }

    #[test]
    fn test_is_new_cluster() {
        let mut cluster_state = create_dummy_cluster_state().unwrap();
        assert!(! cluster_state.is_new_cluster());
        cluster_state.next();
        assert!(! cluster_state.is_new_cluster());
        cluster_state.next();
        assert!(cluster_state.is_new_cluster());
    }
}
