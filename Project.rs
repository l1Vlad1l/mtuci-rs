use std::collections::HashMap;
use std::io::{self, Write};


struct TodoList {
    tasks: HashMap<String, bool>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
        }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.insert(task, false);
    }

    fn complete_task(&mut self, task: &str) {
        if let Some(is_completed) = self.tasks.get_mut(task) {
            *is_completed = true;
        }
    }

    fn remove_task(&mut self, task: &str) {
        self.tasks.remove(task);
    }

    fn print_tasks(&self) {
        for (task, is_completed) in &self.tasks {
            if *is_completed {
                println!("[x] {}", task);
            } else {
                println!("[ ] {}", task);
            }
        }
    }

    fn clear_completed(&mut self) {
        self.tasks.retain(|_, is_completed| !*is_completed);
    }

    fn count_completed(&self) -> usize {
        self.tasks.values().filter(|&&is_completed| is_completed).count()
    }

    fn count_incomplete(&self) -> usize {
        self.tasks.values().filter(|&&is_completed| !is_completed).count()
    }

    fn get_all_tasks(&self) -> Vec<String> {
        self.tasks.keys().cloned().collect()
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let mut input = String::new();

    loop {
        print!("Enter a command (add, complete, remove, print, clear, completed, incomplete, all, quit): ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "add" => {
                print!("Enter a task to add: ");
                io::stdout().flush().unwrap();

                input.clear();
                io::stdin().read_line(&mut input).unwrap();

                todo_list.add_task(input.trim().to_string());
            }
            "complete" => {
                print!("Enter the task to complete: ");
                io::stdout().flush().unwrap();

                input.clear();
                io::stdin().read_line(&mut input).unwrap();

                todo_list.complete_task(input.trim());
            }
            "remove" => {
                print!("Enter the task to remove: ");
                io::stdout().flush().unwrap();

                input.clear();
                io::stdin().read_line(&mut input).unwrap();

                todo_list.remove_task(input.trim());
            }
            "print" => {
                todo_list.print_tasks();
            }
            "clear" => {
                todo_list.clear_completed();
            }
            "completed" => {
                let count = todo_list.count_completed();
                println!("Number of completed tasks: {}", count);
            }
            "incomplete" => {
                let count = todo_list.count_incomplete();
                println!("Number of incomplete tasks: {}", count);
            }
            "all" => {
                let tasks = todo_list.get_all_tasks();
                println!("All tasks:");
                for task in tasks {
                    println!("{}", task);
                }
            }
            "quit" => break,
            _ => {
                println!("Invalid command");
            }
        }
    }
}