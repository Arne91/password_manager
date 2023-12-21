//! Password manager

mod cmd;
mod json_db;
use tokio::select;

use crate::cmd::{get_input, list_commands, process_input};

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Welcome to the password manager.\nEnter a command (help for more information)");

    list_commands();
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
}
