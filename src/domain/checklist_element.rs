use std::fmt;

use super::common;
use super::common::Togglable;

#[derive(Debug)]
pub struct ChecklistElement {
    pub description: String,
    pub status: common::Status
}

impl ChecklistElement {
    pub fn new(description: String) -> ChecklistElement {
        ChecklistElement {
            description,
            status: common::Status::Doing,
        }
    }
}

impl fmt::Display for ChecklistElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - [{}]", self.description, self.status)
    }
}


impl Togglable for ChecklistElement {
    fn toggle(&mut self) {
        match self.status {
            common::Status::Doing => self.status = common::Status::Done,
            common::Status::Done => self.status = common::Status::Doing
        }
    }
}

impl Clone for ChecklistElement {
    fn clone(&self) -> Self {
        ChecklistElement {
            description: self.description.clone(),
            status: self.status.clone()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let checklist_element = ChecklistElement::new("Read the doc".to_string());
        assert_eq!(checklist_element.description, "Read the doc");
        assert!(matches!(checklist_element.status, common::Status::Doing));
    }

    #[test]
    fn toggle() {
        let mut checklist_element = ChecklistElement::new("Read a book".to_string());
        assert!(matches!(checklist_element.status, common::Status::Doing));
        checklist_element.toggle();
        assert!(matches!(checklist_element.status, common::Status::Done));
    }

    #[test]
    fn clone() {
        let checklist_element = ChecklistElement::new("Clone this note".to_string());
        assert!(matches!(checklist_element.status, common::Status::Doing));
        let cloned_checklist_element = checklist_element.clone();
        assert_eq!(cloned_checklist_element.description, "Clone this note");
        assert!(matches!(cloned_checklist_element.status, common::Status::Doing));
    }
}