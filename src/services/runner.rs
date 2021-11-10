use std::process;

use chrono::prelude::*;

use crate::domain::note::Note;
use crate::services::todo_list::TodoList;
use crate::services::utils::{
    read_from_stdin,
    read_string_from_stdin,
    print_success_message
};
use crate::domain::common::Togglable;

pub struct Runner {
    todo_list: TodoList
}

impl Runner {

    pub fn new() -> Runner {
        Runner {
            todo_list: TodoList::new()
        }
    }

    pub fn run(&mut self) {
        println!("TODO LIST APP\n");
        loop {
            println!("Insert a command (\"help\" for the list!)");
            let command = read_from_stdin();
            self.execute_command(command);
        }
    }

    pub fn execute_command(&mut self, command: String) {
        match command.as_str().trim() {
            "help" => self.show_guide(),
            "list" => self.show_list(),
            "add" => self.add_note(),
            "read" => self.show_note(),
            "tag"  => self.add_tag_to_note(),
            "toggle" => self.toggle_note(),
            "add_checklist_item" => self.add_checklist_element_to_note(),
            "toggle_checklist_item" => self.toggle_checklist_element_on_note(),
            "remove" => self.remove_note(),
            "exit" => process::exit(1),
            _ => println!("Error, invalid command!")
        }
    }

    pub fn show_guide(&self) {
        let guide: &'static str = "
        TODO APP - commands:

        1) list: show the complete list of notes or a filtered ones
        (it is possible to search by description, search by tag, filter by a date range)
        
        2) add: insert a new note

        3) read: show the note details

        4) tag: add a tag on a note

        5) toggle: toggle a note status

        6) add_check_list_item: add a new item to the note checklist

        7) toggle_checklist_item: toggle a note checklist status

        8) remove: delete a note

        9) exit: stop the app execution
        ";
        println!("{}", guide);
    }
 
    pub fn show_list(&mut self) {
        println!("Do you want to apply a filter (leave empty to for applying nothing)?\n");
        let filter = read_from_stdin();
        let filtered_list = match filter.as_str().trim() {
            "description" => {
                println!("Insert the string to search:\n");
                let description = read_string_from_stdin();
                self.todo_list.search_by_description(description)
            },
            "tag" => {
                println!("Insert the string to search:\n");
                let tag = read_string_from_stdin();
                self.todo_list.search_by_tag(tag)
            },
            "date" => {
                println!("Insert the start date:\n");
                let from = read_from_stdin();
                println!("Insert the end date:\n");
                let to = read_from_stdin();
                let from = Utc.datetime_from_str(from.as_str().trim(), "%Y-%m-%d %H:%M:%S").unwrap();
                let to = Utc.datetime_from_str(to.as_str().trim(), "%Y-%m-%d %H:%M:%S").unwrap();
                self.todo_list.search_by_date(from, to)
            },
            _ => self.todo_list.all()
        };
        println!("{} notes found\n", filtered_list.len());
    
        for (index, note) in filtered_list.iter().enumerate() {
            println!("{}) {}\n", index + 1, note);
        }
    }

    pub fn add_note(&mut self) {
        println!("Add your note\n");
        println!("Insert a title:\n");
        let title = read_string_from_stdin();
        println!("Insert a description:\n");
        let description = read_string_from_stdin();
        let note = Note::new(
            title,
            description
        );
        self.todo_list.add_note(note);
        print_success_message();
    }

    pub fn show_note(&mut self) {
        println!("Enter the note id:\n");
        let index: usize = read_string_from_stdin().parse().unwrap();
        let note = self.todo_list.get_note(index - 1);
        println!("Title: {}\n", note.title);
        println!("Description: {}\n", note.description);
        println!("Inserted at: {}\n", note.inserted_at);
        println!("Updated at: {}\n", note.updated_at);
        println!("Tags: {:?}\n", note.tags);
        println!("Checklist: \n");
        for (index, point) in note.checklist.iter().enumerate() {
            println!("{} {} [{:?}]\n", index + 1, point.description, point.status);
        }
    }

    pub fn add_tag_to_note(&mut self) {
        println!("Enter the note id:\n");
        let index: usize = read_string_from_stdin().parse().unwrap();
        println!("Insert a tag\n:");
        let tag = read_string_from_stdin();
        let note = self.todo_list.get_note(index - 1);
        note.add_tag(tag);
        print_success_message();
    }

    pub fn toggle_note(&mut self) {
        println!("Check or uncheck your note");
        println!("Enter the note id:\n");
        let index: usize = read_string_from_stdin().parse().unwrap();
        let note = self.todo_list.get_note(index - 1);
        let old_status = note.status.clone();
        note.toggle();
        println!("changed from status {:?} to status {:?}", old_status, note.status);
        print_success_message();
    }

    pub fn add_checklist_element_to_note(&mut self) {
        println!("Enter the note id:\n");
        let index: usize = read_string_from_stdin().parse().unwrap();
        println!("Insert a checklist element\n:");
        let checklist_element = read_string_from_stdin();
        let note = self.todo_list.get_note(index - 1);
        note.add_checklist_element(checklist_element);
        print_success_message();
    }

    pub fn remove_note(&mut self) {
        println!("Enter the note id:\n");
        let index: usize = read_string_from_stdin().parse().unwrap();
        self.todo_list.remove_note(index - 1);
        print_success_message();
    }

    pub fn toggle_checklist_element_on_note(&mut self) {
        println!("Enter the note id:\n");
        let note_index: usize = read_string_from_stdin().parse().unwrap();
        println!("Enter the checklist item id\n:");
        let item_index: usize = read_string_from_stdin().parse().unwrap();
        let note = self.todo_list.get_note(note_index - 1);
        note.toggle_checklist_element(item_index - 1);
        print_success_message();
    }

}