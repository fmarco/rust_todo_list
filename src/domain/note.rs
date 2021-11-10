use std::fmt;

use chrono::DateTime;
use chrono::Utc;
use super::checklist_element;
use super::common;
use super::common::Togglable;

#[derive(Debug)]
pub struct Note {
    pub title: String,
    pub description: String,
    pub status: common::Status,
    pub inserted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub checklist: Vec<checklist_element::ChecklistElement>,
    pub tags: Vec<String>
}


impl Note {
    pub fn new(title: String, description: String) -> Note {
        Note {
            title,
            description,
            status: common::Status::Doing,
            inserted_at: Utc::now(),
            updated_at: Utc::now(),
            checklist: Vec::new(),
            tags: Vec::new()
        }
    }

    pub fn add_checklist_element(&mut self, description: String) {
        self.checklist.push(checklist_element::ChecklistElement::new(description));
    }

    pub fn toggle_checklist_element(&mut self, position: usize) {
        self.checklist[position].toggle();
    }

    pub fn remove_checklist_element(&mut self, position: usize) {
        self.tags.remove(position);
    }

    pub fn add_tag(&mut self, title: String) {
        self.tags.push(title);
    }

    pub fn has_tag(&self, tag: &String) -> bool {
        self.tags.contains(tag)
    }

    pub fn remove_tag(&mut self, position: usize) {
        self.tags.remove(position);
    }

}

impl Togglable for Note {
    fn toggle(&mut self) {
        match self.status {
            common::Status::Doing => {
                self.status = common::Status::Done;
                self.updated_at = Utc::now()
            },
            common::Status::Done => {
                self.status = common::Status::Doing;
                self.updated_at = Utc::now()
            }
        }
    }
}

impl Clone for Note {
    fn clone(&self) -> Self {
        Note {
            title: self.title.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
            inserted_at: self.inserted_at.clone(),
            updated_at: self.updated_at.clone(),
            checklist: self.checklist.clone(),
            tags: self.tags.clone()
        }
    }
}


impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - [{}]", self.title, self.status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let note = Note::new("New note".to_string(), "this is an important note".to_string());
        assert_eq!(note.title, "New note");
        assert_eq!(note.description, "this is an important note");
        assert!(matches!(note.status, common::Status::Doing));
    }

    #[test]
    fn toggle() {
        let mut note = Note::new("Note to be toggled".to_string(), "toggle this note!".to_string());
        assert!(matches!(note.status, common::Status::Doing));
        note.toggle();
        assert!(matches!(note.status, common::Status::Done));
    }

    #[test]
    fn add_tag() {
        let mut note = Note::new("Note with tag".to_string(), "tag it!".to_string());
        assert_eq!(note.tags.len(), 0);
        note.add_tag("Food".to_string());
        assert_eq!(note.tags.len(), 1);
        assert_eq!(note.tags[0], "Food");
    }

    #[test]
    fn toggle_checklist_element() {
        let mut note = Note::new("Note with checklist item to be toggled".to_string(), "toggle inside!".to_string());
        assert!(matches!(note.status, common::Status::Doing));
        note.add_checklist_element("Point 1".to_string());
        note.toggle_checklist_element(0);
        assert!(matches!(note.checklist[0].status, common::Status::Done));
    }
}