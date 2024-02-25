pub mod cluster;
pub mod spawner;

fn main() {
    let cluster = cluster::Cluster::new(
        "levante", 
        "levante.dkrz.de", 
        "u301533", 
        "/home/silvano/.ssh/levante_key").unwrap();
    // let cluster = cluster::Cluster::new(
    //     "levante", 
    //     "levante.dkrz.de", 
    //     "u301533", 
    //     "/tmp/id_rsa").unwrap();
    //
    cluster.add_cluster_to_ssh_config();

    let mut spawn = spawner::Spawner::new(&cluster);
    spawn.preset_name = String::from("test_very_long_name_for_a_job");
    spawn.account = String::from("uo0780");
    spawn.partition = String::from("compute");
    spawn.time = String::from("01:00:00");
    spawn.working_directory = String::from("/work/uo0780/u301533/FRIDOM/");


    let mut sess = cluster.create_session().unwrap();
    spawn.spawn(&mut sess).unwrap();


}

