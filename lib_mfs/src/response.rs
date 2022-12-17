use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MfsResponse {
    success: bool,
    message: Option<String>,
}

impl MfsResponse {
    pub fn new() -> Self {
        Self { success: true, message: None }
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
    pub fn set_message(&mut self, msg: String) {
        self.message = Some(msg);
    }
}