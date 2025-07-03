use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use crate::models::Todo;

pub fn save_to_json(todos: &Vec<Todo>, path: &Path) {
    let json = serde_json::to_string_pretty(&todos).expect("Serialization failed");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open file");
    file.write_all(json.as_bytes()).expect("Write failed");
}

pub fn load_from_json(path: &Path) -> Vec<Todo> {
    if let Ok(file) = File::open(path) {
        serde_json::from_reader(file).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}
