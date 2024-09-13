use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use std::path::PathBuf;
use std::u32;

/// Path to store the todos.json file
fn get_todo_file_path() -> PathBuf {
    let mut path = current_dir().expect("Could not find home directory");
    path.push(".rusty_todo");
    std::fs::create_dir_all(&path).expect("Failed to create directory for todos");
    path.push("todos.json");
    path
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

fn load_todos() -> io::Result<Vec<Todo>> {
    let path = get_todo_file_path();
    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let todos = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);
    Ok(todos)
}

fn save_todos(todos: &Vec<Todo>) -> io::Result<()> {
    let path = get_todo_file_path();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    serde_json::to_writer_pretty(file, todos)?;
    Ok(())
}

/// Simple CLI Todo Application
#[derive(Parser)]
#[command(name = "rusty_todo")]
#[command(about = "A simple CLI Todo application in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add {
        /// Title of the todo
        #[arg(short, long)]
        title: String,

        /// Description of the todo
        #[arg(short, long)]
        description: String,
    },
    /// Remove a todo by its ID
    Remove {
        /// ID of the todo to remove
        #[arg(short, long)]
        id: u32,
    },
    /// Toggle the status of a todo
    Toggle {
        // ID of the todo to toggle complete
        #[arg(short, long)]
        id: u32,
    },
    /// List all todos
    List,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, description } => {
            let mut todos = load_todos()?;
            let new_id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
            let todo = Todo {
                id: new_id,
                title,
                description,
                completed: false,
            };
            todos.push(todo);
            save_todos(&todos)?;
            println!("Added todo with ID {}", new_id);
        }
        Commands::Remove { id } => {
            let mut todos = load_todos()?;
            let initial_len = todos.len();
            todos.retain(|todo| todo.id != id);
            if todos.len() == initial_len {
                println!("No todo found with ID {}", id);
            } else {
                save_todos(&todos)?;
                println!("Removed todo with ID {}", id);
            }
        }
        Commands::Toggle { id } => {
            let mut todos = load_todos()?;
            for todo in &mut todos {
                if todo.id == id {
                    todo.completed = !todo.completed
                }
                println!("Changed the status of todo with ID {}", id);
            }
            save_todos(&todos)?;
        }
        Commands::List => {
            let todos = load_todos()?;
            if todos.is_empty() {
                println!("No todos found.");
            } else {
                for todo in todos {
                    let status = if todo.completed {
                        "✅ Completed"
                    } else {
                        "⏳ Pending"
                    };
                    println!(
                        "[{}] {}: {} [{}]",
                        todo.id, todo.title, todo.description, status
                    );
                }
            }
        }
    }

    Ok(())
}
