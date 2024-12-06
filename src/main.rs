use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json::{self, Result};
#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    order_number: usize,
    description: String,
    done: bool,
}
impl Todo {
    fn new(order_number: usize, description: &str) -> Todo {
        Todo {
            order_number,
            description: description.to_string(),
            done: false,
        }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }

    fn toggle_done(&mut self) {
        self.done = !self.done;
    }
}

fn load_tasks(filename: &str) -> Result<Vec<Todo>> {
    let file = File::open(filename);
    
    match file {
        Ok(f) => {
            let tasks: Vec<Todo> = serde_json::from_reader(f)?;
            Ok(tasks)
        },
        Err(e) => {
            println!("Error opening file: {}", e);
            Ok(Vec::new()) // Return an empty list in case of error
        }
    }
}

fn save_tasks(filename: &str, tasks: &Vec<Todo>) -> Result<()> {
    // Try to create the file, if it fails return the error
    let file = File::create(filename).expect("Failed to create file");

    
    // Serialize and write the tasks to the file
    serde_json::to_writer(file, tasks)?;

    // Return Ok if everything went smoothly
    Ok(())
}
    


fn display_tasks(tasks: &Vec<Todo>) {
    if tasks.is_empty() {
        println!("No tasks to display.");
    } else {
        for task in tasks.iter() {
            let status = if task.done { "Done" } else { "Pending" };
            println!("{}: [{}] {}", task.order_number, status, task.description);
        }
    }
}
fn main() {
    let filename = "tasks.json";
    let mut tasks = match load_tasks(filename) {
        Ok(tasks) => tasks,
        Err(_) => {
            println!("Error loading tasks. Starting with an empty list.");
            Vec::new()
        }
    };

    loop {
        println!("\nTODO APP");
        println!("1. Add Task");
        println!("2. Remove Task");
        println!("3. View Tasks");
        println!("4. Mark Task as Done");
        println!("5. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                let order_number = tasks.len() + 1; // Next order number
                println!("Enter the task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let task = Todo::new(order_number, &description.trim());
                tasks.push(task);
                save_tasks(filename, &tasks).unwrap();
            }
            2 => {
                println!("Enter task number to remove:");
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).unwrap();
                let task_number: usize = task_number.trim().parse().unwrap();
                if task_number > 0 && task_number <= tasks.len() {
                    tasks.remove(task_number - 1);
                    // Recalculate order numbers after removing a task
                    for (i, task) in tasks.iter_mut().enumerate() {
                        task.order_number = i + 1;
                    }
                    println!("Task removed.");
                    save_tasks(filename, &tasks).unwrap();
                } else {
                    println!("Invalid task number.");
                }
            }
            3 => {
                display_tasks(&tasks);
            }
            4 => {
                println!("Enter task number to mark as done:");
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number).unwrap();
                let task_number: usize = task_number.trim().parse().unwrap();
                if task_number > 0 && task_number <= tasks.len() {
                    tasks[task_number - 1].mark_done();
                    println!("Task marked as done.");
                    save_tasks(filename, &tasks).unwrap();
                } else {
                    println!("Invalid task number.");
                }
            }
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}