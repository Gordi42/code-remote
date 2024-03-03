use color_eyre::Result;
use crate::starter::{
    cluster_state::ClusterState,
    spawner_state::SpawnerState};

use crossterm::{
    event::{DisableMouseCapture},
    terminal::{self, LeaveAlternateScreen},
};
use tui_textarea::{TextArea};
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

#[derive(Debug, Default)]
pub enum Focus {
    #[default]
    List,
    Info,
}

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
    Remove,
}

#[derive(Debug, Default)]
pub enum Menu {
    #[default]
    Cluster,
    Spawner,
}

#[derive(Debug, Default)]
pub struct App<'a> {
    pub should_quit: bool,
    pub cluster_state: ClusterState,
    pub spawner_state: Option<SpawnerState>,
    pub text_area: TextArea<'a>,
    pub focus: Focus,
    pub input_mode: InputMode,
    pub input_buffer: String,
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
        match self.menu {
            Menu::Cluster => {
                self.cluster_state.increment_counter(&self.focus);
            }
            Menu::Spawner => {
                self.spawner_state.as_mut().unwrap().next();
            }
        }
    }

    pub fn decrement_counter(&mut self) {
        match self.menu {
            Menu::Cluster => {
                self.cluster_state.decrement_counter(&self.focus);
            }
            Menu::Spawner => {
                self.spawner_state.as_mut().unwrap().previous();
            }
        }
    }

    pub fn toggle_focus(&mut self) {
        match self.focus {
            Focus::List => { 
                if self.cluster_state.is_new_cluster() {
                    self.cluster_state.add_new_cluster();
                }
                self.focus = Focus::Info; }
            Focus::Info => { self.focus = Focus::List; }
        };
    }

    pub fn toggle_editing(&mut self) {
        // only allow toggling when the focus is on the info section
        match self.focus {
            Focus::List => {return}
            _ => {}
        };
        match self.input_mode {
            InputMode::Normal => { 
                let buffer = self.cluster_state.get_input_buffer();
                self.input_buffer = buffer.to_string();
                self.text_area = TextArea::from([buffer]);
                self.input_mode = InputMode::Editing; 
            }
            InputMode::Editing => { 
                self.input_mode = InputMode::Normal; 
            }
            _ => {}
        };
    }

    pub fn save_input_buffer(&mut self) {
        let buffer = self.text_area.lines().join("\n");
        self.cluster_state.set_input_buffer(&buffer);
        self.toggle_editing();
    }

    pub fn open_remove_mode(&mut self) {
        self.input_mode = InputMode::Remove;
    }

    pub fn remove_selected(&mut self) {
        match self.menu {
            Menu::Cluster => {
                self.cluster_state.remove_selected();
                self.cluster_state.save_cluster_list().unwrap();
                self.focus = Focus::List;
            }
            Menu::Spawner => {
                // self.spawner_state.as_mut().unwrap().remove_selected();
            }
        };
        self.input_mode = InputMode::Normal;
    }

    pub fn pressed_right(&mut self) {
        match self.menu {
            Menu::Cluster => {
                if self.cluster_state.is_new_cluster() {
                    self.cluster_state.add_new_cluster();
                    self.focus = Focus::Info;
                    self.cluster_state.info_counter = 0;
                    self.toggle_editing();
                } else {
                    self.spawner_state = Some(
                        self.cluster_state.get_spawner_state().unwrap());
                    self.menu = Menu::Spawner;
                }
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
