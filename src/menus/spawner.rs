use crate::menus::cluster::Cluster;
use crate::double_column_menu::entry::Entry;
use ssh2::Session;
use std::{io::Read, process::Command, default::Default};
use regex::Regex;
use color_eyre::eyre::Result;
use serde::{Serialize, Deserialize};
use std::{fs, fs::OpenOptions, io::Write};



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Spawner {
    pub preset_name: String,
    pub account: String,
    pub partition: String,
    pub time: String,
    pub working_directory: String,
    pub other_options: String,
}

impl Default for Spawner {
    fn default() -> Spawner {
        Spawner {
            preset_name: String::from(""),
            account: String::from(""),
            partition: String::from(""),
            time: String::from("01:00:00"),
            working_directory: String::from(""),
            other_options: String::from(""),
        }
    }
}

impl Entry for Spawner {
    fn get_entry_name(&self) -> String {
        self.preset_name.clone()
    }

    fn set_entry_name(&mut self, name: &str) {
        self.preset_name = name.to_string();
    }

    fn get_value_from_index(&self, index: usize) -> String {
        match index {
            0 => self.preset_name.clone(),
            1 => self.account.clone(),
            2 => self.partition.clone(),
            3 => self.time.clone(),
            4 => self.working_directory.clone(),
            5 => self.other_options.clone(),
            _ => String::from(""),
        }
    }

    fn set_value_from_index(&mut self, index: usize, value: &str) {
        match index {
            0 => self.preset_name = value.to_string(),
            1 => self.account = value.to_string(),
            2 => self.partition = value.to_string(),
            3 => self.time = value.to_string(),
            4 => self.working_directory = value.to_string(),
            5 => self.other_options = value.to_string(),
            _ => {},
        }
    }

    fn get_entry_names(&self) -> Vec<String> {
        vec![
            "Preset Name: ".to_string(),
            "Account: ".to_string(),
            "Partition: ".to_string(),
            "Max. Time: ".to_string(),
            "Work. Dir.: ".to_string(),
            "Other Options: ".to_string(),
        ]
    }

    fn get_entry_values(&self) -> Vec<String> {
        vec![
            self.preset_name.clone(),
            self.account.clone(),
            self.partition.clone(),
            self.time.clone(),
            self.working_directory.clone(),
            self.other_options.clone(),
        ]
    }
}

impl Spawner {
    // =======================================================================
    //           CONSTRUCTOR
    // =======================================================================
    pub fn new(
        preset_name: &str,
        account: &str,
        partition: &str,
        time: &str,
        working_directory: &str,
        other_options: &str,
    ) -> Spawner {
        Spawner {
            preset_name: preset_name.to_string(),
            account: account.to_string(),
            partition: partition.to_string(),
            time: time.to_string(),
            working_directory: working_directory.to_string(),
            other_options: other_options.to_string(),
        }
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
        // append teh node name to the ssh config file
        self.add_cluster_to_ssh_config(&node_name, cluster)?;


        // clear the node from the known hosts file
        // try to clear the node from the known hosts file
        let _ = self.clear_known_host(&node_name);
        self.spawn_vscode(&self.preset_name, session)?;
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
            "vscode-remote://ssh-remote+cr-{}/{}", node_alias, code_wd);
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
        let mut entry = String::new();
        entry.push_str(&format!("# code-remote: start {}\n", self.preset_name));
        entry.push_str(format!("Host cr-{}\n", self.preset_name).as_str());
        entry.push_str(format!("    HostName {}\n", node_name).as_str());
        entry.push_str(format!("    User {}\n", cluster.user).as_str());
        if !cluster.identity_file.is_empty() {
            entry.push_str(format!("    IdentityFile {}\n", cluster.identity_file).as_str());
        }
        entry.push_str(format!("    ProxyJump cr-{}\n", cluster.name).as_str());
        entry.push_str(&format!("# code-remote: end {}", self.preset_name));
        entry
    }

    pub fn add_cluster_to_ssh_config(&self, node_name: &str, cluster: &Cluster) -> Result<()> {
        // Read the contents of the .ssh/config file
        let home = std::env::var("HOME")?;
        let config_file_path = format!("{}/.ssh/config", home);
        let config_content = fs::read_to_string(&config_file_path)?;

        // Define the regex pattern to match the start and end of the code remote entry
        let pattern = format!(r"(?ms)^# code-remote: start {}\n.*?# code-remote: end {}\s*"
                              , self.preset_name, self.preset_name);

        // Create a regex object
        let re = Regex::new(&pattern)?;

        // Replace the code remote entry with an empty string
        let modified_content = re.replace_all(&config_content, "").to_string();

        // Write the modified content back to the file
        fs::write(&config_file_path, modified_content)?;
        
        let entry = self.format_config_entry(node_name, cluster);
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .create(true)
            .open(&config_file_path)?;
        writeln!(file, "{}", entry)?;
        Ok(())
    }

    pub fn clear_known_host(&self, node_alias: &str) -> Result<()> {
        let home_path = std::env::var("HOME")?;
        let known_hosts_path = format!("{}/.ssh/known_hosts", home_path);
        if std::path::Path::new(&known_hosts_path).exists() {
            // don't print the output of the command
            Command::new("ssh-keygen")
                .args(&["-f", &known_hosts_path, "-R", &node_alias])
                .output()?;
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

        Ok(Some(output.trim().to_string()))
    }

    pub fn salloc(&self, session: &mut Session, cluster: &Cluster) -> Result<()> {
        let command = self.get_spawn_command();
        Ok(cluster.execute_and_forward(session, &command)?)
    }

}
