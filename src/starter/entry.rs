pub trait Entry {
    fn get_entry_name(&self) -> String;
    fn set_entry_name(&mut self, name: &str);
}
