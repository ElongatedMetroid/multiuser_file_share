use std::{
    io::{Read, Write},
    mem,
    net::TcpStream,
};

use serde::{Deserialize, Serialize};

/// Write data to a stream in which a MfsTcpReader can understand
/// The data will be writen in this format "data_size(8 bytes)data_bytes(data_size bytes)}"
pub struct MfsStreamWriter;

impl MfsStreamWriter {
    /// Writes a string to stream, this needs to be used so that the reciver of this write will know information about the
    /// data before reading it, such as the lenght of the data
    pub fn write<T: Serialize + for<'a> Deserialize<'a>>(
        stream: &mut TcpStream,
        data: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize into a vector of bytes
        let data_bytes = bincode::serialize(&data)?;
        // Get the size of the data
        let data_size = mem::size_of_val(&data_bytes[..]) as u64;

        // Write 8 byte (64 bit) packet to the stream containing
        stream.write_all(&bincode::serialize(&data_size)?)?;
        stream.write_all(&data_bytes)?;

        Ok(())
    }
}

pub struct MfsStreamReader;

impl MfsStreamReader {
    pub fn read<T: Serialize + for<'a> Deserialize<'a>>(
        stream: &mut TcpStream,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let mut data_size_bytes: [u8; 8] = [0; 8];
        stream.read_exact(&mut data_size_bytes)?;
        let data_size = bincode::deserialize::<u64>(&data_size_bytes)?;

        let mut data_bytes = Vec::new();
        data_bytes.resize(data_size as usize, 0);
        stream.read_exact(&mut data_bytes)?;

        let data = bincode::deserialize::<T>(&data_bytes)?;

        Ok(data)
    }
}
