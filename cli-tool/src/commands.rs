use std::path::Path;

use crate::{models::{Todo, Status}, storage::save_to_json};

pub fn print(todos: &Vec<Todo>) {
    for todo in todos {
        println!("{} | {} | {} | {}", todo.id, todo.title, todo.description, todo.status);
    }
}

pub fn delete(mut todos: Vec<Todo>, id_title: String, path: &Path) {
    let initial_len = todos.len();
    todos.retain(|todo| !(todo.id.to_string() == id_title || todo.title.trim() == id_title.trim()));

    if initial_len == todos.len() {
        println!("⚠️ No such todo exists with ID or title");
    } else {
        println!("🗑️ Deleted 1 item");
        save_to_json(&todos, path);
    }
}

pub fn update(mut todos: Vec<Todo>, id_title: String, status: Status, path: &Path) {
    if let Some(todo) = todos.iter_mut().find(|t| t.id.to_string() == id_title || t.title.trim() == id_title.trim()) {
        todo.status = status;
        println!("✅ Todo updated successfully");
        save_to_json(&todos, path);
    } else {
        println!("⚠️ No such todo exists with ID or title");
    }
}
