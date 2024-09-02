# Task Manager CLI Tool:: 

The Task Manager is a simple command-line interface (CLI) tool written in Rust that helps users manage their to-do list. The tool allows users to add tasks, list all tasks, and mark tasks as completed. It is a beginner-friendly project designed to demonstrate basic concepts in Rust, including struct usage, method implementation, and control flow.


### 1. **Struct Definition**

```rust
struct Task {
    name: String,
    completed: bool,
}
```

- **`struct Task`**: This defines a custom data structure `Task`, which is like a blueprint for creating task objects. Each `Task` has two fields:
  - **`name: String`**: This stores the name of the task as a string.
  - **`completed: bool`**: This is a boolean that indicates whether the task is completed (`true`) or not (`false`).

### 2. **Implementing Methods for `Task`**

```rust
impl Task {
    fn new(name: String) -> Task {
        Task {
            name,
            completed: false,
        }
    }
}
```

- **`impl Task`**: This block is used to implement methods associated with the `Task` struct.
- **`fn new(name: String) -> Task`**: This is a constructor method that creates a new `Task` instance.
  - It takes a string `name` as an argument and returns a `Task` with that name.
  - The `completed` field is initialized to `false`, meaning the task starts as incomplete.

### 3. **Function to Add a Task**

```rust
fn add_task(tasks: &mut Vec<Task>, task_name: String) {
    let task = Task::new(task_name);
    println!("Task added: {}", task.name);  // Print task name before pushing
    tasks.push(task);
}
```

- **`fn add_task`**: This function adds a new task to the list.
  - **`tasks: &mut Vec<Task>`**: This is a mutable reference to a vector of `Task` objects. The vector will store all tasks.
  - **`task_name: String`**: This is the name of the task to be added.
  - **`let task = Task::new(task_name);`**: This creates a new `Task` using the provided name.
  - **`println!("Task added: {}", task.name);`**: This line prints the name of the task that was added.
  - **`tasks.push(task);`**: Finally, the new task is added to the vector of tasks.

### 4. **Function to List All Tasks**

```rust
fn list_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "[x]" } else { "[ ]" };
        println!("{}: {} {}", index + 1, status, task.name);
    }
}
```

- **`fn list_tasks`**: This function lists all the tasks along with their statuses.
  - **`tasks: &Vec<Task>`**: This is an immutable reference to the vector of tasks.
  - **`for (index, task) in tasks.iter().enumerate()`**: This loop iterates over the tasks, with `index` being the position in the vector and `task` being the task itself.
  - **`let status = if task.completed { "[x]" } else { "[ ]" };`**: This determines the status of the task. If it's completed, the status is `[x]`; otherwise, it's `[ ]`.
  - **`println!("{}: {} {}", index + 1, status, task.name);`**: This prints the task number, status, and name.

### 5. **Function to Complete a Task**

```rust
fn complete_task(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
        println!("Task completed.");
    } else {
        println!("Invalid task index.");
    }
}
```

- **`fn complete_task`**: This function marks a task as completed.
  - **`tasks: &mut Vec<Task>`**: This is a mutable reference to the vector of tasks.
  - **`index: usize`**: This is the index of the task to be completed.
  - **`if let Some(task) = tasks.get_mut(index)`**: This tries to get a mutable reference to the task at the specified index. If the index is valid, the task is retrieved.
  - **`task.completed = true;`**: This marks the task as completed.
  - **`println!("Task completed.");`**: This prints a confirmation message.
  - **`else` block**: If the index is invalid, an error message is printed.

### 6. **Main Function**

```rust
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
```

- **`fn main()`**: This is the entry point of the program.
  - **`let mut tasks: Vec<Task> = Vec::new();`**: This creates a new, empty vector to store tasks.
  - **`loop { ... }`**: This infinite loop repeatedly displays the menu and processes user input until the user chooses to exit.
  - **`println!("1. Add Task"); ...`**: These lines display the menu options to the user.
  - **`print!("Choose an option: ");`**: This prompts the user to choose an option.
  - **`io::stdout().flush().unwrap();`**: This ensures the prompt is displayed immediately by flushing the output buffer.
  - **`let mut choice = String::new();`**: This creates a string to store the user's input.
  - **`io::stdin().read_line(&mut choice)`**: This reads the user's input from the console.
  - **`match choice.trim()`**: This matches the user's input against the possible menu options:
    - **`"1"`**: If the user chooses to add a task, the program prompts for the task name and adds it to the list.
    - **`"2"`**: If the user chooses to list tasks, the program displays all tasks.
    - **`"3"`**: If the user chooses to complete a task, the program prompts for the task number and marks it as completed.
    - **`"4"`**: If the user chooses to exit, the loop breaks, and the program ends.
    - **`_`**: If the input doesn't match any option, an error message is displayed, and the menu is shown again.
