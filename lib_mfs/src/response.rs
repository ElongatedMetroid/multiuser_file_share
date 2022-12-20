use serde_derive::{Deserialize, Serialize};

use crate::data::Data;

#[derive(Debug, Serialize, Deserialize)]
pub struct MfsResponse {
    success: bool,
    /// Will be printed if data is none, otherwise it will be the filename of data
    message: Option<String>,
    /// Will be written to a file with the name message
    data: Option<Data>,
}

impl MfsResponse {
    pub fn new() -> Self {
        Self {
            success: true,
            message: None,
            data: None,
        }
    }
    pub fn success(&self) -> bool {
        self.success
    }
    pub fn set_success(&mut self) {
        self.success = true;
    }
    pub fn set_failure(&mut self) {
        self.success = false;
    }
    pub fn message(&self) -> &Option<String> {
        &self.message
    }
    pub fn set_message(&mut self, msg: Option<String>) {
        self.message = msg;
    }
    pub fn data(&self) -> &Option<Data> {
        &self.data
    }
    pub fn set_data(&mut self, data: Option<Data>) {
        self.data = data;
    }
}
