use std::fs::File;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub enum Data {
    /// String of characters
    String(String),
    /// Single character
    Char(char),
    /// Number without decimal values
    Int(i64),
    /// True or false value
    Bool(bool),
    /// Vector of bytes
    Blob(Vec<u8>),
}

pub trait WriteData {
    fn write_data(&mut self, data: &Data) -> Result<(), Box<dyn std::error::Error>>;
}

impl WriteData for File {
    fn write_data(&mut self, data: &Data) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;
        match data {
            Data::String(value) => write!(self, "{}", value)?,
            Data::Char(value) => write!(self, "{}", value)?,
            Data::Int(value) => write!(self, "{}", value)?,
            Data::Bool(value) => write!(self, "{}", value)?,
            Data::Blob(value) => self.write_all(&value)?,
        }

        Ok(())
    }
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Data::String(String::from(value))
    }
}

impl From<String> for Data {
    fn from(value: String) -> Self {
        Data::String(value)
    }
}

impl From<char> for Data {
    fn from(value: char) -> Self {
        Data::Char(value)
    }
}

impl From<i64> for Data {
    fn from(value: i64) -> Self {
        Data::Int(value)
    }
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        Data::Bool(value)
    }
}

impl From<Vec<u8>> for Data {
    fn from(value: Vec<u8>) -> Self {
        Data::Blob(value)
    }
}
