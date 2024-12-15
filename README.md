# rustybrain

rustybrain is an archlinux package for keeping track of tasks using your CLI, helping you stay on top of your to-do list with ease. Designed to fit right into your archlinux workflow, rustybrain offers simple commands to add, view, delete, and mark tasks, with persistent storage to keep everything right where you left it.
## Features

    Add Tasks: Quickly add new tasks to your list.
    View Tasks: See a list of tasks, including completion status.
    Delete Tasks: Remove tasks easily by their number.
    Mark as Done: Check off completed tasks to stay organized.
    Persistent Storage: Tasks are saved to a JSON file so you won’t lose progress.

## Installation
### For Arch Linux Users (AUR)

  Using yay (recommended):

```yay -S rustybrain```

### Manual Build: Clone the repository and build manually with Cargo:

    git clone https://github.com/sricharanandra/rustybrain.git
    cd rustybrain
    cargo build --release
    sudo cp target/release/rustybrain /usr/local/bin
### Optional: Setting up alias

Open your terminal config file. In my case, I am using zsh so it is .zshrc.
  Add to the file  ``` alias rb = "rustybrain" ```
    and then save the config. 
Next, in your terminal, source the config file by using
    ``` source <file> ```
    for example, ``` source ~/.zshrc ```
    using this you can now use rb instead of rustybrain as your command
## Usage

rustybrain runs directly from the terminal with a single command for each action. After installation, you can manage tasks by simply typing commands like:

### View Commands List
```rustybrain help```

### Add a task
```rustybrain add "Finish Rust project"```

### View all tasks
```rustybrain view```

### Mark a task as done
```rustybrain mark 1```

### Delete a task
```rustybrain delete 1```

Each command works individually—no need to launch the app beforehand! Just type rustybrain with the command you want, and RustyBrain will handle the rest.
Examples
```
rustybrain add "Buy groceries"          # Adds "Buy groceries" to your task list
rustybrain view                         # Shows your current tasks
rustybrain mark 1                       # Marks the first task as done
rustybrain delete 2                     # Deletes the second task
```
If you have setup an alias like rb in your terminal configs, you can simply use 
``` rb add <task> ``` 
and so on...
That’s it! rustybrain aims to be a helpful companion that simplifies your workflow and keeps you on track. Enjoy a less rusty memory!
