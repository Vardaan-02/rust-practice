mod models;
mod storage;
mod commands;

use std::path::Path;
use models::{Todo, Status};
use uuid::Uuid;
use storage::{load_from_json, save_to_json};
use commands::{delete, update, print};

fn main() {
    let path = Path::new("db.json");
    let mut todos = load_from_json(path);

    let command = std::env::args().nth(1).expect("No command provided");

    match command.as_str() {
        "add" => {
            let title = std::env::args().nth(2).expect("No title provided");
            let description = std::env::args().nth(3).expect("No description provided");
            let item = Todo {
                id: Uuid::new_v4(),
                title,
                description,
                status: Status::Pending,
            };
            todos.push(item);
            save_to_json(&todos, path);
            println!("✅ Todo added successfully");
        }

        "delete" => {
            let id_title = std::env::args().nth(2).expect("No ID or title provided");
            delete(todos, id_title, path);
        }

        "update" => {
            let id_title = std::env::args().nth(2).expect("No ID or title provided");
            let status_input = std::env::args().nth(3).expect("No status provided");

            let status = match status_input.to_lowercase().as_str() {
                "done" => Status::Done,
                "deleted" => Status::Deleted,
                "pending" => Status::Pending,
                _ => panic!("Invalid status"),
            };

            update(todos, id_title, status, path);
        }

        "show" => {
            print(&todos);
        }

        _ => {
            println!("❌ Invalid Command");
        }
    }
}
