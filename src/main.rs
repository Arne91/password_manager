//! Password manager

mod cmd;
mod json_db;
use tokio::select;

use crate::{
    cmd::{get_input, process_input},
    json_db::JsonDB,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Welcome to the password manager.\nEnter a command (help for more information)");

    // open database
    log::debug!("open database");
    let db = JsonDB::new("Hallo Welt", "db");

    log::debug!("db: {db:?}");

    let db = JsonDB::open("database", "db");

    log::debug!("Database: {}", db.name);
    log::debug!("Path: {:?}", db.path);
    for (idx, entry) in db.entries.iter().enumerate() {
        log::debug!("Entry {idx}: {entry:?}");
    }
    loop {
        select! {
            input = get_input() => {
                if process_input(input.as_str()).await {
                    break;
                }
            }
            else => {
                break;
            }
        }
    }

    // TODO save database and close!
}
