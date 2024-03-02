use color_eyre::eyre::Result;

pub mod cluster;
pub mod spawner;
pub mod cluster_state;
pub mod spawner_state;
pub mod toml_list;


fn main() -> Result<()> {
    let cluster_state = cluster_state::ClusterState::new_load()?;
    let mut spawner_state = cluster_state.get_spawner_state()?;
    // spawner_state.next();
    spawner_state.spawn()?;

    Ok(())
}

