#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[allow(clippy::missing_safety_doc)]
mod openexr_ffi;

mod attrs;
mod errors;
mod results;

pub use attrs::*;
pub use errors::*;
pub use openexr_ffi::*;
pub use results::*;

#[cfg(test)]
mod tests {
    use crate as sys;

    #[test]
    fn it_works() {
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        let mut extra = std::ptr::null();

        unsafe {
            sys::exr_get_library_version(
                &mut major, &mut minor, &mut patch, &mut extra,
            );
        }

        assert_eq!(major, 3);
        assert_eq!(minor, 4);
        assert_eq!(patch, 0);
    }
}
