use color_eyre::Result;
use crate::starter::{
    cluster_state::ClusterState,
    spawner_state::SpawnerState,
    state::State};

use crossterm::{
    event::{DisableMouseCapture},
    terminal::{self, LeaveAlternateScreen},
};
use std::io;

#[derive(Debug, Default)]
pub enum Action {
    #[default]
    Tick,
    OpenClusterMenu,
    OpenSpawnerMenu,
    StartSpawner,
    Quit,
    None,
}

#[derive(Debug, Default)]
pub enum Menu {
    #[default]
    Cluster,
    Spawner,
}

#[derive(Debug, Default)]
pub struct App {
    pub action: Action,
    pub should_quit: bool,
    pub cluster_state: ClusterState,
    pub spawner_state: SpawnerState,
    pub menu: Menu,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut new_app = Self::default();
        new_app.cluster_state.load_entries()?;
        Ok(new_app)
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn open_cluster_menu(&mut self) {
        self.menu = Menu::Cluster;
    }

    pub fn open_spawner_menu(&mut self) {
        let cluster = self.cluster_state.get_entry().unwrap();
        self.spawner_state.cluster_name = cluster.name.clone();
        self.spawner_state.load_entries().unwrap();
        self.menu = Menu::Spawner;
    }

    pub fn start_spawner(&mut self) {
        if self.cluster_state.is_new_entry() {
            return;
        }
        self.quit();
        reset().expect("failed to reset the terminal");
        let cluster = self.cluster_state.get_entry().unwrap();
        self.spawner_state.spawn(&cluster).unwrap();
    }

    pub fn handle_action(&mut self) {
        match self.action {
            Action::Quit => { self.quit(); }
            Action::Tick => { self.tick(); }
            Action::OpenClusterMenu => { self.open_cluster_menu(); }
            Action::OpenSpawnerMenu => { self.open_spawner_menu(); }
            Action::StartSpawner => { self.start_spawner(); }
            _ => {}
        };
        self.action = Action::None;
    }

}



/// Resets the terminal interface.
fn reset() -> Result<()> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(
        io::stderr(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}
