//! module to process commands from user

use std::sync::Mutex;

use dialoguer::Input;
use lazy_static::lazy_static;
use rpassword::read_password;
use tokio::{sync::mpsc::UnboundedSender, task};

use crate::json_db::JsonDB;

#[derive(Debug)]
pub enum CmdError {
    /// Too much parameter
    TooMuchParam,
    /// Too less parameter
    TooLessParam,
    /// Wrong Command
    WrongCmd,
}

/// Process commands
pub enum ProcessCmds {
    /// Exit
    Exit,
    /// Load Database
    Load(String),
    /// Show Database
    Show,
    /// Close Database
    Close,
    /// Help
    Help,
}
/// Get input of user
pub(crate) async fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    println!("Enter command:\t");
    stdin.read_line(&mut buffer).unwrap();
    buffer.to_ascii_lowercase()
}

#[allow(dead_code)]
async fn get_password() -> String {
    println!("Enter Password: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let password = read_password().unwrap();
    password
}

pub fn list_commands() {
    println!("Following commands are supported:\n--------------");
    println!("exit\tExit the program");
    println!("load <JSON_DB>\tLoad the database <JSON_DB>");
    println!("close\tClose the database");
    println!("show\tShow the content of the database");
    println!("help\tShow this help text");
    println!();
}

lazy_static! {
    static ref DATABASE: Mutex<Option<JsonDB>> = Mutex::new(None);
}

impl TryFrom<&str> for ProcessCmds {
    type Error = CmdError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        log::debug!("value: {value}");
        let value: Vec<_> = value.split_whitespace().map(|val| val).collect();
        for val in &value {
            log::debug!("val: {val}");
        }

        match value[0] {
            "exit" => Ok(Self::Exit),
            "load" => match value.len() {
                2 => Ok(Self::Load(value[1].to_string())),
                ..=1 => Err(Self::Error::TooLessParam),
                _ => Err(Self::Error::TooMuchParam),
            },
            "close" => Ok(Self::Close),
            "show" => Ok(Self::Show),
            "help" => Ok(Self::Help),
            _ => Err(Self::Error::WrongCmd),
        }
    }
}

/// Process input
pub(crate) async fn process_input(input: &str) -> bool {
    // let input = Input::<String>::new()
    //     .with_prompt("Enter command")
    //     .interact()
    //     .unwrap();
    // let input = get_input().await;
    log::debug!("user_content: {input}");

    let input = ProcessCmds::try_from(input.trim()).unwrap();

    // process input
    match input {
        ProcessCmds::Exit => {
            println!("Program is terminated.");
            true
        }
        ProcessCmds::Load(_) => {
            let mut db = DATABASE.lock().unwrap();
            *db = Some(JsonDB::open("database", "db"));
            false
        }
        ProcessCmds::Show => {
            let db = DATABASE.lock().unwrap();
            match &*db {
                Some(db) => println!("{db}"),
                None => println!("No database was loaded"),
            }
            false
        }
        ProcessCmds::Close => {
            let mut db = DATABASE.lock().unwrap();
            *db = None;
            false
        }
        ProcessCmds::Help => {
            list_commands();
            false
        }
    }
}
