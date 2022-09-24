use openexr_core_sys as sys;
use thiserror::Error;

pub type ExrResult<T> = Result<T, ExrError>;

#[derive(Error, Debug)]
pub enum ExrError {
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Unknown")]
    Unknown,
}

impl From<ExrError> for sys::Error {
    fn from(error: ExrError) -> Self {
        match error {
            ExrError::OutOfMemory => sys::Error::OutOfMemory,
            ExrError::Unknown => sys::Error::Unknown,
        }
    }
}
