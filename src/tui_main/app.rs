use color_eyre::Result;
use crate::starter::{
    cluster_state::ClusterState,
    spawner_state::SpawnerState};

use crossterm::{
    event::{DisableMouseCapture},
    terminal::{self, LeaveAlternateScreen},
};
use std::io;

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
pub struct App {
    pub should_quit: bool,
    pub counter: u8,
    pub cluster_state: ClusterState,
    pub spawner_state: Option<SpawnerState>,
    pub focus: Focus,
    pub input_mode: InputMode,
    pub menu: Menu,
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
        match self.focus {
            Focus::List => {
                match self.menu {
                    Menu::Cluster => {
                        self.cluster_state.next();
                    }
                    Menu::Spawner => {
                        self.spawner_state.as_mut().unwrap().next();
                    }
                };
            }
            Focus::Info => { }
        };
        return
    }

    pub fn decrement_counter(&mut self) {
        match self.focus {
            Focus::List => {
                match self.menu {
                    Menu::Cluster => {
                        self.cluster_state.previous();
                    }
                    Menu::Spawner => {
                        self.spawner_state.as_mut().unwrap().previous();
                    }
                };
            }
            Focus::Info => { }
        };
        return
    }

    pub fn pressed_right(&mut self) {
        match self.menu {
            Menu::Cluster => {
                self.spawner_state = Some(
                    self.cluster_state.get_spawner_state().unwrap());
                self.menu = Menu::Spawner;
            }
            Menu::Spawner => {
                self.quit();
                reset().expect("failed to reset the terminal");
                let cluster = self.cluster_state.get_cluster().unwrap();
                self.spawner_state.as_ref().unwrap().spawn(&cluster).unwrap();
            }
        };
    }

    pub fn pressed_left(&mut self) {
        match self.menu {
            Menu::Cluster => { }
            Menu::Spawner => {
                self.menu = Menu::Cluster;
                self.spawner_state = None;
            }
        };
    }


    pub fn enter_info(&mut self) {
        self.focus = Focus::Info;
    }

    pub fn enter_list(&mut self) {
        self.focus = Focus::List;
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
