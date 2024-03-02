use color_eyre::Result;
use crate::starter::{
    cluster_state::ClusterState,
    spawner_state::SpawnerState};

pub enum Action {
    Tick,
    Increment,
    Decrement,
    EnterInfo,
    EnterList,
    Quit,
    None,
}

#[derive(Debug)]
pub enum Focus {
    List,
    Info,
}
impl Default for Focus {
    fn default() -> Self {
        Focus::List
    }
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}
impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
    }
}

#[derive(Debug)]
pub enum Menu {
    Cluster,
    Spawner,
}
impl Default for Menu {
    fn default() -> Self {
        Menu::Cluster
    }
}

#[derive(Debug, Default)]
pub struct App<'a> {
    pub should_quit: bool,
    pub counter: u8,
    pub cluster_state: ClusterState,
    pub spawner_state: Option<SpawnerState<'a>>,
    pub focus: Focus,
    pub input_mode: InputMode,
    pub menu: Menu,
}

impl App<'_> {
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
        match self.focus {
            Focus::List => {
                self.cluster_state.next();
            }
            Focus::Info => { }
        };
        return
    }

    pub fn decrement_counter(&mut self) {
        match self.focus {
            Focus::List => {
                self.cluster_state.previous();
            }
            Focus::Info => { }
        };
        return
    }

    // pub fn enter_spawner(&mut self) {
    //     self.spawner_state = Some(
    //         self.cluster_state.get_spawner_state().unwrap());
    //     self.menu = Menu::Spawner;
    // }

    pub fn enter_cluster(&mut self) {
        self.menu = Menu::Cluster;
    }

    pub fn enter_info(&mut self) {
        self.focus = Focus::Info;
    }

    pub fn enter_list(&mut self) {
        self.focus = Focus::List;
    }
}
