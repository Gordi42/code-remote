use crate::starter::{
    cluster::Cluster,
    spawner_state::SpawnerState,
    toml_list::TomlList};
use color_eyre::eyre::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterState {
    pub counter: u32,
    cluster_list: TomlList<Cluster>,
    _new_cluster: Cluster,
}

impl ClusterState {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new_empty() -> Result<ClusterState> {
        let cluster_list: TomlList<Cluster> = TomlList::new();
        Ok(ClusterState {
            counter: 0,
            cluster_list: cluster_list,
            _new_cluster: Cluster::new_empty(),
        })
    }
    pub fn new_load() -> Result<ClusterState> {
        let cluster_list: TomlList<Cluster> = TomlList::load("clusters")?;
        Ok(ClusterState {
            counter: 0,
            cluster_list: cluster_list,
            _new_cluster: Cluster::new_empty(),
        })
    }

    // =======================================================================
    //  GETTERS AND SETTERS
    // =======================================================================

    pub fn add_cluster(&mut self, cluster: Cluster) {
        self.cluster_list.push(cluster);
    }

    pub fn get_cluster(&self) -> Result<&Cluster> {
        let index = self.counter as usize;
        if self.is_new_cluster() {
            return Ok(&self._new_cluster);
        }
        self.cluster_list.get(index)
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
        self.cluster_list.save("clusters")
    }

    pub fn load_cluster_list(&mut self) -> Result<()> {
        let loaded_list: TomlList<Cluster> = TomlList::load("clusters")?;
        self.cluster_list = loaded_list;
        Ok(())
    }

    // =======================================================================
    //            CONTROLS
    // =======================================================================

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
