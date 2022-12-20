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

impl Data {
    pub fn as_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match self {
            Data::String(s) => Ok(bincode::serialize(s)?),
            Data::Char(c) => Ok(bincode::serialize(c)?),
            Data::Int(i) => Ok(bincode::serialize(i)?),
            Data::Bool(b) => Ok(bincode::serialize(b)?),
            Data::Blob(b) => Ok(b.clone()),
        }
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
