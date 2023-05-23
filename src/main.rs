use rpassword::read_password;
use std::collections::HashSet;
use std::io;
use std::io::Write;
use tokio::select;

trait DbEntryTrait {
    fn create_entry(&self) -> Result<(), String>;
    fn read_entry(&self) -> Result<(), String>;
}

enum Database {
    JsonDB(JsonDB),
}

struct JsonDB {
    name: String,
    path: String,
    entries: HashSet<Entry>,
}

impl JsonDB {
    fn new(db_path: &str) -> Self {
        let mut entries = HashSet::<Entry>::new();
        entries.insert(Entry {
            password: String::from("hallo Welt"),
            website: String::from("hallo.welt.de"),
            user: String::from("hallo@welt.de"),
        });
        Self {
            name: String::from("JsonDB"),
            path: String::from(db_path),
            entries: entries,
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
    fn path(&self) -> String {
        self.path.clone()
    }
    fn entries(&self) -> String {
        let mut output = String::new();
        for val in self.entries.iter() {
            output.push_str(format!("Website: {}, User: {}\n", val.website, val.user).as_str());
        }
        output
    }
}
impl DbEntryTrait for JsonDB {
    fn create_entry(&self) -> Result<(), String> {
        Ok(())
    }
    fn read_entry(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Entry {
    website: String,
    user: String,
    password: String,
}

impl Entry {
    fn new(website: String, user: String, password: String) -> Result<Self, String> {
        if !Self::check_password(password.clone()) {
            return Err(String::from("Wrong Password"));
        }
        if !Self::check_user(user.clone()) {
            return Err(String::from("Wrong Username"));
        }
        if !Self::check_website(website.clone()) {
            return Err(String::from("Wrong Website"));
        }
        Ok(Self {
            website,
            user,
            password,
        })
    }
    fn check_password(_password: String) -> bool {
        true
    }
    fn check_user(_user: String) -> bool {
        true
    }
    fn check_website(_website: String) -> bool {
        true
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
    env_logger::init();
    println!("Welcome to the password manager.\nEnter a command (help for more information)");

    // open database
    log::debug!("open database");
    let db = Database::JsonDB(JsonDB::new("database.json"));
    match db {
        Database::JsonDB(mut val) => {
            // Test entry. Later it should be an own test case.
            let entry = Entry::new(
                String::from("www.testsite.de"),
                String::from("HelloWorld@testsite.de"),
                String::from("SecretPassword42"),
            )
            .unwrap();
            val.entries.insert(entry);
            log::debug!("Database: {}", val.name());
            log::debug!("Path: {}", val.path());
            log::debug!("Entries:\n{}", val.entries());
        }
    }
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
    // process input 
    match input {
        Exit => {
            println!("Program is terminated.");
            std::process::exit(0);
        }
        Master => {
            let passwd = get_password().await;
            log::debug!("password: {passwd}");
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
