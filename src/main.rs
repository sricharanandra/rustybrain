use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    added_time: u64,
    done: bool,
}

fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("rustytasks").unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write("rustytasks", data).expect("Failed to write tasks to file");
}

fn show_welcome() {
    println!("Welcome to RustyBrain!");
    println!("Your friendly CLI task management app.");
    println!("Usage:");
    println!("  rustybrain add <task> - Add a new task");
    println!("  rustybrain view - View your tasks");
    println!("  rustybrain delete <task_number> - Delete a task");
    println!("  rustybrain mark <task_number> - Mark a task as done");
    println!("  rustybrain help - Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = load_tasks();

    if args.len() == 1 {
        show_welcome();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task description to add.");
                return;
            }
            let description = args[2..].join(" ");
            let added_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            tasks.push(Task {
                description,
                added_time,
                done: false,
            });
            save_tasks(&tasks);
            println!("Task added! No need to remember it anymore.");
        }
        "delete" => {
            if args.len() < 3 {
                println!("Please provide the task number to delete.");
                return;
            }
            let task_number: usize = args[2].parse().unwrap_or(0);
            if task_number == 0 || task_number > tasks.len() {
                println!("Invalid task number.");
            } else {
                tasks.remove(task_number - 1);
                save_tasks(&tasks);
                println!("Task deleted! You’re a little less rusty now.");
            }
        }
        "mark" => {
            if args.len() < 3 {
                println!("Please provide the task number to mark as done.");
                return;
            }
            let task_number: usize = args[2].parse().unwrap_or(0);
            if task_number == 0 || task_number > tasks.len() {
                println!("Invalid task number.");
            } else {
                tasks[task_number - 1].done = true;
                save_tasks(&tasks);
                println!("Task marked as done!");
            }
        }
        "view" => {
            if tasks.is_empty() {
                println!("No tasks available. Looks like your brain is clear!");
            } else {
                println!("Your tasks:");
                for (index, task) in tasks.iter().enumerate() {
                    let status = if task.done { "✓ " } else { "✗ " };
                    println!("{}. {} [{}]", index + 1, task.description, status);
                }
            }
        }
        "help" => show_welcome(),
        _ => println!("Unknown command. Type 'rustybrain help' for available commands."),
    }
}
