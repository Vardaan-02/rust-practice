use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Status {
    Deleted,
    Done,
    Pending,
}

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: Status,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Status::Deleted => "Deleted 🗑️",
            Status::Done => "Done ✅",
            Status::Pending => "Pending 🕘",
        };
        write!(f, "{}", s)
    }
}
