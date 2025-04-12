use serde::{Serialize, Deserialize};
use color_eyre::eyre::Result;
use std::default::Default;
use ratatui::{prelude::*, widgets::*};
use crossterm::event::{KeyCode, KeyEvent};
use tui_textarea::{TextArea};

use crate::double_column_menu::{
    entry::Entry,
    counter::Counter,
    toml_list::TomlList,
    render_helper_functions::*,};
use crate::app::Action;

#[derive(Debug, Default, PartialEq)]
pub enum Focus {
    #[default]
    List,
    Info,
}

#[derive(Debug, Default, PartialEq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
    Remove,
}

pub trait DoubleColumnMenu<T: Serialize + for<'a> Deserialize<'a> + PartialEq + Entry + Default> {
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
    fn get_focus_mut(&mut self) -> &mut Focus;
    fn get_input_mode(&self) -> &InputMode;
    fn get_input_mode_mut(&mut self) -> &mut InputMode;
    fn get_text_area(&mut self) -> &mut TextArea<'static>;
    fn action_right(&mut self, action: &mut Action);
    fn action_left(&mut self, action: &mut Action);

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
        self.add_entry(new_entry);
    }

    fn remove_selected(&mut self) {
        let index = self.get_list_counter().get_value() as usize;
        self.get_entries_mut().entry.remove(index);
        let list_length = self.get_entries().len() as u32;
        self.get_list_counter_mut().update_length(list_length+1);
        // reset the focus to the list
        *self.get_focus_mut() = Focus::List;
        *self.get_input_mode_mut() = InputMode::Normal;
        self.save_entries().unwrap();
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

    fn render(&mut self, f: &mut Frame, area: &Rect) {
        // always render the menu
        self.render_menu(f, area);
        // render additional widgets based on the input mode
        match *self.get_input_mode() {
            InputMode::Editing => self.render_editor(f),
            InputMode::Remove => render_remove_dialog(f),
            _ => {}
        }
    }
    
    fn render_menu(&self, f: &mut Frame, area: &Rect) {
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

        // split the inner area vertically for the list in the top
        // and control information in the bottom
        let vertical_layout = vertical_split_fixed(&inner_area, 1);

        // create a layout for the inner area
        let layout = horizontal_split_fixed(&vertical_layout[0], 15);

        let enable_highlight = focus == &Focus::Info;

        let entry = self.get_entry().unwrap();
        let counter = self.get_info_counter().get_value() as usize;

        render_list(f, &layout[0], entry.get_entry_names(), enable_highlight, 
                    counter, "  ");
        render_list(f, &layout[1], entry.get_entry_values(), enable_highlight, 
                    counter, "  ");

        let control_info_text = match self.get_input_mode() {
            InputMode::Editing => "Press `Enter` to save, `Esc` to cancel.",
            _ => match self.get_focus() {
                Focus::List => "Press `Enter` to select, 'd' to delete.",
                Focus::Info => "Press `Enter` to edit.",
            }
        };

        let control_info = Paragraph::new(control_info_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
        f.render_widget(control_info, vertical_layout[1]);
    }

    fn render_editor(&mut self, f: &mut Frame) {
        let window_width = f.area().width;
        let text_area_width = (0.8 * (window_width as f32)) as u16;

        let rect = centered_rect(f.area(), text_area_width, 3);

        f.render_widget(Clear, rect); //this clears out the background

        let text_area = self.get_text_area();
        text_area.set_block(
            Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("Edit entry: "),
            );

        text_area.set_style(
            Style::default().fg(Color::Green));

        f.render_widget(text_area.widget(), rect);
    }



    // =======================================================================
    //           INPUT HANDLING
    // =======================================================================

    fn input(&mut self, action: &mut Action, key_event: KeyEvent) {
        match self.get_input_mode() {
            InputMode::Normal => self.input_normal_mode(action, key_event),
            InputMode::Remove => self.input_remove_mode(key_event),
            InputMode::Editing => self.input_editing_mode(key_event),
        }
    }

    fn input_normal_mode(&mut self, action: &mut Action, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => *action = Action::Quit,
            KeyCode::Tab => self.toggle_focus(),
            KeyCode::Down | KeyCode::Char('j') => self.on_down(),
            KeyCode::Up | KeyCode::Char('k') => self.on_up(),
            KeyCode::Right | KeyCode::Char('l') => self.on_right(action),
            KeyCode::Left | KeyCode::Char('h') => self.on_left(action),
            KeyCode::Enter => self.on_enter(action),
            KeyCode::Char('d') => self.open_remove_mode(),
            KeyCode::Char('i') => self.open_input_mode(),
            _ => {}
        };
    }

    fn input_remove_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter | KeyCode::Char('y') => self.remove_selected(),
            KeyCode::Esc | KeyCode::Char('n') => {
                *self.get_input_mode_mut() = InputMode::Normal;
            }
            _ => {}
        };
    }

    fn input_editing_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter => {
                self.close_input_mode()
            },
            KeyCode::Esc => {
                *self.get_input_mode_mut() = InputMode::Normal;
            },
            _ => {
                self.get_text_area().input(key_event);
            }
        };
    }

    fn on_right(&mut self, action: &mut Action) {
        // do nothing if the current entry is a new entry
        if self.is_new_entry() {
            return;
        }
        // do nothing if the focus is on the info section
        if self.get_focus() == &Focus::Info {
            return;
        }
        self.action_right(action);
    }

    fn on_left(&mut self, action: &mut Action) {
        // do nothing if the focus is on the info section
        if self.get_focus() == &Focus::Info {
            return;
        }
        self.action_left(action);
    }

    fn on_up(&mut self) {
        match self.get_focus() {
            Focus::List => {
                self.get_list_counter_mut().decrement();
            }
            Focus::Info => {
                self.get_info_counter_mut().decrement();
            }
        }
    }

    fn on_down(&mut self) {
        match self.get_focus() {
            Focus::List => {
                self.get_list_counter_mut().increment();
            }
            Focus::Info => {
                self.get_info_counter_mut().increment();
            }
        }
    }

    fn on_enter(&mut self, action: &mut Action) {
        // check if the current entry is a new entry
        if self.is_new_entry() {
            self.add_new_entry();
            *self.get_focus_mut() = Focus::Info;
            self.get_info_counter_mut().reset();
            self.open_input_mode();
            return;
        }
        // otherwise, either open the input mode or perform the action
        match self.get_focus() {
            Focus::List => self.on_right(action),
            Focus::Info => self.open_input_mode(),
        }
    }

    fn toggle_focus(&mut self) {
        // do nothing if the current entry is a new entry
        if self.is_new_entry() {
            return;
        }
        match self.get_focus() {
            Focus::List => { *self.get_focus_mut() = Focus::Info; }
            Focus::Info => { *self.get_focus_mut() = Focus::List; }
        };
    }

    fn open_remove_mode(&mut self) {
        if self.is_new_entry() {
            return;
        }
        *self.get_input_mode_mut() = InputMode::Remove;
    }

    fn open_input_mode(&mut self) {
        if self.get_focus() == &Focus::List {
            return
        };
        let buffer = self.get_input_buffer();
        *self.get_text_area() = TextArea::from([buffer]);
        *self.get_input_mode_mut() = InputMode::Editing;
    }

    fn close_input_mode(&mut self) {
        let buffer = self.get_text_area().lines().join("\n");
        self.set_input_buffer(&buffer);
        *self.get_input_mode_mut() = InputMode::Normal;
    }
     
}

