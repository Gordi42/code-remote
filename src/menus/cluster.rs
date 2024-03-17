use std::{
    fs::{File, OpenOptions},
    net::TcpStream,
    io::{self, Read, prelude::*} 
};
use ssh2::Session;
use serde::{Serialize, Deserialize};
use color_eyre::Result;
use crate::double_column_menu::entry::Entry;
use std::fs;
use regex::Regex;

#[derive(Debug, Default, PartialEq)]
pub enum SessionType {
    #[default]
    IdentityFile,
    Passphrase,
    Password,
}

#[derive(Debug, PartialEq)]
pub enum ClusterError {
    EmptyName,
    EmptyHost,
    EmptyUser,
    EmptyIdentityFile,
    NoneExistingIdentityFile,
}

impl std::fmt::Display for ClusterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClusterError::EmptyName => write!(
                f, "Cluster name is empty"),
            ClusterError::EmptyHost => write!(
                f, "Cluster host is empty"),
            ClusterError::EmptyUser => write!(
                f, "Cluster user is empty"),
            ClusterError::EmptyIdentityFile => write!(
                f, "Cluster identity file is empty"),
            ClusterError::NoneExistingIdentityFile => write!(
                f, "Cluster identity file does not exist"),
        }
    }
}

// =======================================================================
//            CLUSTER STRUCT
// =======================================================================

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cluster {
    pub name: String,
    pub host: String,
    pub user: String,
    pub identity_file: String,
}

impl Entry for Cluster {
    fn get_entry_name(&self) -> String {
        self.name.clone()
    }

    fn set_entry_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn get_value_from_index(&self, index: usize) -> String {
        match index {
            0 => self.name.clone(),
            1 => self.host.clone(),
            2 => self.user.clone(),
            3 => self.identity_file.clone(),
            _ => String::new(),
        }
    }

    fn set_value_from_index(&mut self, index: usize, value: &str) {
        match index {
            0 => self.name = value.to_string(),
            1 => self.host = value.to_string(),
            2 => self.user = value.to_string(),
            3 => self.identity_file = value.to_string(),
            _ => {},
        }
    }

    fn get_entry_names(&self) -> Vec<String> {
        vec![
            "Name: ".to_string(),
            "Host: ".to_string(),
            "User: ".to_string(),
            "IdentityFile: ".to_string(),
        ]
    }

    fn get_entry_values(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.host.clone(),
            self.user.clone(),
            self.identity_file.clone(),
        ]
    }
}

impl Cluster {
    // =======================================================================
    //            CONSTRUCTOR
    // =======================================================================
    pub fn new(name: &str, host: &str, user: &str, identity_file: &str) -> Cluster {
        Cluster {
            name: name.to_string(),
            host: host.to_string(),
            user: user.to_string(),
            identity_file: identity_file.to_string(),
        }
    }

    // =======================================================================
    //            FILE OPERATIONS
    // =======================================================================
    
    /// Format the cluster entry for the ssh config file
    fn format_config_entry(&self) -> String {
        let mut entry = String::new();
        entry.push_str(&format!("# code-remote: start {}\n", self.name));
        entry.push_str(&format!("Host cr-{}\n", self.name));
        entry.push_str(&format!("    HostName {}\n", self.host));
        entry.push_str(&format!("    User {}\n", self.user));
        if !self.identity_file.is_empty() {
            entry.push_str(&format!("    IdentityFile {}\n", self.identity_file));
        }
        entry.push_str(&format!("# code-remote: end {}", self.name));
        entry
    }

    /// Add the cluster to the ssh config file
    pub fn add_cluster_to_ssh_config(&self) -> Result<()>{
        // Read the contents of the .ssh/config file
        let home = std::env::var("HOME")?;
        let config_file_path = format!("{}/.ssh/config", home);
        let config_content = fs::read_to_string(&config_file_path)?;

        // Define the regex pattern to match the start and end of the code remote entry
        let pattern = format!(r"(?ms)^# code-remote: start {}\n.*?# code-remote: end {}\s*", self.name, self.name);

        // Create a regex object
        let re = Regex::new(&pattern)?;

        // Replace the code remote entry with an empty string
        let modified_content = re.replace_all(&config_content, "").to_string();

        // Write the modified content back to the file
        fs::write(&config_file_path, modified_content)?;
        
        let entry = self.format_config_entry();
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .create(true)
            .open(&config_file_path)?;
        writeln!(file, "{}", entry)?;
        Ok(())
    }
    
    // =======================================================================
    //             SSH OPERATIONS
    // =======================================================================

    /// Read the private key from the identity file
    pub fn read_private_key(&self) -> Result<String> {
        let mut file = File::open(&self.identity_file)?;
        let mut private_key_str = String::new();
        file.read_to_string(&mut private_key_str)?;
        Ok(private_key_str)
    }

    pub fn create_session(&self, session_type: &SessionType, password: &str) -> Result<Session> {
        // read the private key from the identity file
        let private_key = match session_type {
            SessionType::IdentityFile => self.read_private_key()?,
            SessionType::Passphrase => self.read_private_key()?,
            _ => String::new(),
        };
        // Connect to the Host (check if the host is reachable)
        let tcp = TcpStream::connect(format!("{}:22", &self.host))?;
        // Create a new session
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        // Try to authenticate
        match session_type {
            SessionType::IdentityFile => {
                sess.userauth_pubkey_memory(
                    &self.user, None, &private_key, None)
            },
            SessionType::Passphrase => {
                sess.userauth_pubkey_memory(
                    &self.user, None, &private_key, Some(password))
            },
            SessionType::Password => {
                sess.userauth_password(&self.user, &password)
            },
        }?;
        Ok(sess)
    }

    /// Execute a command and forward the output to the terminal
    pub fn execute_and_forward(&self, session: &Session, command: &str) -> Result<()>{
        let mut channel = session.channel_session()?;
        channel.exec(command)?;

        println!("{}", command);

        let mut ssh_stderr = channel.stderr();
        let stderr = io::stderr();
        let mut stderr = stderr.lock();
        io::copy(&mut ssh_stderr, &mut stderr)?;

        channel.wait_close()?;
        Ok(())
    }

}

// Tests 
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn create_tmp_file() {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create("/tmp/id_rsa").unwrap();
        file.write_all(b"").unwrap();
        // test if file exists
        assert!(Path::new("/tmp/id_rsa").exists());
    }

    #[test]
    fn test_new_cluster() {
        create_tmp_file();
        let cluster = Cluster::new(
            "test", "localhost", "root", "/tmp/id_rsa");
        assert_eq!(cluster.name, "test");
        assert_eq!(cluster.host, "localhost");
        assert_eq!(cluster.user, "root");
        assert_eq!(cluster.identity_file, "/tmp/id_rsa");
    }

}

