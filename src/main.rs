use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct TodoItem {
    description: String,
    status: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    items: Vec<TodoItem>,
}

fn main() {
    // Read from JSON file
    let mut todo_list: TodoList = match read_from_file("todo.json") {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(_) => TodoList { items: vec![] },
    };

    // Print the todo list
    for item in &todo_list.items {
        println!("Description: {}", item.description);
        println!("Status: {}\n", item.status);
    }

    // Add a new todo item
    let new_item = TodoItem {
        description: String::from("Write example code"),
        status: false,
    };
    todo_list.items.push(new_item);

    // Write to JSON file
    let serialized = serde_json::to_string_pretty(&todo_list).unwrap();
    write_to_file("todo.json", serialized).expect("Failed to write to file.");

}




fn read_from_file(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_to_file(filename: &str, contents: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}


