use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    added_time: u64,
    done: bool,
}

fn get_tasks_file_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not find the home directory");
    let mut file_path = PathBuf::from(home_dir);
    file_path.push(".rustytasks"); // Hidden file in the home directory
    file_path
}

fn load_tasks() -> Vec<Task> {
    let file_path = get_tasks_file_path();
    if let Ok(data) = fs::read_to_string(file_path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let file_path = get_tasks_file_path();
    let data = serde_json::to_string(tasks).expect("Failed to serialize tasks");
    fs::write(file_path, data).expect("Failed to write tasks to file");
}

fn show_welcome() {
    println!("Welcome to RustyBrain!");
    println!("I remember things so you don't have to.");
    println!("Commands list:");
    println!("  rustybrain add <task> - Add a new task");
    println!("  rustybrain view - View your tasks");
    println!("  rustybrain edit <task_number> <edited_task> - Edit a saved task");
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

    let command = if args[0] == "rustybrain" || args[0] == "rb" {
        if args.len() > 1 {
            args[1].as_str()
        } else {
            println!("Try using 'rb help' ");
            return;
        }
    } else {
        println!("Invalid command. Try 'rb help'");
        return;
    };

    match command {
        "add" => {
            if args.len() < 3 {
                println!("Try again and maybe this time mention the task you want to add.");
                return;
            }
            let description = args[2..].join(" ");
            let added_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(e) => {
                    eprintln!("Error getting system time: {:?}", e);
                    return;
                }
            };
            tasks.push(Task {
                description,
                added_time,
                done: false,
            });
            save_tasks(&tasks);
            println!("Task added! No need to remember it anymore.");
        }
        "edit" => {
            if args.len() < 4 {
                println!("Your brain must be more rusty than I imagined. Try using the command properly or try rustybrain help.");
                return;
            }

            let task_to_be_edited: usize = args[2].parse().unwrap_or(0);
            if task_to_be_edited == 0 || task_to_be_edited > tasks.len() {
                println!("I can't help you edit something that doesn't exist. But I can help you remove that rust in your brain. Try running rustybrain help.");
            }
            let edited_task = args[3..].join(" ");
            tasks[task_to_be_edited - 1].description = edited_task; //i transferred ownership bcoz
                                                                    //i dont need it anymore. add .clone() if you want to use edited_task within the same
                                                                    //scope in later time.
            save_tasks(&tasks);
            println!("Task {} has been updated.", task_to_be_edited);
        }
        "delete" => {
            if args.len() < 3 {
                println!("Maybe provide a valid task to delete?");
                return;
            }
            let task_number: usize = args[2].parse().unwrap_or(0);
            if task_number == 0 || task_number > tasks.len() {
                println!("Task doesn't exist. Are you sure you don't have a rusty brain?");
            } else {
                tasks.remove(task_number - 1);
                save_tasks(&tasks);
                println!("Task deleted! You're a little less rusty now.");
            }
        }
        "mark" => {
            if args.len() < 3 {
                println!("Can't mark a task if you don't tell me which one.");
                return;
            }
            let task_number: usize = args[2].parse().unwrap_or(0);
            if task_number == 0 || task_number > tasks.len() {
                eprintln!("Doesn't exist. Maybe you're just imagining tasks?");
            } else {
                tasks[task_number - 1].done = true;
                save_tasks(&tasks);
                println!("Task marked as done! Keep up the good work.");
            }
        }
        "view" => {
            if tasks.is_empty() {
                println!("No tasks available. Looks like your brain is clear!");
            } else {
                println!("Here's what you forgot to do:");
                for (index, task) in tasks.iter().enumerate() {
                    let status = if task.done { "✓" } else { "✗" };
                    println!("{}. {} [{}]", index + 1, task.description, status);
                }
            }
        }
        "help" => show_welcome(),
        _ => println!("Unknown command. Type 'rustybrain help' for available commands."),
    }
}
