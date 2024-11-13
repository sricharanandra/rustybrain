use std::env;

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        display_welcome_message();
    } else {
        match args[1].as_str() {
            "add" => add_task(&args[2..].join(" "), &mut tasks),
            "delete" | "remove" => delete_task(&args[2..], &mut tasks),
            "view" => view_tasks(&tasks),
            "help" => display_help(),
            _ => println!("Unknown command. Type 'rustybrain help' for options."),
        }
    }
}

fn display_welcome_message() {
    println!("Welcome to RustyBrain!");
    println!("Your friendly CLI task management app.");
    println!();
    println!("Here's how to use RustyBrain:");
    println!("Use one of the following commands to get started:");
    println!("  rustybrain add <task>      - Add a new task");
    println!("  rustybrain view            - View all tasks");
    println!("  rustybrain delete <number> - Delete a task by number");
    println!("  rustybrain help            - Show this help message");
    println!();
    println!("Come back soon for a clearer head! Remember, don't rust away!");
}

fn display_help() {
    println!("RustyBrain Help:");
    println!();
    println!("Commands:");
    println!("  add <task>           - Add a new task");
    println!("  view                 - View all tasks");
    println!("  delete <task_number> - Delete a task by number");
    println!("  help                 - Show this help message");
    println!("  exit                 - Exit the app");
}

fn add_task(task: &str, tasks: &mut Vec<String>) {
    if task.trim().is_empty() {
        println!("You need to provide a task to add! It's not rocket science... or is it?");
    } else {
        tasks.push(task.to_string());
        println!("Task added! Now your brain can relax—no need to remember it anymore!");
    }
}

fn delete_task(args: &[String], tasks: &mut Vec<String>) {
    if tasks.is_empty() {
        println!("Looks like your brain is empty, so there's nothing to delete!");
        return;
    }

    if args.is_empty() {
        println!("Please specify the task number to delete.");
        return;
    }

    match args[0].parse::<usize>() {
        Ok(index) if index > 0 && index <= tasks.len() => {
            tasks.remove(index - 1);
            println!("Task deleted! You're a little less rusty now!");
        }
        _ => println!("You can't delete tasks that don't exist!"),
    }
}

fn view_tasks(tasks: &[String]) {
    println!();
    if tasks.is_empty() {
        println!("Looks like your task list is as empty as a rusty brain!");
    } else {
        println!("Here are the things you need to do that you clearly forgot:");
        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
    }
    println!();
}
