use crate::contexts::initializer::ContextInitializer;
use crate::contexts::reader::ReaderContext;
use crate::{ExrError, ExrResult, OkResult};

use openexr_core_sys as sys;
use openexr_core_sys::exr_context_initializer_t;

use std::ffi::CString;
use std::path::Path;

pub fn read(file_name: impl AsRef<Path>) -> ExrResult<ReaderContext> {
    let path = file_name.as_ref().to_str().unwrap_or_default();
    // TODO: Figure out better way to handle this error
    let file_name = CString::new(path).map_err(|_| ExrError::FileAccess)?;
    start_read_internal(file_name, std::ptr::null())
}

pub fn read_with_init(
    file_name: impl AsRef<Path>,
    init: ContextInitializer,
) -> ExrResult<ReaderContext> {
    let path = file_name.as_ref().to_str().unwrap_or_default();
    // TODO: Figure out better way to handle this error
    let file_name = CString::new(path).map_err(|_| ExrError::FileAccess)?;
    let initalizer = init.into();

    start_read_internal(file_name, &initalizer)
}

pub fn test_file_header(file_name: impl AsRef<Path>) -> ExrResult<()> {
    let path = file_name.as_ref().to_str().unwrap_or_default();
    // TODO: Figure out better way to handle this error
    let file_name = CString::new(path).map_err(|_| ExrError::FileAccess)?;
    test_file_header_internal(file_name, std::ptr::null())
}

pub fn test_file_header_with_init(
    file_name: impl AsRef<Path>,
    init: ContextInitializer,
) -> ExrResult<()> {
    let path = file_name.as_ref().to_str().unwrap_or_default();
    // TODO: Figure out better way to handle this error
    let file_name = CString::new(path).map_err(|_| ExrError::FileAccess)?;
    let initalizer = init.into();

    test_file_header_internal(file_name, &initalizer)
}

fn start_read_internal(
    file_name: CString,
    initalizer: *const exr_context_initializer_t,
) -> ExrResult<ReaderContext> {
    let mut context = std::ptr::null_mut();

    unsafe {
        sys::exr_start_read(&mut context, file_name.as_ptr(), initalizer)
    }
    .ok()?;

    let writer = ReaderContext::from_context(context);

    Ok(writer)
}

fn test_file_header_internal(
    file_name: CString,
    initalizer: *const exr_context_initializer_t,
) -> ExrResult<()> {
    unsafe { sys::exr_test_file_header(file_name.as_ptr(), initalizer) }
        .ok()?;

    Ok(())
}
