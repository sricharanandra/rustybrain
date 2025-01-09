use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,
    group: String, // Group the task belongs to
    priority: u8,  // Priority level (1 = High, 2 = Medium, 3 = Low)
    added_time: u64,
    done: bool, // Completion status
}

fn parse_flag_value(flag: &str, prefix: &str) -> Option<String> {
    if flag.starts_with(prefix) {
        Some(flag[prefix.len()..].to_string())
    } else {
        None
    }
}

fn get_tasks_file_path() -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not find the home directory");
    let mut file_path = PathBuf::from(home_dir);
    file_path.push(".rustytasks");
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

fn create_empty_group(group_name: &str) {
    let mut tasks = load_tasks();
    // Check if the group already exists
    if tasks.iter().any(|task| task.group == group_name) {
        println!("Group '{}' already exists.", group_name);
    } else {
        // Create a dummy task to represent the group
        tasks.push(Task {
            description: "Empty group placeholder".to_string(),
            group: group_name.to_string(),
            priority: 3,
            added_time: 0, // No specific time for an empty group
            done: false,
        });
        save_tasks(&tasks);
        println!("Group '{}' created successfully.", group_name);
    }
}

fn show_welcome() {
    println!("Welcome to rustybrain, your terminal task manager!");
    println!("Available commands:");
    println!("  add <task> [--group=<group>] [--priority=<level>]   Add a task.");
    println!("  view [--group=<group>] [--sort=<priority|time>]      View tasks.");
    println!("  --make-group=<group_name>                            Create an empty group.");
    println!("  help                                                Show this help message.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = load_tasks();

    if args.len() == 1 {
        show_welcome();
        return;
    }

    let command = &args[1];

    if command == "--make-group" {
        if args.len() < 3 {
            println!("Please provide a group name.");
            return;
        }
        let group_name = &args[2];
        create_empty_group(group_name);
        return;
    }

    let command = args[1].as_str();

    match command {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task description.");
                return;
            }

            let description = args[2].clone();
            let mut group = "default".to_string(); // Default group if not specified
            let mut priority = 3; // Default priority (low)

            for arg in &args[3..] {
                if let Some(val) = parse_flag_value(arg, "--group=") {
                    group = val;
                } else if let Some(val) = parse_flag_value(arg, "--priority=") {
                    priority = val.parse().unwrap_or(3);
                }
            }

            let added_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            tasks.push(Task {
                description,
                group: group.clone(), // Clone `group` to avoid move error
                priority,
                added_time,
                done: false,
            });
            save_tasks(&tasks);
            println!(
                "Task added to group '{}' with priority {}.",
                group, priority
            );
        }
        "view" => {
            let mut filtered_tasks = tasks.clone();
            let mut sort_by = "time".to_string(); // Change `sort_by` to `String`

            for arg in &args[2..] {
                if let Some(group) = parse_flag_value(arg, "--group=") {
                    filtered_tasks = filtered_tasks
                        .into_iter()
                        .filter(|task| task.group == group)
                        .collect();
                } else if let Some(sort) = parse_flag_value(arg, "--sort=") {
                    sort_by = sort.clone(); // `sort_by` is now a `String`, so this is fine
                }
            }

            if sort_by == "priority" {
                filtered_tasks.sort_by_key(|task| task.priority);
            } else if sort_by == "time" {
                filtered_tasks.sort_by_key(|task| task.added_time);
            }

            if filtered_tasks.is_empty() {
                println!("No tasks available.");
            } else {
                println!("Here are your tasks:");
                for (index, task) in filtered_tasks.iter().enumerate() {
                    let status = if task.done { "✓" } else { "✗" };
                    println!(
                        "{}. [{}] {} [{}] (Priority: {})",
                        index + 1,
                        task.group,
                        task.description,
                        status,
                        task.priority
                    );
                }
            }
        }
        "help" => show_welcome(),
        _ => println!("Unknown command. Type 'rustybrain help' for available commands."),
    }
}
