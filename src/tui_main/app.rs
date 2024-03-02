use color_eyre::Result;
use crate::starter::cluster_state::ClusterState;

pub enum Action {
    Tick,
    Increment,
    Decrement,
    Quit,
    None,
}

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub counter: u8,
    pub cluster_state: ClusterState,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut new_app = Self::default();
        new_app.cluster_state = ClusterState::new_load()?;
        Ok(new_app)
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        self.cluster_state.next();
    }

    pub fn decrement_counter(&mut self) {
        self.cluster_state.previous();
    }
}
