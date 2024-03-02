use std::{
    path::Path,
    fs::{File, OpenOptions},
    net::TcpStream,
    io::{self, Read, prelude::*} 
};
use ssh2::{Session, Channel};
use serde::{Serialize, Deserialize};
use color_eyre::{eyre::eyre, Result};


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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    pub name: String,
    pub host: String,
    pub user: String,
    pub identity_file: String,
}

impl Cluster {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new(name: &str, 
           host: &str, 
           user: &str, 
           identity_file: &str) -> Cluster {
        Cluster {
            name: name.to_string(),
            host: host.to_string(),
            user: user.to_string(),
            identity_file: identity_file.to_string(),
        }
    }

    pub fn new_empty() -> Cluster {
        Cluster {
            name: String::new(),
            host: String::new(),
            user: String::new(),
            identity_file: String::new(),
        }
    }
    // =======================================================================
    //             EVALUATION FUNCTIONS
    // =======================================================================

    pub fn cluster_is_valid(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(eyre!(ClusterError::EmptyName));
        }
        if self.host.is_empty() {
            return Err(eyre!(ClusterError::EmptyHost));
        }
        if self.user.is_empty() {
            return Err(eyre!(ClusterError::EmptyUser));
        }
        if self.identity_file.is_empty() {
            return Err(eyre!(ClusterError::EmptyIdentityFile));
        }
        if !Path::new(&self.identity_file).exists() {
            return Err(eyre!(ClusterError::NoneExistingIdentityFile));
        }
        Ok(())
    }

    /// Test if a connection to the cluster can be established
    pub fn test_connection(&self) -> Result<bool>{

        let private_key_str = self.read_private_key()?;
        let tcp = TcpStream::connect(format!("{}:22", &self.host))?;
        let mut sess = Session::new()?;

        // Associate the session with the TCP stream
        sess.set_tcp_stream(tcp);
        sess.handshake()?;

        // Try to authenticate using the private key
        sess.userauth_pubkey_memory(&self.user, None, &private_key_str, None)?;
        Ok(true)
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
        format!(
            "Host {}\n    HostName {}\n    User {}\n    IdentityFile {}",
            self.name, self.host, self.user, self.identity_file)
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

    /// Create a new session
    pub fn create_session(&self) -> Result<Session> {
        self.test_connection()?;
        let private_key_str = self.read_private_key()?;
        let tcp = TcpStream::connect(format!("{}:22", &self.host))?;
        let mut sess = Session::new()?;
        // Associate the session with the TCP stream
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        // Try to authenticate using the private key
        sess.userauth_pubkey_memory(&self.user, None, &private_key_str, None)?;
        Ok(sess)
    }

    /// Create a new channel
    pub fn create_channel(&self) -> Result<Channel> {
        let sess = self.create_session()?;
        Ok(sess.channel_session()?)
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

    #[test]
    fn test_new_cluster_empty_name() {
        let cluster = Cluster::new("", "localhost", "root", "/tmp/id_rsa");
        let is_valid = cluster.cluster_is_valid();
        assert!(is_valid.is_err());
    }

    #[test]
    fn test_new_cluster_empty_host() {
        let cluster = Cluster::new("test", "", "root", "/tmp/id_rsa");
        let is_valid = cluster.cluster_is_valid();
        assert!(is_valid.is_err());
    }

    #[test]
    fn test_new_cluster_empty_user() {
        let cluster = Cluster::new("test", "localhost", "", "/tmp/id_rsa");
        let is_valid = cluster.cluster_is_valid();
        assert!(is_valid.is_err());
    }

    #[test]
    fn test_new_cluster_empty_identity_file() {
        let cluster = Cluster::new("test", "localhost", "root", "");
        let is_valid = cluster.cluster_is_valid();
        assert!(is_valid.is_err());
    }

    #[test]
    fn test_new_cluster_none_existing_identity_file() {
        let cluster = Cluster::new(
            "test", "localhost", "root", "this_file_does_not_exist");
        let is_valid = cluster.cluster_is_valid();
        assert!(is_valid.is_err());
    }

    // Test if a connection to the cluster can be established
    #[test]
    fn test_test_connection() {
        let cluster = Cluster::new(
            "levante", 
            "levante.dkrz.de", 
            "u301533", 
            "/home/silvano/.ssh/levante_key");
        assert!(cluster.test_connection().unwrap());
    }

    #[test]
    fn test_test_connection_wrong_host() {
        let cluster = Cluster::new(
            "levante", 
            "wrong_host", 
            "u301533", 
            "/home/silvano/.ssh/levante_key");
        assert!(cluster.test_connection().is_err());
    }

    #[test]
    fn test_test_connection_wrong_user() {
        let cluster = Cluster::new(
            "levante", 
            "levante.dkrz.de", 
            "wrong_user", 
            "/home/silvano/.ssh/levante_key");
        assert!(cluster.test_connection().is_err());
    }

    #[test]
    fn test_test_connection_wrong_identity_file() {
        create_tmp_file();
        let cluster = Cluster::new(
            "levante", 
            "levante.dkrz.de", 
            "u301533", 
            "/tmp/id_rsa");
        assert!(cluster.test_connection().is_err());
    }

}

