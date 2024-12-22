use openexr_core_sys::{exr_error_code_t, exr_result_t};

pub use openexr_core_sys::ExrError;
pub type ExrResult<T> = Result<T, ExrError>;

pub trait OkResult {
    type Result;

    fn ok(self) -> Self::Result;
}

impl OkResult for exr_result_t {
    type Result = ExrResult<()>;

    fn ok(self) -> Self::Result {
        match self.0 {
            0 => Ok(()),
            code => Err(exr_error_code_t::from_value(code).into()),
        }
    }
}
