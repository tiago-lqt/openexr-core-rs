use openexr_core_sys as sys;
use std::{ffi::CStr, fmt::Display};

use crate::ExrResult;

pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub extra: String,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

        if !self.extra.is_empty() {
            write!(f, "-{}", self.extra)?;
        }

        Ok(())
    }
}

pub fn get_library_version() -> Version {
    let mut major = 0i32;
    let mut minor = 0i32;
    let mut patch = 0i32;
    let mut extra = std::ptr::null();

    unsafe {
        sys::exr_get_library_version(
            &mut major, &mut minor, &mut patch, &mut extra,
        );

        Version {
            major: major as u32,
            minor: minor as u32,
            patch: patch as u32,
            // Even if no extra version is not specified in the build,
            // the extra pointer always points to a valid (empty) string
            extra: CStr::from_ptr(extra).to_string_lossy().into_owned(),
        }
    }
}

pub fn get_default_error_message(
    error: impl Into<sys::ExrError>,
) -> ExrResult<String> {
    unsafe {
        let sys_error: sys::ExrError = error.into();

        let message = sys::exr_get_default_error_message(sys_error.into());

        Ok(CStr::from_ptr(message).to_string_lossy().into_owned())
    }
}

pub fn set_default_maximum_image_size(width: i32, height: i32) {
    unsafe {
        sys::exr_set_default_maximum_image_size(width, height);
    }
}

pub fn get_default_maximum_image_size() -> (i32, i32) {
    let mut width = 0;
    let mut height = 0;

    unsafe {
        sys::exr_get_default_maximum_image_size(&mut width, &mut height);
    }

    (width, height)
}

pub fn set_default_maximum_tile_size(width: i32, height: i32) {
    unsafe {
        sys::exr_set_default_maximum_tile_size(width, height);
    }
}

pub fn get_default_maximum_tile_size() -> (i32, i32) {
    let mut width = 0;
    let mut height = 0;

    unsafe {
        sys::exr_get_default_maximum_tile_size(&mut width, &mut height);
    }

    (width, height)
}
