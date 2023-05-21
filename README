# Rust Password Manager

This is a simple password manager written in Rust. It allows you to manage passwords and perform various operations using command-line input.

# Current State
Work in progress.

## Prerequisites

- Rust programming language
- Cargo package manager

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Arne91/password_manager.git
   ```

2. Navigate to the project directory:

   ```bash
   cd password_manager
   ```

3. Build the project using Cargo:

   ```bash
   cargo build
   ```

4. Run the executable:

   ```bash
   cargo run
   ```

## Usage

Upon running the program, you will be prompted to enter a command. The available commands are as follows:

- `exit`: Terminates the program.
- `master`: Prompts for a master password.
- `help`: Displays a list of available commands.

Enter the desired command and press Enter to execute it. The program will perform the corresponding action and provide any relevant output.

## Project Structure

The project consists of the following files:

- `main.rs`: Contains the main entry point of the program.
- `json_db.rs`: Defines the `JsonDB` struct and its methods for managing JSON databases.
- `process_commands.rs`: Implements the `ProcessCommands` enum and its associated methods for processing user input.
- `utils.rs`: Provides utility functions for getting user input and passwords.

## Database

The program uses a JSON database (`JsonDB`) to store password entries. The `JsonDB` is in progress.

The `JsonDB` struct implements the `DbEntryTrait` trait, which defines methods for creating and reading entries in the database.

## Password Input

To ensure password security, the program uses the `rpassword` crate to read passwords securely without displaying them on the console. The `get_password` function retrieves a password from the user.

## Asynchronous Execution

The program utilizes the Tokio asynchronous runtime (`tokio::main`) to handle asynchronous input processing. It uses the `select!` macro to await user input or exit the program.

## License

This project is licensed under the [MIT License](LICENSE).