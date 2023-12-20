//! module to process commands from user

use rpassword::read_password;

/// Process commands
pub enum ProcessCmds {
    /// Exit
    Exit,
    /// Master
    Master,
    /// Help
    Help,
    /// Invalid
    Invalid,
}

/// Get input of user
pub(crate) async fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    println!();
    buffer.to_ascii_lowercase()
}

async fn get_password() -> String {
    println!("Enter Password: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let password = read_password().unwrap();
    password
}

impl ProcessCmds {
    async fn list_commands() {
        println!("Following commands are supported:\n--------------");
        println!("exit");
        println!("master");
        println!("help");
        println!();
    }
}

impl From<&str> for ProcessCmds {
    fn from(value: &str) -> Self {
        match value {
            "exit" => Self::Exit,
            "master" => Self::Master,
            "help" => Self::Help,
            _ => Self::Invalid,
        }
    }
}

/// Process input
pub(crate) async fn process_input(input: &str) -> bool {
    let input = ProcessCmds::from(input.trim());
    // process input
    match input {
        ProcessCmds::Exit => {
            println!("Program is terminated.");
            true
        }
        ProcessCmds::Master => {
            let passwd = get_password().await;
            log::debug!("password: {passwd}");
            false
        }
        ProcessCmds::Help => {
            ProcessCmds::list_commands().await;
            false
        }
        ProcessCmds::Invalid => {
            println!("unknown command");
            false
        }
    }
}
