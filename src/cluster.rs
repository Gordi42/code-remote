use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use ssh2::{Session, Channel};
use std::net::TcpStream;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Cluster {
    pub name: String,
    pub host: String,
    pub user: String,
    pub identity_file: String,
}

#[derive(Debug, PartialEq)]
pub enum ClusterError {
    EmptyName,
    EmptyHost,
    EmptyUser,
    EmptyIdentityFile,
    NoneExistingIdentityFile,
}

#[derive(Debug, PartialEq)]
pub enum ConnectionError {
    WrongHost,
    WrongUserOrIdentityFile,
}

impl Cluster {
    pub fn new(name: &str, 
           host: &str, 
           user: &str, 
           identity_file: &str) -> Result<Cluster, ClusterError> {
        if name.is_empty() {
            return Err(ClusterError::EmptyName);
        }
        if host.is_empty() {
            return Err(ClusterError::EmptyHost);
        }
        if user.is_empty() {
            return Err(ClusterError::EmptyUser);
        }
        if identity_file.is_empty() {
            return Err(ClusterError::EmptyIdentityFile);
        }
        if !Path::new(identity_file).exists() {
            return Err(ClusterError::NoneExistingIdentityFile);
        }
        Ok(Cluster {
            name: name.to_string(),
            host: host.to_string(),
            user: user.to_string(),
            identity_file: identity_file.to_string(),
        })
    }

    /// Read the private key from the identity file
    pub fn read_private_key(&self) -> String {
        let mut file = File::open(&self.identity_file)
            .expect("Failed to open private key file");
        let mut private_key_str = String::new();
        file.read_to_string(&mut private_key_str)
            .expect("Failed to read private key");
        private_key_str
    }

    /// Test if a connection to the cluster can be established
    pub fn test_connection(&self) -> Result<bool, ConnectionError>{

        let private_key_str = self.read_private_key();
        let tcp = TcpStream::connect(format!("{}:22", &self.host));
        // check the host is reachable
        if tcp.is_err() { return Err(ConnectionError::WrongHost); }
        let mut sess = Session::new().unwrap();

        // Associate the session with the TCP stream
        sess.set_tcp_stream(tcp.unwrap());
        sess.handshake().unwrap();

        // Try to authenticate using the private key
        let auth_status = sess.userauth_pubkey_memory(
            &self.user, None, &private_key_str, None);
        if auth_status.is_err() { 
            return Err(ConnectionError::WrongUserOrIdentityFile); 
        }
        Ok(true)
    }

    /// Create a new session
    pub fn create_session(&self) -> Result<Session, ConnectionError> {
        self.test_connection()?;
        let private_key_str = self.read_private_key();
        let tcp = TcpStream::connect(format!("{}:22", &self.host)).unwrap();
        let mut sess = Session::new().unwrap();
        // Associate the session with the TCP stream
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        // Try to authenticate using the private key
        sess.userauth_pubkey_memory(&self.user, None, &private_key_str, None)
            .unwrap();
        Ok(sess)
    }

    /// Create a new channel
    pub fn create_channel(&self) -> Result<Channel, ConnectionError> {
        let sess = self.create_session()?;
        Ok(sess.channel_session().unwrap())
    }

    /// Get the $HOME/.ssh/config file
    /// if does not exist, create it and return it
    pub fn get_config_file(&self) -> File {
        let home = std::env::var("HOME").unwrap();
        let ssh_config = format!("{}/.ssh/config", home);
        let file_status = File::open(&ssh_config);
        match file_status {
            Ok(file) => file,
            Err(_) => {
                // create the directory if it does not exist
                std::fs::create_dir_all(format!("{}/.ssh", home)).unwrap();
                // create the file
                File::create(&ssh_config).unwrap()
            },
        };
        OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .create(false)
            .open(&ssh_config)
            .expect("Failed to open file")
    }
    
    /// Format the cluster entry for the ssh config file
    fn format_config_entry(&self) -> String {
        format!(
            "Host {}\n    HostName {}\n    User {}\n    IdentityFile {}",
            self.name, self.host, self.user, self.identity_file)
    }

    /// Check if a given pattern is in a file
    fn is_pattern_in_file(&self, file: &File, pattern: &str) -> bool {
        let mut file = file;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("Failed to read file");
        file_contents.contains(pattern)
    }

    /// Check if a entry is in the ssh config file 
    /// if not => add it
    pub fn append_ssh_config(&self, entry: &str) {
        let mut file = self.get_config_file();
        if !self.is_pattern_in_file(&file, entry) {
            writeln!(file, "\n{}", entry).unwrap();
        }
    }

    /// Add the cluster to the ssh config file
    pub fn add_cluster_to_ssh_config(&self) {
        let entry = self.format_config_entry();
        self.append_ssh_config(&entry);
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
            "test", "localhost", "root", "/tmp/id_rsa").unwrap();
        assert_eq!(cluster.name, "test");
        assert_eq!(cluster.host, "localhost");
        assert_eq!(cluster.user, "root");
        assert_eq!(cluster.identity_file, "/tmp/id_rsa");
    }

    #[test]
    fn test_new_cluster_empty_name() {
        let cluster = Cluster::new("", "localhost", "root", "/tmp/id_rsa");
        assert_eq!(cluster, Err(ClusterError::EmptyName));
    }

    #[test]
    fn test_new_cluster_empty_host() {
        let cluster = Cluster::new("test", "", "root", "/tmp/id_rsa");
        assert_eq!(cluster, Err(ClusterError::EmptyHost));
    }

    #[test]
    fn test_new_cluster_empty_user() {
        let cluster = Cluster::new("test", "localhost", "", "/tmp/id_rsa");
        assert_eq!(cluster, Err(ClusterError::EmptyUser));
    }

    #[test]
    fn test_new_cluster_empty_identity_file() {
        let cluster = Cluster::new("test", "localhost", "root", "");
        assert_eq!(cluster, Err(ClusterError::EmptyIdentityFile));
    }

    #[test]
    fn test_new_cluster_none_existing_identity_file() {
        let cluster = Cluster::new(
            "test", "localhost", "root", "this_file_does_not_exist");
        assert_eq!(cluster, Err(ClusterError::NoneExistingIdentityFile));
    }

    // Test if a connection to the cluster can be established
    #[test]
    fn test_test_connection() {
        let cluster = Cluster::new(
            "levante", 
            "levante.dkrz.de", 
            "u301533", 
            "/home/silvano/.ssh/levante_key").unwrap();
        assert!(cluster.test_connection().unwrap());
    }

    #[test]
    fn test_test_connection_wrong_host() {
        let cluster = Cluster::new(
            "levante", 
            "wrong_host", 
            "u301533", 
            "/home/silvano/.ssh/levante_key").unwrap();
        assert_eq!(cluster.test_connection(), Err(ConnectionError::WrongHost));
    }

    #[test]
    fn test_test_connection_wrong_user() {
        let cluster = Cluster::new(
            "levante", 
            "levante.dkrz.de", 
            "wrong_user", 
            "/home/silvano/.ssh/levante_key").unwrap();
        assert_eq!(
            cluster.test_connection(), 
            Err(ConnectionError::WrongUserOrIdentityFile));
    }

    #[test]
    fn test_test_connection_wrong_identity_file() {
        create_tmp_file();
        let cluster = Cluster::new(
            "levante", 
            "levante.dkrz.de", 
            "u301533", 
            "/tmp/id_rsa").unwrap();
        assert_eq!(
            cluster.test_connection(), 
            Err(ConnectionError::WrongUserOrIdentityFile));
    }

}

