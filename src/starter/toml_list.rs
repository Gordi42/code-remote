use color_eyre::eyre;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TomlList<T> {
    pub entry: Vec<T>,
}

impl<T: Serialize> TomlList<T> {
    // =======================================================================
    //             CONSTRUCTORS
    // =======================================================================
    pub fn new() -> TomlList<T> {
        TomlList {
            entry: Vec::new(),
        }
    }

    // =======================================================================
    //   MAIN METHODS
    // =======================================================================

    pub fn push(&mut self, item: T) {
        self.entry.push(item);
    }

    pub fn get(&self, index: usize) -> eyre::Result<&T> {
        let entry = self.entry.get(index)
            .ok_or_else(|| eyre::eyre!("Index out of bounds."))?;
        Ok(entry)
    }

    pub fn len(&self) -> usize {
        self.entry.len()
    }

    // =======================================================================
    //            FILE OPERATIONS
    // =======================================================================

    pub fn save(&self, filename: &str) -> eyre::Result<()> {
        let home = std::env::var("HOME")?;
        let config_dir = format!("{}/.config/code-remote", home);
        std::fs::create_dir_all(&config_dir)?;
        let file = format!("{}/{}.toml", config_dir, filename);
        let toml_str = toml::to_string(&self)?;
        // write the toml string to the file
        // if the file exists, it should be overwritten
        std::fs::write(file, toml_str)?;
        Ok(())
    }

    pub fn load(filename: &str) -> eyre::Result<TomlList<T>> 
    where for<'de> T: Deserialize<'de> {
        let home = std::env::var("HOME")?;
        let file = format!("{}/.config/code-remote/{}.toml", home, filename);
        // if the file does not exist, return an empty list
        if !std::path::Path::new(&file).exists() {
            return Ok(TomlList::new());
        }
        // otherwise, load the list
        let toml_str = std::fs::read_to_string(file)?;
        let list: TomlList<T> = toml::from_str(&toml_str)?;
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cluster;

    #[test]
    fn test_new() {
        let list: TomlList<cluster::Cluster> = TomlList::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut list: TomlList<cluster::Cluster> = TomlList::new();
        let cluster = cluster::Cluster::new("test", "test", "test", "test");
        list.push(cluster);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut list: TomlList<cluster::Cluster> = TomlList::new();
        let cluster = cluster::Cluster::new(
            "mname", "mhost", "muser", "mid");
        list.push(cluster);
        let entry = list.get(0).unwrap();
        assert_eq!(entry.name, "mname");
        assert_eq!(entry.host, "mhost");
        assert_eq!(entry.user, "muser");
        assert_eq!(entry.identity_file, "mid");
    }

    #[test]
    fn test_save() {
        let mut list: TomlList<cluster::Cluster> = TomlList::new();
        let cluster = cluster::Cluster::new("test", "test", "test", "test");
        list.push(cluster);
        list.save("test").unwrap();
        let home = std::env::var("HOME").unwrap();
        let file = format!("{}/.config/code-remote/test.toml", home);
        assert!(std::path::Path::new(&file).exists());
    }

    #[test]
    fn test_load() {
        // Create the list
        let mut list: TomlList<cluster::Cluster> = TomlList::new();
        let cluster = cluster::Cluster::new(
            "mname", "mhost", "muser", "mid");
        list.push(cluster);
        // Save the list
        list.save("test").unwrap();

        // Load the list
        let loaded_list: TomlList<cluster::Cluster> = TomlList::load("test").unwrap();

        // Test the loaded list
        assert_eq!(loaded_list.len(), 1);
        let entry = loaded_list.get(0).unwrap();
        assert_eq!(entry.name, "mname");
        assert_eq!(entry.host, "mhost");
        assert_eq!(entry.user, "muser");
        assert_eq!(entry.identity_file, "mid");
    }
}
