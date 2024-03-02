use color_eyre::eyre::Result;

pub mod cluster;
pub mod spawner;
pub mod cluster_state;

fn main() -> Result<()> {
    let cluster = cluster::Cluster::new(
        "levante", 
        "levante.dkrz.de", 
        "u301533", 
        "/home/silvano/.ssh/levante_key");
    // let cluster = cluster::Cluster::new(
    //     "levante", 
    //     "levante.dkrz.de", 
    //     "u301533", 
    //     "/tmp/id_rsa").unwrap();
    //
    cluster.add_cluster_to_ssh_config()?;

    // let mut spawn = spawner::Spawner::new(&cluster);
    // spawn.preset_name = String::from("test_very_long_name_for_a_job");
    // spawn.account = String::from("uo0780");
    // spawn.partition = String::from("compute");
    // spawn.time = String::from("00:05:00");
    // spawn.working_directory = String::from("/work/uo0780/u301533/FRIDOM/");
    //
    //
    // let mut sess = cluster.create_session()?;
    // spawn.spawn(&mut sess)?;
    //
    let cluster2 = cluster::Cluster::new(
        "levante2", 
        "levante2.dkrz.de", 
        "u301531", 
        "/home/silvano/.ssh/levante_key");

    let mut cluster_state = cluster_state::ClusterState::new();
    cluster_state.add_cluster(cluster);
    cluster_state.add_cluster(cluster2);
    cluster_state.save_cluster_list()?;

    let _cluster_state2 = cluster_state::ClusterState::load_cluster_list()?;
    //
    // cluster_state.save_cluster_list()?;

    // let toml_str = toml::to_string(&cluster_state)?;
    // println!("{}", toml_str);
    //
    // let cluster_state2: cluster_state::ClusterState = toml::from_str(&toml_str)?;

    Ok(())
}

