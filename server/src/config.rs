use std::{io::{self, Read}, fs::File};

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    addr: Option<String>,
    threads: Option<usize>,
    max_data_size: Option<u64>,
    break_up_data: Option<bool>,
    user_data_filename: Option<String>,
}

impl Config {
    pub fn load(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buf = String::new();
        
        File::open(filename)?.read_to_string(&mut buf)?;
        
        Ok(toml::from_str(&buf)?)
    }
    pub fn addr(&self) -> &str {
        match self.addr {
            Some(ref addr) => addr.as_str(),
            None => "127.0.0.1:1234",
        }
    }
    pub fn threads(&self) -> usize {
        match self.threads {
            Some(threads) => threads,
            None => 8,
        }
    }
    pub fn max_data_size(&self) -> u64 {
        match self.max_data_size {
            Some(max_data_size) => max_data_size,
            None => 2000,
        }
    }
    pub fn break_up_data(&self) -> bool {
        match self.break_up_data {
            Some(break_up_data) => break_up_data,
            None => true,
        }
    }
    pub fn user_data_filename(&self) -> &str {
        match self.user_data_filename {
            Some(ref user_data_filename) => user_data_filename.as_str(),
            None => "users",
        }
    }
}