use std::{
    fs::{File, OpenOptions},
    net::TcpStream,
    io::{self, Read, prelude::*} 
};
use ssh2::Session;
use serde::{Serialize, Deserialize};
use color_eyre::Result;
use crate::double_column_menu::entry::Entry;

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
    
    /// Get the $HOME/.ssh/config file
    /// if does not exist, create it and return it
    pub fn get_config_file(&self) -> Result<File> {
        let home = std::env::var("HOME")?;
        let ssh_config = format!("{}/.ssh/config", home);
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .create(true)
            .open(&ssh_config)?;
        Ok(file)
    }
    
    /// Format the cluster entry for the ssh config file
    fn format_config_entry(&self) -> String {
        let mut entry = String::new();
        entry.push_str(&format!("Host cr-{}\n", self.name));
        entry.push_str(&format!("    HostName {}\n", self.host));
        entry.push_str(&format!("    User {}\n", self.user));
        if !self.identity_file.is_empty() {
            entry.push_str(&format!("    IdentityFile {}\n", self.identity_file));
        }
        entry
    }

    /// Check if a given pattern is in a file
    fn is_pattern_in_file(&self, file: &mut File, pattern: &str) -> Result<bool> {
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;
        Ok(file_contents.contains(pattern))
    }

    /// Check if a entry is in the ssh config file 
    /// if not => add it
    pub fn append_ssh_config(&self, entry: &str) -> Result<()> {
        let mut file = self.get_config_file()?;
        if !(self.is_pattern_in_file(&mut file, entry)?) {
            writeln!(file, "\n{}", entry)?;
        }
        Ok(())
    }

    /// Add the cluster to the ssh config file
    pub fn add_cluster_to_ssh_config(&self) -> Result<()>{
        let entry = self.format_config_entry();
        self.append_ssh_config(&entry)
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

