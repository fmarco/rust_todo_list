use std::fmt;

#[derive(Debug)]
pub enum Status {
    Doing,
    Done
}

impl Clone for Status {
    fn clone(&self) -> Self {
        match self {
            Status::Doing => Status::Doing,
            Status::Done => Status::Done
        }
    }
}

pub trait Togglable {
    fn toggle(&mut self);
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Doing => write!(f, "Doing"),
            Status::Done => write!(f, "Done")
        }
    }
}
