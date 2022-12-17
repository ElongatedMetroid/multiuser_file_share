use std::{error, fmt};

#[derive(Debug)]
pub enum MfsErrorSource {
    DataToBig((u64, u64)),
    UserAlreadyExists,
    UserDoesNotExist,
}

#[derive(Debug)]
pub struct MfsError {
    pub source: MfsErrorSource,
}

impl fmt::Display for MfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MfsError: {}", match self.source {
            MfsErrorSource::DataToBig((max_size, attempted_size)) => format!(
                "the size of the data that was trying to be writen to the stream was to large: {}>{}", attempted_size, max_size
            ),
            MfsErrorSource::UserAlreadyExists => String::from(
                "the user already exists"
            ),
            MfsErrorSource::UserDoesNotExist => String::from(
                "the user does not exist"
            ),
        })
    }
}

impl error::Error for MfsError {}