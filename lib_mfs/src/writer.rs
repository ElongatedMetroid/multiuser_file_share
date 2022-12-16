use serde::{Deserialize, Serialize};
use std::{io::Write, mem, net::TcpStream};

use crate::error::{MfsStreamError, MfsStreamErrorSource};

/// Write data to a stream in which a MfsTcpReader can understand
/// The data will be writen in this format "data_size(8 bytes)data_bytes(data_size bytes)}"
pub struct MfsStreamWriter {
    /// Max data size of packets
    max_data_size: u64,
    /// Will the packet be broke up if the data's size you try to write is greater than max_data_size
    break_up_data: bool,
}

impl MfsStreamWriter {
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
    /// Writes a string to stream, this needs to be used so that the reciver of this write will know information about the
    /// data before reading it, such as the lenght of the data
    pub fn write<T: Serialize + for<'a> Deserialize<'a>>(
        &self,
        stream: &mut TcpStream,
        data: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize into a vector of bytes
        let mut data_bytes = bincode::serialize(&data)?;
        let data_size = mem::size_of_val(&data_bytes[..]) as u64;

        // If the size of the data is greater than the max_data_size and breaking up that data if set to false, return an error
        if data_size > self.max_data_size() && !self.break_up_data() {
            return Err(Box::new(MfsStreamError {
                source: MfsStreamErrorSource::DataToBig((self.max_data_size(), data_size)),
            }));
        }

        // Serialize the data size into a vector of bytes
        let mut data_size_bytes = bincode::serialize(&data_size)?;

        let mut data: Vec<u8> = Vec::new();
        data.append(&mut data_size_bytes);
        data.append(&mut data_bytes);

        stream.write_all(&data)?;

        Ok(())
    }
}
