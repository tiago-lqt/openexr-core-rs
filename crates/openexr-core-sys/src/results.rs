use crate::errors::ExrError;
pub use crate::openexr_ffi::*;
use std::convert::TryFrom;
use thiserror::Error;

impl From<ExrError> for exr_result_t {
    fn from(error: ExrError) -> exr_result_t {
        let error_code: exr_error_code_t = error.into();
        exr_result_t(error_code as i32)
    }
}

#[derive(Error, Debug)]
pub enum ExrResultErrorConvertError {
    #[error("Result is success, cannot convert to error")]
    Success,
}

impl TryFrom<exr_result_t> for ExrError {
    type Error = ExrResultErrorConvertError;

    fn try_from(value: exr_result_t) -> Result<Self, Self::Error> {
        if (value.0) == exr_error_code_t::EXR_ERR_SUCCESS as i32 {
            return Err(ExrResultErrorConvertError::Success);
        }

        let error = exr_error_code_t::from_value(value.0);
        Ok(error.into())
    }
}

pub type SysResult<T> = std::result::Result<T, ExrError>;
