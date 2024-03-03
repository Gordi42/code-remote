use crate::starter::cluster::Cluster;
use ssh2::Session;
use std::{io::Read, process::Command};
use regex::Regex;
use color_eyre::eyre::Result;
use serde::{Serialize, Deserialize};

const NODE_NAME_REGEX: &str = r"l\d{5}";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Spawner {
    pub preset_name: String,
    // pub cluster: &'a Cluster,
    pub account: String,
    pub partition: String,
    pub time: String,
    pub working_directory: String,
    pub other_options: String,
}

impl Spawner {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new() -> Spawner {
        Spawner {
            preset_name: String::from(""),
            // cluster: cluster,
            account: String::from(""),
            partition: String::from(""),
            time: String::from(""),
            working_directory: String::from(""),
            other_options: String::from(""),
        }
    }

    // =======================================================================
    //       GETTERS AND SETTERS
    // =======================================================================
    pub fn get_entry_names(&self) -> Vec<String> {
        vec![
            "Preset Name: ".to_string(),
            "Account: ".to_string(),
            "Partition: ".to_string(),
            "Max. Time: ".to_string(),
            "Work. Dir.: ".to_string(),
            "Other Options: ".to_string(),
        ]
    }

    pub fn get_entry_values(&self) -> Vec<String> {
        vec![
            self.preset_name.clone(),
            self.account.clone(),
            self.partition.clone(),
            self.time.clone(),
            self.working_directory.clone(),
            self.other_options.clone(),
        ]
    }

    // =======================================================================
    //             MAIN FUNCTIONS
    // =======================================================================

    pub fn spawn(&self, session: &mut Session, cluster: &Cluster) -> Result<()> {
        // get the node name
        let mut node_name = self.get_node_name(session)?;
        // if the node name is not found, spawn the job
        if node_name.is_none() {
            self.salloc(session, cluster)?;
            node_name = self.get_node_name(session)?;
        }
        let node_name = node_name.unwrap(); 
        let node_alias = format!("{}-{}", cluster.name, node_name);
        // append teh node name to the ssh config file
        let config_entry = self.format_config_entry(&node_name, cluster);
        cluster.append_ssh_config(&config_entry)?;

        // clear the node from the known hosts file
        self.clear_known_host(&node_alias)?;
        self.spawn_vscode(&node_alias, session)?;
        Ok(())
    }

    pub fn get_spawn_command(&self) -> String {
        let mut command = String::from("salloc");
        if !self.preset_name.is_empty() {
            command.push_str(" --job-name=");
            command.push_str(&self.preset_name);
        }
        if !self.account.is_empty() {
            command.push_str(" -A ");
            command.push_str(&self.account);
        }
        if !self.partition.is_empty() {
            command.push_str(" -p ");
            command.push_str(&self.partition);
        }
        if !self.time.is_empty() {
            command.push_str(" -t ");
            command.push_str(&self.time);
        }
        if !self.other_options.is_empty() {
            command.push_str(" ");
            command.push_str(&self.other_options);
        }
        command.push_str(" --no-shell; exit");
        command
    }

    pub fn spawn_vscode(&self, node_alias: &str, session: &mut Session) -> Result<()> {
        // get the home directory if the working directory is not set
        let mut code_wd = self.working_directory.clone();
        if self.working_directory.is_empty() {
            let command = "echo $HOME";
            let mut channel = session.channel_session()?;
            channel.exec(&command)?;
            let mut output = String::new();
            channel.read_to_string(&mut output)?;
            code_wd = output.trim().to_string();
        }

        let remote_argument = format!(
            "vscode-remote://ssh-remote+{}/{}", node_alias, code_wd);
        Command::new("code")
            .arg("--folder-uri").arg(remote_argument)
            .output()
            .expect("Failed to start Visual Studio Code");
        Ok(())
    }

    // =======================================================================
    //            FILE OPERATIONS
    // =======================================================================

    pub fn format_config_entry(&self, node_name: &str, cluster: &Cluster) -> String {
        let host = format!("{}-{}", cluster.name, node_name);
        format!(
            "Host {}\n    HostName {}\n    User {}\n    IdentityFile {}\n    ProxyJump {}",
            host, 
            node_name, 
            cluster.user, 
            cluster.identity_file, 
            cluster.name)
    }

    pub fn clear_known_host(&self, node_alias: &str) -> Result<()> {
        if std::path::Path::new("$HOME/.ssh/known_hosts").exists() {
            let clear_command = format!(
                "ssh-keygen -f $HOME/.ssh/known_hosts -R {}", node_alias);
            Command::new(clear_command)
                .spawn()?;
        }
        Ok(())
    }

    // =======================================================================
    //             SSH OPERATIONS
    // =======================================================================

    pub fn get_node_name(&self, session: &mut Session) -> Result<Option<String>> {

        // get the node name
        let command = format!("squeue -u $USER --name {}", self.preset_name);
        let mut channel = session.channel_session()?;
        channel.exec(&command)?;
        // read the output
        let mut output = String::new();
        channel.read_to_string(&mut output)?;

        // Create a regex pattern to match "lXXXXX"
        let re = Regex::new(NODE_NAME_REGEX)?;

        // Search for the pattern in the text
        if let Some(mat) = re.find(&output) {
            let pattern = mat.as_str();
            Ok(Some(String::from(pattern)))
        } else {
            Ok(None)
        }

    }

    pub fn salloc(&self, session: &mut Session, cluster: &Cluster) -> Result<()> {
        let command = self.get_spawn_command();
        Ok(cluster.execute_and_forward(session, &command)?)
    }

}
