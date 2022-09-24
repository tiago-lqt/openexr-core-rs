use openexr_core_sys as sys;
pub use sys::Error;

use crate::ExrResult;

/// Reader is a File opened for reading by start_read(). It is safe to call
/// any methods on Reader concurrently.
pub struct Reader {
    file: sys::exr_context_t,
}

impl Reader {
    pub fn inner(&self) -> sys::exr_context_t {
        self.file
    }
}

unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

pub fn start_read(_filename: &str) -> ExrResult<Reader> {
    todo!();

    // let filename = CString::new(filename).unwrap();
    // let mut file = std::ptr::null_mut();
    // unsafe {
    //     sys::exr_error_code_t(sys::exr_start_read(
    //         &mut file,
    //         filename.as_ptr(),
    //         std::ptr::null(),
    //     ))
    //     .ok(Reader { file })
    // }
}

pub fn start_write(_filename: &str, _use_temp_file: bool) -> ExrResult<Writer> {
    todo!()
    // let filename = CString::new(filename).unwrap();
    // let mut file = std::ptr::null_mut();
    // unsafe {
    // sys::exr_error_code_t(sys::exr_start_write(
    //     &mut file,
    //     filename.as_ptr(),
    //     use_temp_file,
    //     None,
    // ))
    // .ok(Writer { file })
    // }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            let mut inner = self.inner();

            sys::exr_finish((&mut inner) as *mut sys::exr_context_t);
        };
    }
}

pub struct Writer {
    file: sys::exr_context_t,
}

impl Writer {
    pub fn inner(&self) -> sys::exr_context_t {
        self.file
    }

    pub(crate) fn _inner_mut(&mut self) -> sys::exr_context_t {
        self.file
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        unsafe {
            let mut inner = self.inner();

            sys::exr_finish((&mut inner) as *mut sys::exr_context_t);
        };
    }
}
