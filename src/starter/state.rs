use serde::{Serialize, Deserialize};
use color_eyre::eyre::Result;
use std::default::Default;
use ratatui::{prelude::*, widgets::*};

use crate::starter::{
    entry::Entry,
    counter::Counter,
    toml_list::TomlList};

#[derive(Debug, Default, PartialEq)]
pub enum Focus {
    #[default]
    List,
    Info,
}

pub trait State<T: Serialize + for<'a> Deserialize<'a> + PartialEq + Entry + Default> {
// =======================================================================
//  METHODS TO IMPLEMENT
// =======================================================================
    fn get_list_counter(&self) -> &Counter;
    fn get_list_counter_mut(&mut self) -> &mut Counter;
    fn get_info_counter(&self) -> &Counter;
    fn get_info_counter_mut(&mut self) -> &mut Counter;
    fn get_entries(&self) -> &TomlList<T>;
    fn get_entries_mut(&mut self) -> &mut TomlList<T>;
    fn get_filename(&self) -> &str;
    fn get_titlename(&self) -> &str;
    fn get_focus(&self) -> &Focus;

// =======================================================================
//  DEFAULT METHODS
// =======================================================================
    // =======================================================================
    //             CONSTRUCTORS
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

    fn get_input_buffer(&self) -> String {
        let index = self.get_info_counter().get_value() as usize;
        let entry = self.get_entry().unwrap();
        entry.get_value_from_index(index)
    }

    fn set_input_buffer(&mut self, value: &str) {
        let index = self.get_info_counter().get_value() as usize;
        let new_name = if index == 0 {
            self.check_entry_name(value)
        } else {
            value.to_string()
        };
        let entry = self.get_entry_mut().unwrap();
        entry.set_value_from_index(index, &new_name);
        self.save_entries().unwrap();
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

    // =======================================================================
    //            FILE OPERATIONS
    // =======================================================================

    fn save_entries(&mut self) -> Result<()> {
        let filename: String;
        {
            filename = self.get_filename().to_string();
        }
        let entries = self.get_entries_mut();
        entries.save(&filename)
    }

    fn load_entries(&mut self) -> Result<()> {
        let filename: String;
        {
            filename = self.get_filename().to_string();
        }
        let entries = self.get_entries_mut();
        let new_entries = TomlList::load(&filename)?;
        entries.set_list(new_entries.entry);
        let entry_len = entries.len() as u32;
        self.get_list_counter_mut().update_length(entry_len + 1);
        Ok(())
    }

    // =======================================================================
    //            Rendering
    // =======================================================================
    
    fn render(&self, f: &mut Frame, area: &Rect) {
        // split the area horizontally, such that the left hand side 
        // shows a list of selectable entries, end the right hand 
        // side information about the entry 
        let layout = horizontal_split(area, 30);

        // Render the list section.
        let titlename = self.get_titlename();
        let focus = self.get_focus();
        let inner_area = render_border(
            f, &layout[0], titlename, focus == &Focus::List);
        let index = self.get_list_counter().get_value();
        // create a list with the cluster names
        render_list(f, &inner_area, 
                    self.get_entry_names(), true, 
                    index as usize, " > ");

        // Render the info section.
        if self.is_new_entry() {
            render_create_new_dialog(f, &layout[1]);
        } else {
            self.render_info(f, &layout[1]);
        }
         
    }

    fn render_info(&self, f: &mut Frame, area: &Rect) {
        let focus = self.get_focus();
        let inner_area = render_border(
            f, area, "Info: ", focus == &Focus::Info);

        // create a layout for the inner area
        let layout = horizontal_split_fixed(&inner_area, 15);

        let enable_highlight = focus == &Focus::Info;

        let entry = self.get_entry().unwrap();
        let counter = self.get_info_counter().get_value() as usize;

        render_list(f, &layout[0], entry.get_entry_names(), enable_highlight, 
                    counter, "  ");
        render_list(f, &layout[1], entry.get_entry_values(), enable_highlight, 
                    counter, "  ");
    }

    // =======================================================================
    //           INPUT HANDLING
    // =======================================================================

     



}

// =======================================================================
//  UI Helper Functions
// =======================================================================

/// Render the borders of a widget and return the area inside the borders.
fn render_border(f: &mut Frame, area: &Rect, title: &str,
                     is_focused: bool) -> Rect {
    let mut block = Block::default().title(title).borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    if is_focused {
        block = block.fg(Color::Blue);
    }
    // add a <tab> to the title if the widget is not focused
    let block = match is_focused {
        true => block,
        false => block.title(block::Title::from("<tab>")
                            .alignment(Alignment::Right)),
    };
    f.render_widget(block.clone(), *area);
    block.inner(*area)
}

fn render_list(f: &mut Frame, area: &Rect, items: Vec<String>,
                   enable_highlight: bool, counter: usize, 
                   highlight_symbol: &str) {
    let highlight_style = match enable_highlight {
        true => Style::default().add_modifier(Modifier::BOLD)
            .bg(Color::Blue).fg(Color::Black),
        false => Style::default(),
    };
    // create the list state
    let mut state = ListState::default();
    state.select(Some(counter));
    // create the list
    let list = List::new(items)
        .style(Style::default())
        .highlight_style(highlight_style)
        .highlight_symbol(highlight_symbol)
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    // render the list
    f.render_stateful_widget(list, *area, &mut state);
}

fn render_create_new_dialog(f: &mut Frame, area: &Rect) {
    let text = "Press `Enter` to create a new entry.";
    f.render_widget(
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL)
            .border_type(BorderType::Rounded))
            .style(Style::default())
            .alignment(Alignment::Center),
        *area);
}

fn horizontal_split(area: &Rect, percentage: u16) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(percentage),
                Constraint::Percentage(100 - percentage),
            ]
            .as_ref(),
        )
        .split(*area);
    layout.to_vec()
}

fn horizontal_split_fixed(area: &Rect, fixed: u16) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(fixed),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(*area);
    layout.to_vec()
}
