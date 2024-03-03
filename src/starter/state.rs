use serde::Serialize;
use color_eyre::eyre::Result;
use std::default::Default;

use crate::starter::{
    entry::Entry,
    counter::Counter,
    toml_list::TomlList};

pub trait State<T: Serialize + PartialEq + Entry + Default> {
// =======================================================================
//  METHODS TO IMPLEMENT
// =======================================================================
    fn get_list_counter(&self) -> &Counter;
    fn get_list_counter_mut(&mut self) -> &mut Counter;
    fn get_info_counter(&self) -> &Counter;
    fn get_info_counter_mut(&mut self) -> &mut Counter;
    fn get_entries(&self) -> &TomlList<T>;
    fn get_entries_mut(&mut self) -> &mut TomlList<T>;

// =======================================================================
//  DEFAULT METHODS
// =======================================================================

    // -----------------------------------------------------------------------
    //  GETTERS AND SETTERS
    // -----------------------------------------------------------------------
    
    fn get_entry(&self) -> Result<&T> {
        let index = self.get_list_counter().get_value();
        self.get_entries().get(index as usize)
    }

    fn get_entry_mut(&mut self) -> Result<&mut T> {
        let index = self.get_list_counter().get_value();
        self.get_entries_mut().get_mut(index as usize)
    }

    fn get_entry_names(&self) -> Vec<String> {
        let entries = self.get_entries();
        let mut spawn_list: Vec<String> = entries.entry
            .iter().map(|c| c.get_entry_name()).collect();
        spawn_list.push("Create New".to_string());
        spawn_list
    }

    // -----------------------------------------------------------------------
    //  ADDING AND REMOVING ENTRIES
    // -----------------------------------------------------------------------

    fn add_entry(&mut self, entry: T){
        self.get_entries_mut().push(entry);
        let list_length = self.get_entries().len() as u32;
        self.get_list_counter_mut().update_length(list_length+1);
    }

    fn add_new_entry(&mut self) {
        let mut new_entry = T::default();
        new_entry.set_entry_name(&self.check_entry_name("New Entry"));
        // spawner.preset_name = self.check_entry_name("New Preset");
        self.add_entry(new_entry);
    }

    fn remove_selected(&mut self) {
        let index = self.get_list_counter().get_value() as usize;
        self.get_entries_mut().entry.remove(index);
        let list_length = self.get_entries().len() as u32;
        self.get_list_counter_mut().update_length(list_length+1);
    }

    // -----------------------------------------------------------------------
    //  CHECKERS
    // -----------------------------------------------------------------------

    fn is_new_entry(&self) -> bool {
        let counter = self.get_list_counter().get_value();
        let length = self.get_entries().len() as u32;
        counter == length
    }
    
    /// Check if a new name is valid. E.g. it is not empty and 
    /// it does not exist in the list of entries.
    /// If the name is not valid, it returns a modified name:
    /// new name = name + "(i)"  where i is the smallest integer such that
    /// the new name does not exist in the list of entries.
    fn check_entry_name(&self, name: &str) -> String {
        let mut new_name = name.to_string();
        let mut i = 1;
        let mut name_list = self.get_entry_names();
        // We need to remove the current selected entry from the list
        if !self.is_new_entry() {
            let old_name = self.get_entry().unwrap().get_entry_name();
            name_list.retain(|c| c != &old_name);
        } else {
            name_list.pop();
        }
        while name_list.contains(&new_name) {
            new_name = format!("{}({})", name, i);
            i += 1;
        }
        new_name
    }
}
