struct Task {
    name: String,
    completed: bool,
}

impl Task {
    fn new(name: String) -> Task {
        Task {
            name: name,
            completed: false,
        }
    }
}


fn add_task(tasks: &mut Vec<Task>, task_name: String) {
    let task = Task::new(task_name);
    println!("Task added: {}", task.name);  // Print task name before pushing
    tasks.push(task);
}


fn list_tasks(tasks: &Vec<Task>) {
    for(index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "[ x ]" } else { "[  ]" };
        println!("{}: {} {}", index + 1, status, task.name);
    }
}

fn complete_task(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
        println!("Task completed.");
    } else {
        println!("Invalid task index.");
    }
}

use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                print!("Enter task name: ");
                io::stdout().flush().unwrap();
                let mut task_name = String::new();
                io::stdin().read_line(&mut task_name).expect("Failed to read line");
                add_task(&mut tasks, task_name.trim().to_string());
            }
            "2" => list_tasks(&tasks),
            "3" => {
                print!("Enter task number to complete: ");
                io::stdout().flush().unwrap();
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).expect("Failed to read line");
                if let Ok(index) = task_number.trim().parse::<usize>() {
                    complete_task(&mut tasks, index - 1);
                } else {
                    println!("Invalid input.");
                }
            }
            "4" => break,
            _ => println!("Invalid choice, try again."),
        }
    }
}
