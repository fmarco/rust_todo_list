use chrono::prelude::*;

use crate::domain::note::Note;

#[derive(Debug)]
pub struct TodoList {
    notes: Vec<Note>
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            notes: Vec::new()
        }
    }

    pub fn count(&self) -> usize {
        self.notes.len()
    }
 
    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn remove_note(&mut self, position: usize) {
        self.notes.remove(position);
    }

    pub fn get_note(&mut self, position: usize) -> &mut Note {
        &mut self.notes[position]
    }

    pub fn all(&self) -> Vec<Note> {
        self.notes.clone()
    }

    pub fn search_by_tag(&self, tag: String) -> Vec<Note> {
        self.all().into_iter().filter(|note| note.has_tag(&tag)).collect()
    }

    pub fn search_by_description(&self, description: String) -> Vec<Note> {
        self.all().into_iter().filter(|note| note.description.contains(&description)).collect()
    }

    pub fn search_by_date(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Vec<Note> {
        self.all().into_iter().filter(|note| (note.inserted_at >= from) && (note.inserted_at <= to)).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_note() {
        let mut todo_list = TodoList::new();
        assert_eq!(todo_list.notes.len(), 0);
        let note = Note::new("New note".to_string(), "this is an important note".to_string());
        todo_list.add_note(note);
        assert_eq!(todo_list.notes.len(), 1);
    }

    #[test]
    fn remove_note() {
        let mut todo_list = TodoList::new();
        let note = Note::new("Old note".to_string(), "this is note to remove".to_string());
        todo_list.add_note(note);
        assert_eq!(todo_list.notes.len(), 1);
        todo_list.remove_note(0);
        assert_eq!(todo_list.notes.len(), 0);
    }

    #[test]
    fn get_note() {
        let mut todo_list = TodoList::new();
        let note = Note::new("Old note".to_string(), "this is note to remove".to_string());
        todo_list.add_note(note);
        assert_eq!(todo_list.notes.len(), 1);
        let note = todo_list.get_note(0);
        assert_eq!(note.title, "Old note");
        assert_eq!(note.description, "this is note to remove");
    }

    #[test]
    fn search_by_tag() {
        let mut todo_list = TodoList::new();
        let mut note = Note::new("Tagged".to_string(), "this is tagged note".to_string());
        note.add_tag("Rust".to_string());
        todo_list.add_note(note);
        let filtered_notes = todo_list.search_by_tag("Rust".to_string());
        assert_eq!(filtered_notes.len(), 1);
        assert_eq!(filtered_notes[0].title, "Tagged");
        let filtered_notes = todo_list.search_by_tag("Python".to_string());
        assert_eq!(filtered_notes.len(), 0);
    }

    #[test]
    fn search_by_description() {
        let mut todo_list = TodoList::new();
        let note = Note::new("Filtered".to_string(), "this is note filtered".to_string());
        todo_list.add_note(note);
        let filtered_notes = todo_list.search_by_description("filtered".to_string());
        assert_eq!(filtered_notes.len(), 1);
        assert_eq!(filtered_notes[0].title, "Filtered");
        let filtered_notes = todo_list.search_by_description("Somethingelse".to_string());
        assert_eq!(filtered_notes.len(), 0);
    }

    #[test]
    fn search_by_date() {
        let mut todo_list = TodoList::new();
        let mut note = Note::new("Filtered by date".to_string(), "this is note filtered".to_string());
        note.inserted_at = Utc.datetime_from_str("2021-10-09 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        todo_list.add_note(note);
        let from = Utc.datetime_from_str("2021-10-08 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let to = Utc.datetime_from_str("2021-10-12 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let filtered_notes = todo_list.search_by_date(from, to);
        assert_eq!(filtered_notes.len(), 1);
        assert_eq!(filtered_notes[0].title, "Filtered by date");
        let mut todo_list = TodoList::new();
        let mut note = Note::new("Filtered by date wrong".to_string(), "this is note filtered".to_string());
        note.inserted_at = Utc.datetime_from_str("2021-08-09 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        todo_list.add_note(note);
        let filtered_notes = todo_list.search_by_date(from, to);
        assert_eq!(filtered_notes.len(), 0);
    }
}