//! JSON database

use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Display, fs::File, io::Read, path::Path};

/// JSON database structure
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonDB {
    /// Name of Database
    pub name: String,
    /// Path where the database is located
    pub path: Box<Path>,
    /// entries
    pub entries: HashSet<Entry>,
}

impl Display for JsonDB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Database: {}", self.name).unwrap();
        writeln!(f, "Path: {:?}", self.path).unwrap();
        writeln!(f).unwrap();
        self.entries.iter().for_each(|entry| {
            writeln!(f, "{}", entry).unwrap();
        });

        Ok(())
    }
}

impl JsonDB {
    /// Create a new database with name and a path where the databases are
    pub fn new(name: &str, db_pathes: &str) -> Self {
        let entries = HashSet::<Entry>::new();
        let db_path = name.to_ascii_lowercase().replace(' ', "_");
        let db_path_with_extension = format!("{}/{}.json", db_pathes, db_path);
        let db_path = Path::new(&db_path_with_extension);

        Self {
            name: String::from(name),
            path: db_path.into(),
            entries: entries,
        }
    }

    /// Open a data with its name
    pub fn open(name: &str, db_pathes: &str) -> Self {
        let mut db = Self::new(name, db_pathes);
        let mut file =
            File::open(db.path.clone()).expect(&format!("Failed to open file \"{:?}\"", db.path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("Failed to read file \"{:?}\"", db.path));
        db = serde_json::from_str(&contents).expect(&format!(
            "Failed to deserialize JSON file \"{:?}\"",
            db.path
        ));
        db
    }

    #[allow(dead_code)]
    fn entries(&self) -> String {
        let mut output = String::new();
        for val in self.entries.iter() {
            output.push_str(format!("Website: {}, User: {}\n", val.website, val.user).as_str());
        }
        output
    }
}
impl JsonDB {
    #[allow(dead_code)]
    fn create_entry(&self) -> Result<(), String> {
        todo!();
    }
    fn read_entry(&self) -> Result<(), String> {
        todo!();
    }
}

/// One entry of the database
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entry {
    /// Website
    pub website: String,
    /// User
    pub user: String,
    /// Password from the user
    pub password: String,
}
impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.website).unwrap();
        writeln!(f, "User: {}", self.user).unwrap();
        writeln!(f, "Password: {}", self.password).unwrap();

        Ok(())
    }
}

impl Entry {
    #[allow(dead_code)]
    pub fn new(website: String, user: String, password: String) -> Result<Self, String> {
        if !Self::check_password(&password) {
            return Err(String::from("Wrong Password"));
        }
        if !Self::check_user(&user) {
            return Err(String::from("Wrong Username"));
        }
        if !Self::check_website(&website) {
            return Err(String::from("Wrong Website"));
        }
        Ok(Self {
            website,
            user,
            password,
        })
    }
    fn check_password(_password: &str) -> bool {
        true
    }
    fn check_user(_user: &str) -> bool {
        true
    }
    fn check_website(_website: &str) -> bool {
        true
    }
}
