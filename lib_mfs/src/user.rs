use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use serde_derive::{Deserialize, Serialize};

use crate::error::{MfsError, MfsErrorSource};

#[derive(Debug, Serialize, Deserialize)]
pub struct MfsUser {
    username: String,
    password: String,
}

impl MfsUser {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MfsUsers {
    file_name: String,
    users: HashMap<String, String>,
}

impl MfsUsers {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: String::from(file_name),
            users: HashMap::new(),
        }
    }
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::open(&self.file_name)?;

        f.write_all(&bincode::serialize(self)?)?;

        Ok(())
    }
    pub fn add_user(&mut self, user: MfsUser) -> Result<(), Box<dyn std::error::Error>> {
        // If the user you tried to add was already there return an error
        if let Some(_) = self.users.insert(user.username, user.password) {
            return Err(Box::new(MfsError {
                source: MfsErrorSource::UserAlreadyExists,
            }));
        }

        Ok(())
    }
    pub fn contains_user(&mut self, user: &MfsUser) -> bool {
        self.users.contains_key(&user.username)
    }
    pub fn correct_password(&mut self, user: &MfsUser) -> Result<bool, Box<dyn std::error::Error>> {
        match self.users.get(&user.username) {
            Some(password) => return Ok(*password == user.password),
            None => {
                return Err(Box::new(MfsError {
                    source: MfsErrorSource::UserDoesNotExist,
                }))
            }
        }
    }
}

impl TryFrom<&str> for MfsUsers {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut f = File::open(value)?;

        let mut users_bytes = Vec::new();
        f.read_to_end(&mut users_bytes)?;
        let users = bincode::deserialize::<MfsUsers>(&users_bytes)?;

        Ok(users)
    }
}
