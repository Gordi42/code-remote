pub trait Entry {
    fn get_entry_name(&self) -> String;
    fn set_entry_name(&mut self, name: &str);
    fn get_value_from_index(&self, index: usize) -> String;
    fn set_value_from_index(&mut self, index: usize, value: &str);
}
