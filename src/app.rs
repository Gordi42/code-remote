use color_eyre::{Result, eyre::Report};
use crate::menus::{
    cluster_menu::ClusterMenu,
    spawner_menu::SpawnerMenu,
    cluster::SessionType};
use crate::double_column_menu::double_column_menu::DoubleColumnMenu;
use crate::double_column_menu::render_helper_functions::render_info_dialog;
use ratatui::{backend::CrosstermBackend, Terminal};
use ratatui::prelude::*;
use crate::tui::Tui;
use ssh2::Session;
use rpassword;


#[derive(Debug, Default)]
pub enum Action {
    #[default]
    None,
    Quit,
    OpenClusterMenu,
    OpenSpawnerMenu,
    StartSpawner,
}

#[derive(Debug, Default)]
pub enum Menu {
    #[default]
    Cluster,
    Spawner,
}

#[derive(Debug, Default)]
pub enum Popup {
    #[default]
    None,
    Error(String),
}

#[derive(Default)]
pub struct App {
    pub action: Action,
    pub should_quit: bool,
    pub should_redraw: bool,
    pub cluster_menu: ClusterMenu,
    pub spawner_menu: SpawnerMenu,
    pub menu: Menu,
    pub popup: Popup,
    pub session: Option<Session>,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut new_app = Self::default();
        new_app.should_redraw = false;
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
        // check if the identity file is set
        // if not, open the spawner menu with password
        let session_type = if cluster.identity_file.is_empty() {
            SessionType::Password
        } else {
            SessionType::IdentityFile
        };
        self.open_session(session_type);
    }

    pub fn set_session(&mut self, session: Session) {
        let cluster = self.cluster_menu.get_entry().unwrap();
        cluster.add_cluster_to_ssh_config().unwrap();
        self.spawner_menu.cluster_name = cluster.name.clone();
        self.spawner_menu.load_entries().unwrap();
        self.menu = Menu::Spawner;
        self.session = Some(session);
    }

    pub fn format_error_message(&self, error: &Report) -> String {
        let mut error_msg = format!("Error: {}", error);
        if error_msg.contains("failed to lookup address information") {
            error_msg = "Error: failed to lookup address information: \
                         Name or service not known.\n\
                         Please check the hostname!"
                         .to_string();
        }
        if error_msg.contains("Connection refused") {
            error_msg = "Error: Connection refused.\n\
                         Please check if the SSH port is open!"
                         .to_string();
        }
        if error_msg.contains("Username/PublicKey") {
            error_msg = "Error: Username/PublicKey combination invalid.\n\
                         Please check both entries!"
                         .to_string();
        }
        if error_msg.contains("File not found") {
            error_msg = "Error: Identity file not found.\n\
                         Please check if the file path exists!"
                         .to_string();
        }
        error_msg
    }

    pub fn open_session(&mut self, session_type: SessionType) {
        // get password or passphrase from the user
        let password = match session_type {
            SessionType::Passphrase => {
                ask_for_passphrase("Enter your passphrase: ").unwrap()
            },
            SessionType::Password => {
                ask_for_passphrase("Enter your password: ").unwrap()
            },
            _ => "".to_string(),
        };
        // render a info dialog that says "Connecting To Cluster ..."
        self.render_connect_screen();
        let cluster = self.cluster_menu.get_entry().unwrap();
        let session_result = cluster.create_session(
            &session_type, &password);
        // handle the result
        match session_result {
            Ok(session) => {
                self.set_session(session);
            },
            Err(e) => {
                let error_msg = self.format_error_message(&Report::msg(e));
                // check if the error message contains keyfile auth failed
                // if so, open the session but with a passphrase
                if error_msg.contains("keyfile auth failed") {
                    if session_type == SessionType::IdentityFile {
                        self.open_session(SessionType::Passphrase);
                        return;
                    }
                }
                self.popup = Popup::Error(error_msg);
            },
        };
    }


    pub fn start_spawner(&mut self) {
        if self.cluster_menu.is_new_entry() {
            return;
        }
        self.quit();
        Tui::reset().expect("failed to reset the terminal");
        let cluster = self.cluster_menu.get_entry().unwrap();
        let spawner = self.spawner_menu.get_entry().unwrap();
        spawner.spawn(self.session.as_mut().unwrap(), &cluster).unwrap();
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

    pub fn render_connect_screen(&mut self) {
        // render a info dialog that says "Connecting To Cluster ..."
        let backend = CrosstermBackend::new(std::io::stderr());
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| {
            render_info_dialog(
                frame, "Connecting To Cluster ...", Color::Yellow, 1);
        }).unwrap();
        terminal.hide_cursor().unwrap();
        self.should_redraw = true;
    }

}


pub fn ask_for_passphrase(message: &str) -> Result<String> {
    // disable the alternative Terminal UI for now
    Tui::reset().expect("failed to reset the terminal");

    // Prompt the user for a password without showing the input
    let password = rpassword::prompt_password_stdout(message).unwrap();
    Tui::enter_alternate_screen().unwrap();

    // Use the password as needed
    Ok(password)

}

