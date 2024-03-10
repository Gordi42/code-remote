use color_eyre::Result;
use crate::menus::{
    cluster_menu::ClusterMenu,
    spawner_menu::SpawnerMenu};
use crate::double_column_menu::state::DoubleColumnMenu;
use crate::tui_main::tui::Tui;


#[derive(Debug, Default)]
pub enum Action {
    #[default]
    None,
    OpenClusterMenu,
    OpenSpawnerMenu,
    StartSpawner,
    Quit,
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
    pub cluster_menu: ClusterMenu,
    pub spawner_menu: SpawnerMenu,
    pub menu: Menu,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut new_app = Self::default();
        new_app.cluster_menu.load_entries()?;
        Ok(new_app)
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn open_cluster_menu(&mut self) {
        self.menu = Menu::Cluster;
    }

    pub fn open_spawner_menu(&mut self) {
        let cluster = self.cluster_menu.get_entry().unwrap();
        self.spawner_menu.cluster_name = cluster.name.clone();
        self.spawner_menu.load_entries().unwrap();
        self.menu = Menu::Spawner;
    }

    pub fn start_spawner(&mut self) {
        if self.cluster_menu.is_new_entry() {
            return;
        }
        self.quit();
        Tui::reset().expect("failed to reset the terminal");
        let cluster = self.cluster_menu.get_entry().unwrap();
        self.spawner_menu.spawn(&cluster).unwrap();
    }

    pub fn handle_action(&mut self) {
        match self.action {
            Action::Quit => { self.quit(); }
            Action::OpenClusterMenu => { self.open_cluster_menu(); }
            Action::OpenSpawnerMenu => { self.open_spawner_menu(); }
            Action::StartSpawner => { self.start_spawner(); }
            _ => {}
        };
        self.action = Action::None;
    }

}
