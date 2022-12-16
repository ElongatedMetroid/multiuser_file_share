use serde::{Deserialize, Serialize};
use std::{io::Read, net::TcpStream};

use crate::error::{MfsStreamError, MfsStreamErrorSource};

pub struct MfsStreamReader {
    max_data_size: u64,
    /// Set to true if you want people to be able to send data with a size greater than max_data_size by using multiple
    /// packets
    break_up_data: bool,
}

impl MfsStreamReader {
    pub fn new(max_data_size: u64, break_up_data: bool) -> Self {
        Self {
            max_data_size,
            break_up_data,
        }
    }
    pub fn max_data_size(&self) -> u64 {
        self.max_data_size
    }
    pub fn break_up_data(&self) -> bool {
        self.break_up_data
    }
    pub fn set_max_data_size(&mut self, new: u64) {
        self.max_data_size = new;
    }
    pub fn set_break_up_data(&mut self, new: bool) {
        self.break_up_data = new;
    }
    pub fn read<T: Serialize + for<'a> Deserialize<'a>>(
        &self,
        stream: &mut TcpStream,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let mut data_size_bytes: [u8; 8] = [0; 8];
        stream.read_exact(&mut data_size_bytes)?;
        let data_size = bincode::deserialize::<u64>(&data_size_bytes)?;

        if data_size > self.max_data_size() && !self.break_up_data() {
            return Err(Box::new(
                MfsStreamError {
                    source: MfsStreamErrorSource::DataToBig((self.max_data_size(), data_size))
                }
            ));
        }

        let mut data_bytes = Vec::new();
        data_bytes.resize(data_size as usize, 0);
        stream.read_exact(&mut data_bytes)?;

        let data = bincode::deserialize::<T>(&data_bytes)?;

        Ok(data)
    }
}
