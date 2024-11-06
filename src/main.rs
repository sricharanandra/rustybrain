use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    let mut first_launch = true;

    loop {
        if first_launch {
            println!("Welcome to RustyBrain!");
            println!("Your friendly CLI task management app.");
            println!();

            println!("Here's how to use RustyBrain:");
            println!("Type a command to get started, or type 'help' for options.");
            println!();

            first_launch = false;
        }

        println!("Enter your command:");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input");
        let input = input.trim();

        match input {
            _ if input.starts_with("add ") => {
                let task = input[4..].trim();
                if task.is_empty() {
                    println!(
                        "You need to provide a task to add! It's not rocket science... or is it?"
                    );
                } else {
                    tasks.push(task.to_string());
                    println!(
                        "\nTask added! Now your brain can relax—no need to remember it anymore!\n"
                    );
                }
            }

            _ if input.starts_with("delete") || input.starts_with("remove") => {
                if tasks.is_empty() {
                    println!("Look's your brain is empty, so there's nothing to delete!");
                } else {
                    let index_to_remove: usize = input[6..].trim().parse().unwrap();

                    if index_to_remove > tasks.len() || index_to_remove <= 0 {
                        println!("You can't delete tasks that don't exist!");
                    } else {
                        tasks.remove(index_to_remove - 1);
                        println!("\nTask deleted! You're a little less rusty now!\n")
                    }
                }
            }
            "view" => {
                println!();
                println!("Here are the things you need to do that you clearly forgot:");
                println!();

                if tasks.is_empty() {
                    println!("Looks like your task list is as empty as a rusty brain!");
                } else {
                    for (index, task) in tasks.iter().enumerate() {
                        println!("{}. {}", index + 1, task);
                    }
                }
                println!();
            }
            "help" => {
                println!();
                println!("Here's how to use RustyBrain:");
                println!();
                println!("1. add <task>: Add a new task");
                println!("2. view: View your tasks");
                println!("3. delete/remove <task_number>: Delete a task");
                println!("3. help: Show this help message");
                println!("4. exit: Exit the app");
                println!();
                continue;
            }
            "exit" => {
                println!();
                println!("Come back soon for a clearer head! Remember, don't rust away!");
                break;
            }
            _ => {
                println!(
                    "\nYour brain must be rusty, because that choice isn't even in the cards!\n"
                );
            }
        }

        println!("What's the next task for your trusty rusty brain?");
        println!();
    }
}
