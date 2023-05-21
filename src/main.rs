use rpassword::read_password;
use std::io;
use std::io::Write;
use tokio::select;

trait DbEntryTrait {
    fn create_entry(&self) -> Result<(), String>;
    fn read_entry(&self) -> Result<(), String>;
}

#[derive(Debug)]
enum Database {
    JsonDB(JsonDB),
}

#[derive(Debug)]
struct JsonDB {
    name: String,
    path: String,
}

impl JsonDB {
    fn new(db_path: &str) -> Self {
        Self {
            name: String::from("JsonDB"),
            path: String::from(db_path),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn path(&self) -> String {
        self.path.clone()
    }
}
impl DbEntryTrait for JsonDB {
    fn create_entry(&self) -> Result<(), String> {
        todo!();
        Ok(())
    }
    fn read_entry(&self) -> Result<(), String> {
        todo!();
        Ok(())
    }
}

async fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    println!();
    buffer.to_ascii_lowercase()
}

async fn get_password() -> String {
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    password
}

#[tokio::main]
async fn main() {
    println!("Welcome to the password manager.\nEnter a command (help for more information)");

    // open database
    let db = Database::JsonDB(JsonDB::new("cats.db"));
    match db {
        Database::JsonDB(val) => {
            println!("Database: {}", val.name());
            println!("Path: {}", val.path());
        }
    }
    //println!("db: {:?}", db);
    loop {
        select! {
            input = get_input() => {
                process_input(input.as_str()).await;
            }
            else => {
                break;
            }
        }
    }

    // close database
}

async fn process_input(input: &str) {
    use ProcessCommands::*;
    let input = ProcessCommands::from(input.trim());
    // Verarbeite die Eingabe
    match input {
        Exit => {
            println!("Das Programm wird beendet.");
            std::process::exit(0);
        }
        Master => {
            let passwd = get_password().await;
            println!("password: {passwd}");
        }
        Help => {
            ProcessCommands::list_commands().await;
        }
        Invalid => {
            println!("unknown command");
        }
    }
}

enum ProcessCommands {
    Exit,
    Master,
    Help,
    Invalid,
}

impl ProcessCommands {
    async fn list_commands() {
        println!("Following commands are supported:\n--------------");
        println!("exit");
        println!("master");
        println!("help");
        println!();
    }
}

impl From<&str> for ProcessCommands {
    fn from(value: &str) -> Self {
        match value {
            "exit" => Self::Exit,
            "master" => Self::Master,
            "help" => Self::Help,
            _ => Self::Invalid,
        }
    }
}
