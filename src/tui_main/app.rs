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
    EnterInfo,
    EnterList,
    Quit,
    None,
}

#[derive(Debug, Default, PartialEq)]
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

    pub fn increment_info_counter(&mut self) {
        match self.menu {
            Menu::Cluster => {
                self.cluster_state.info_counter.increment();
            }
            Menu::Spawner => {
                let spawner_state = self.spawner_state.as_mut().unwrap();
                spawner_state.info_counter.increment();
            }
        }
    }

    pub fn decrement_info_counter(&mut self) {
        match self.menu {
            Menu::Cluster => {
                self.cluster_state.info_counter.decrement();
            }
            Menu::Spawner => {
                let spawner_state = self.spawner_state.as_mut().unwrap();
                spawner_state.info_counter.decrement();
            }
        }
    }

    pub fn increment_counter(&mut self) {
        match self.menu {
            Menu::Cluster => {
                self.cluster_state.list_counter.increment();
            }
            Menu::Spawner => {
                let spawner_state = self.spawner_state.as_mut().unwrap();
                spawner_state.list_counter.increment();
            }
        }
    }

    pub fn decrement_counter(&mut self) {
        match self.menu {
            Menu::Cluster => {
                // self.cluster_state.decrement_counter();
                self.cluster_state.list_counter.decrement();
            }
            Menu::Spawner => {
                let spawner_state = self.spawner_state.as_mut().unwrap();
                spawner_state.list_counter.decrement();
            }
        }
    }

    pub fn toggle_focus(&mut self) {
        match self.focus {
            Focus::List => { 
                if self.cluster_state.is_new_entry() {
                    self.cluster_state.add_new_entry();
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
                let buffer = match self.menu {
                    Menu::Cluster => 
                        self.cluster_state.get_input_buffer(),
                    Menu::Spawner => 
                        self.spawner_state.as_ref()
                            .unwrap().get_input_buffer(),
                };
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
        match self.menu {
            Menu::Cluster => {
                self.cluster_state.set_input_buffer(&buffer);
            }
            Menu::Spawner => {
                let cluster = self.cluster_state.get_entry().unwrap();
                self.spawner_state.as_mut().unwrap()
                    .set_input_buffer(&buffer, &cluster);
            }
        };
        self.toggle_editing();
    }

    pub fn open_remove_mode(&mut self) {
        self.input_mode = InputMode::Remove;
    }

    pub fn remove_selected(&mut self) {
        match self.menu {
            Menu::Cluster => {
                self.cluster_state.remove_selected();
                self.cluster_state.save_entries().unwrap();
                self.focus = Focus::List;
            }
            Menu::Spawner => {
                let spawner_state = self.spawner_state.as_mut().unwrap();
                spawner_state.remove_selected();
                let cluster = self.cluster_state.get_entry().unwrap();
                spawner_state.save_entries(&cluster).unwrap();
                self.focus = Focus::List;
            }
        };
        self.input_mode = InputMode::Normal;
    }

    pub fn pressed_right(&mut self) {
        match self.menu {
            Menu::Cluster => {
                if self.cluster_state.is_new_entry() {
                    self.cluster_state.add_new_entry();
                    self.focus = Focus::Info;
                    self.cluster_state.info_counter.reset();
                    self.toggle_editing();
                } else {
                    self.spawner_state = Some(
                        self.cluster_state.get_spawner_state().unwrap());
                    self.menu = Menu::Spawner;
                }
            }
            Menu::Spawner => {
                let is_new: bool;
                {
                    is_new = self.spawner_state.as_ref().unwrap()
                        .is_new_entry();
                }
                if is_new {
                    let spawner_state = self.spawner_state.as_mut().unwrap();
                    spawner_state.add_new_entry();
                    self.focus = Focus::Info;
                    spawner_state.info_counter.reset();
                    self.toggle_editing();
                } else {
                    self.quit();
                    reset().expect("failed to reset the terminal");
                    let cluster = self.cluster_state.get_entry().unwrap();
                    let spawner_state = self.spawner_state.as_ref().unwrap();
                    spawner_state.spawn(&cluster).unwrap();
                }
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
