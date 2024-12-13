use serde::{Serialize, Deserialize};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut title = String::new();
    let mut description = String::new();

    println!("Enter the title of the task:");
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read input");
    let title = title.trim().to_string();

    println!("Enter the description of the task:");
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read input");
    let description = description.trim().to_string();

    let id = if tasks.is_empty() {
        1
    } else {
        tasks.last().unwrap().id + 1
    };

    let new_task = Task::new(id, title, description);
    tasks.push(new_task);
    println!("Task added successfully!");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found!");
    } else {
        for task in tasks {
            println!(
                "ID: {}, Title: {}, Description: {}, Completed: {}",
                task.id, task.title, task.description, task.completed
            );
        }
    }
}

fn edit_task(tasks: &mut Vec<Task>) {
    println!("Enter the ID of the task to edit:");
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read input");

    if let Ok(id) = id_input.trim().parse::<u32>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            println!("Editing task: {:?}", task);
            println!("Enter new title (leave empty to keep current):");
            let mut title = String::new();
            io::stdin().read_line(&mut title).expect("Failed to read input");
            let title = title.trim();
            if !title.is_empty() {
                task.title = title.to_string();
            }

            println!("Enter new description (leave empty to keep current):");
            let mut description = String::new();
            io::stdin().read_line(&mut description).expect("Failed to read input");
            let description = description.trim();
            if !description.is_empty() {
                task.description = description.to_string();
            }

            println!("Mark as completed? (yes/no):");
            let mut completed_input = String::new();
            io::stdin()
                .read_line(&mut completed_input)
                .expect("Failed to read input");
            if completed_input.trim().eq_ignore_ascii_case("yes") {
                task.completed = true;
            }

            println!("Task updated successfully!");
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID input.");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("Enter the ID of the task to delete:");
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Failed to read input");

    if let Ok(id) = id_input.trim().parse::<u32>() {
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(pos);
            println!("Task deleted successfully!");
        } else {
            println!("Task with ID {} not found.", id);
        }
    } else {
        println!("Invalid ID input.");
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write("tasks.json", json).expect("Failed to write to file");
    println!("Tasks saved successfully!");
}

fn load_tasks() -> Vec<Task> {
    if let Ok(json) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&json).unwrap_or_else(|_| {
            println!("Failed to parse tasks file. Starting with an empty list.");
            Vec::new()
        })
    } else {
        println!("No tasks file found. Starting with an empty list.");
        Vec::new()
    }
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("\nTo-Do List Menu:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Edit Task");
        println!("4. Delete Task");
        println!("5. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => edit_task(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                save_tasks(& tasks);
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
