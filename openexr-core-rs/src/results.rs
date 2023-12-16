use openexr_core_sys as sys;

pub type ExrError = sys::Error;
pub type ExrResult<T> = Result<T, ExrError>;

pub trait OkResult {
    type Result;

    fn ok(self) -> Self::Result;
}

impl OkResult for sys::exr_result_t {
    type Result = ExrResult<()>;

    fn ok(self) -> Self::Result {
        match self.0 {
            0 => Ok(()),
            code => Err(sys::exr_error_code_t::from_value(code).into()),
        }
    }
}
