use std::{error, fmt};

#[derive(Debug)]
pub enum MfsStreamErrorSource {
    DataToBig((u64, u64)),
}

#[derive(Debug)]
pub struct MfsStreamError {
    pub source: MfsStreamErrorSource,
}

impl fmt::Display for MfsStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MfsStreamWriterError: {}", match self.source {
            MfsStreamErrorSource::DataToBig((max_size, attempted_size)) => format!(
                "the size of the data that was trying to be writen to the stream was to large: {}>{}", attempted_size, max_size
            ),
        })
    }
}

impl error::Error for MfsStreamError {}