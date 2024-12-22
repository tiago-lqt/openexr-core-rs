use std::ffi::c_void;

#[derive(Debug)]
pub struct UserData {
    inner: *mut c_void,
}

impl From<*mut c_void> for UserData {
    fn from(value: *mut c_void) -> Self {
        UserData { inner: value }
    }
}
