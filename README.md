# Rust CLI Todo Application

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/Rust-1.60+-brightgreen.svg)

A simple and efficient Command-Line Interface (CLI) Todo application built in Rust. Manage your tasks directly from the terminal with ease—add, remove, list, and mark todos as complete with straightforward commands.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building the Project](#building-the-project)
- [Usage](#usage)
  - [Add a Todo](#add-a-todo)
  - [Remove a Todo](#remove-a-todo)
  - [Mark a Todo as Complete](#mark-a-todo-as-complete)
  - [List Todos](#list-todos)
- [Project Structure](#project-structure)
- [Enhancements](#enhancements)
- [License](#license)

## Features

- **Add a Todo:** Easily add new tasks with a title and description.
- **Remove a Todo:** Delete tasks by specifying their unique ID.
- **List Todos:** View all your tasks with their status (Pending or Completed).
- **Mark as Complete:** Update the status of tasks to indicate completion.
- **Persistent Storage:** Todos are saved in a JSON file located in your home directory.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust Installed:** You need to have Rust installed on your system. If you haven't installed Rust yet, follow the instructions below.

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo:** Rust’s package manager, Cargo, is installed alongside Rust. You can verify the installation by running:

  ```bash
  cargo --version
  ```

- **Basic Knowledge of Rust:** Familiarity with Rust's syntax and Cargo (Rust’s package manager) will be beneficial.

## Installation

1. **Clone the Repository:**

   Open your terminal and clone the repository to your local machine:

   ```bash
   git clone https://github.com/yourusername/todo_cli.git
   cd todo_cli
   ```

2. **Install Dependencies:**

   Ensure that all the necessary dependencies are specified in `Cargo.toml`. The primary dependencies include:

   - `clap` for parsing command-line arguments.
   - `serde` and `serde_json` for data serialization and deserialization.
   - `dirs` for handling directory paths across different operating systems.

   If you've followed the project setup steps previously, these dependencies should already be included. Otherwise, verify your `Cargo.toml`:

   ```toml
   [package]
   name = "todo_cli"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   clap = { version = "4.1.8", features = ["derive"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   dirs = "4.0"
   ```

## Building the Project

1. **Navigate to the Project Directory:**

   Ensure you're in the root directory of the project:

   ```bash
   cd todo_cli
   ```

2. **Build the Application:**

   Use Cargo to build the project in release mode for optimized performance:

   ```bash
   cargo build --release
   ```

   The compiled binary will be located at `target/release/todo_cli`.

3. **(Optional) Install the Application Globally:**

   To use the application from anywhere in your system, install it globally using Cargo:

   ```bash
   cargo install --path .
   ```

   This command installs the binary to Cargo’s bin directory, typically `~/.cargo/bin`. Ensure this directory is included in your system's `PATH` environment variable.

## Usage

Once built or installed, you can use the `todo_cli` application to manage your todos. Below are the available commands and usage examples.

### Add a Todo

Add a new todo by providing a title and description.

```bash
todo_cli add --title "Buy Groceries" --description "Milk, Eggs, Bread, and Butter"
```

**Short Flags:**

```bash
todo_cli add -t "Read Book" -d "Finish reading 'The Rust Programming Language'"
```

**Output:**

```
Added todo with ID 1
```

### Remove a Todo

Remove an existing todo by specifying its unique ID.

```bash
todo_cli remove --id 1
```

**Short Flag:**

```bash
todo_cli remove -i 2
```

**Output:**

```
Removed todo with ID 1.
```

**If the ID does not exist:**

```
No todo found with ID 3.
```

### Mark a Todo as Complete

Update the status of a todo to indicate it has been completed.

```bash
todo_cli complete --id 1
```

**Short Flag:**

```bash
todo_cli complete -i 2
```

**Output:**

```
Marked todo with ID 1 as completed.
```

**If the todo is already completed:**

```
Todo with ID 1 is already completed.
```

**If the ID does not exist:**

```
No todo found with ID 2.
```

### List Todos

Display all existing todos with their IDs, titles, descriptions, and statuses.

```bash
todo_cli list
```

**Output:**

```
[1] Buy Groceries: Milk, Eggs, Bread, and Butter [⏳ Pending]
[2] Read Book: Finish reading 'The Rust Programming Language' [✅ Completed]
```

**If no todos are found:**

```
No todos found.
```

### Help Information

To view help information and see available commands and options:

```bash
todo_cli --help
```

**Sample Output:**

```
Simple CLI Todo Application

USAGE:
    todo_cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add       Add a new todo
    complete  Mark a todo as complete
    help      Print this message or the help of the given subcommand(s)
    list      List all todos
    remove    Remove a todo by its ID
```

## Project Structure

Here's an overview of the project's structure:

```
todo_cli/
├── Cargo.toml
├── README.md
└── src
    └── main.rs
```

- **Cargo.toml:** Contains project metadata and dependencies.
- **README.md:** Documentation and usage instructions.
- **src/main.rs:** The main source code file implementing the CLI Todo application.

## Enhancements

While the current application provides essential functionality, here are some suggestions for further improvements:

1. **Update a Todo:**

   Implement an `edit` command to modify the title or description of existing todos.

2. **Toggle Completion Status:**

   Allow users to toggle a todo's completion status between complete and incomplete.

3. **Delete Completed Todos:**

   Provide a command to remove all todos that have been marked as completed.

4. **Prioritize Todos:**

   Add a priority level to todos (e.g., High, Medium, Low) and allow sorting based on priority.

5. **Search and Filter:**

   Enable searching for todos by keywords or filtering by completion status.

6. **Color-Coded Output:**

   Use crates like `colored` or `termion` to display colored output for better readability.

7. **Due Dates:**

   Incorporate due dates for todos and notify users of upcoming or overdue tasks.

8. **Persistent Storage Enhancements:**

   Switch from JSON to a more robust storage solution like SQLite for better scalability and querying capabilities.

9. **Integration with Task Management Tools:**

   Integrate with tools like Trello or Todoist for synchronized task management across platforms.

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. **Fork the Repository**

2. **Create a Feature Branch**

   ```bash
   git checkout -b feature/YourFeature
   ```

3. **Commit Your Changes**

   ```bash
   git commit -m "Add Your Feature"
   ```

4. **Push to the Branch**

   ```bash
   git push origin feature/YourFeature
   ```

5. **Open a Pull Request**

## License

This project is licensed under the [MIT License](LICENSE).

---

Feel free to customize this `README.md` to better fit your project's specifics, such as updating the repository URL, adding screenshots, or including additional sections as needed.
