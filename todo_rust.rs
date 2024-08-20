//to create a ToDo tool for productivity 
//https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=faa174f5d9ffd410d41e153d15e5d0f7

use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn display(&self) {
        let status = if self.completed { "✔️" } else { "❌" };
        println!("{} - {}", status, self.description);
    }
}

fn main() -> io::Result<()> {
    let path = "tasks.txt";
    let mut tasks = load_tasks(path)?;

    loop {
        println!("\nTo-Do List");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Complete Task");
        println!("4. Exit");
        print!("Choose an option: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                print!("Enter task description: ");
                io::stdout().flush()?;
                let mut description = String::new();
                io::stdin().read_line(&mut description)?;
                let task = Task::new(description.trim());
                tasks.push(task);
                save_tasks(path, &tasks)?;
            }
            2 => {
                for (index, task) in tasks.iter().enumerate() {
                    print!("{}: ", index + 1);
                    task.display();
                }
            }
            3 => {
                print!("Enter task number to complete: ");
                io::stdout().flush()?;
                let mut task_number = String::new();
                io::stdin().read_line(&mut task_number)?;
                let task_number: usize = match task_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if task_number > 0 && task_number <= tasks.len() {
                    tasks[task_number - 1].mark_completed();
                    save_tasks(path, &tasks)?;
                }
            }
            4 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }

    Ok(())
}

fn load_tasks(path: &str) -> io::Result<Vec<Task>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path)?;
    let mut tasks = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" - ").collect();
        if parts.len() == 2 {
            let completed = parts[0] == "✔️";
            let description = parts[1].to_string();
            let mut task = Task {
                description,
                completed,
            };
            tasks.push(task);
        }
    }
    Ok(tasks)
}

fn save_tasks(path: &str, tasks: &[Task]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    for task in tasks {
        let status = if task.completed { "✔️" } else { "❌" };
        writeln!(file, "{} - {}", status, task.description)?;
    }
    Ok(())
}


Output;
To-Do List
1. Add Task
2. List Tasks
3. Complete Task
4. Exit
Choose an option: Enter task description: 
To-Do List
1. Add Task
2. List Tasks
3. Complete Task
4. Exit
Choose an option: Enter task description: 
To-Do List
1. Add Task
2. List Tasks
3. Complete Task
4. Exit
Choose an option: Enter task description: 
To-Do List
1. Add Task
2. List Tasks
3. Complete Task
4. Exit
Choose an option: 1: ❌ - hackathon submit
2: ❌ - apply scholarship
3: ❌ - wish Tom on his birthday

To-Do List
1. Add Task
2. List Tasks
3. Complete Task
4. Exit
Choose an option: Enter task number to complete: 
To-Do List
1. Add Task
2. List Tasks
3. Complete Task
4. Exit
Choose an option: 1: ❌ - hackathon submit
2: ✔️ - apply scholarship
3: ❌ - wish Tom on his birthday

To-Do List
1. Add Task
2. List Tasks
3. Complete Task
4. Exit
Choose an option: 
