use crate::cluster;
use color_eyre::eyre;

#[derive(Debug)]
pub struct ClusterState {
    counter: u32,
    cluster_list: Vec<cluster::Cluster>,
    _new_cluster: cluster::Cluster,
}

impl ClusterState {
    pub fn new() -> ClusterState {
        ClusterState {
            counter: 0,
            cluster_list: Vec::new(),
            _new_cluster: cluster::Cluster::new_empty(),
        }
    }

    pub fn add_cluster(&mut self, cluster: cluster::Cluster) {
        self.cluster_list.push(cluster);
    }

    // pub fn cluster_is_valid(self) -> eyre::Result<()> {
    //     let cluster = self.get_cluster()?;
    //     cluster.check_if_cluster_is_valid()
    // }


    pub fn get_cluster(&self) -> eyre::Result<&cluster::Cluster> {
        let index = self.counter as usize;
        if self.is_new_cluster() {
            return Ok(&self._new_cluster);
        }
        let cluster = self.cluster_list.get(index).ok_or_else(|| eyre::eyre!("Cluster index out of bounds."))?;
        Ok(cluster)
    }

    pub fn is_new_cluster(&self) -> bool {
        // if the counter is at the end of the list, the new cluster is selected
        self.counter == self.cluster_list.len() as u32
    }

    pub fn save_cluster_list(&self) -> eyre::Result<()> {
        let home = std::env::var("HOME")?;
        let config_dir = format!("{}/.config/code-remote", home);
        std::fs::create_dir_all(&config_dir)?;
        let file = format!("{}/clusters.json", config_dir);
        let mut clusters = Vec::new();
        for cluster in &self.cluster_list {
            let cluster_json = serde_json::to_string(cluster)?;
            clusters.push(cluster_json);
        }
        let clusters_json = serde_json::to_string(&clusters)?;
        std::fs::write(file, clusters_json)?;
        Ok(())
    }

    pub fn load_cluster_list(&mut self) -> eyre::Result<()> {
        let home = std::env::var("HOME")?;
        let file = format!("{}/.config/code-remote/clusters.json", home);
        let clusters_json = std::fs::read_to_string(file)?;
        let clusters: Vec<String> = serde_json::from_str(&clusters_json)?;
        for cluster_json in clusters {
            let cluster: cluster::Cluster = serde_json::from_str(&cluster_json)?;
            self.add_cluster(cluster);
        }
        Ok(())
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

    fn create_dummy_cluster_state() -> ClusterState {
        let mut cluster_state = ClusterState::new();
        let cluster = cluster::Cluster::new("cluster1", "host1", "user1", "identity_file1");
        cluster_state.add_cluster(cluster);
        let cluster = cluster::Cluster::new("cluster2", "host2", "user2", "identity_file2");
        cluster_state.add_cluster(cluster);
        cluster_state
    }

    #[test]
    fn test_save_cluster_list() {
        let cluster_state = create_dummy_cluster_state();
        cluster_state.save_cluster_list().unwrap();
        let home = std::env::var("HOME").unwrap();
        let file = format!("{}/.config/code-remote/clusters.json", home);
        assert!(std::path::Path::new(&file).exists());
    }

    #[test]
    fn test_load_cluster_list() {
        let dummy_cluster_state = create_dummy_cluster_state();
        dummy_cluster_state.save_cluster_list().unwrap();

        let mut cluster_state = ClusterState::new();
        cluster_state.load_cluster_list().unwrap();
        assert_eq!(cluster_state.cluster_list.len(), 2);
        assert_eq!(cluster_state.get_cluster().unwrap().name, "cluster1");
        cluster_state.next();
        assert_eq!(cluster_state.get_cluster().unwrap().name, "cluster2");
    }

    #[test]
    fn test_next() {
        let mut cluster_state = create_dummy_cluster_state();
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
        let mut cluster_state = create_dummy_cluster_state();
        cluster_state.previous();
        assert_eq!(cluster_state.counter, 2);
        cluster_state.previous();
        assert_eq!(cluster_state.counter, 1);
        cluster_state.previous();
        assert_eq!(cluster_state.counter, 0);
    }

    #[test]
    fn test_is_new_cluster() {
        let mut cluster_state = create_dummy_cluster_state();
        assert!(! cluster_state.is_new_cluster());
        cluster_state.next();
        assert!(! cluster_state.is_new_cluster());
        cluster_state.next();
        assert!(cluster_state.is_new_cluster());
    }
}
