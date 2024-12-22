use openexr_core::ExrError;
use openexr_core_sys as sys;
use std::convert::TryInto;
use std::os::raw::c_char;

pub unsafe extern "C" fn error_callback(
    _context: sys::exr_const_context_t,
    code: sys::exr_result_t,
    _msg: *const c_char,
) {
    let error: ExrError = code.try_into().unwrap();
    println!("err_cb ERROR {}", error);
}

pub unsafe extern "C" fn silent_error_callback(
    _context: sys::exr_const_context_t,
    _code: sys::exr_result_t,
    _msg: *const c_char,
) {
}
