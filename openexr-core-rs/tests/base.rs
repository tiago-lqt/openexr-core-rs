use std::ffi::CStr;

use openexr_core::*;
use openexr_core_sys as sys;

#[test]
pub fn version() {
    let version = openexr_core::get_library_version();

    assert_eq!(version.major, sys::OPENEXR_VERSION_MAJOR);
    assert_eq!(version.minor, sys::OPENEXR_VERSION_MINOR);
    assert_eq!(version.patch, sys::OPENEXR_VERSION_PATCH);

    let openexr_version_extra = unsafe {
        CStr::from_ptr(sys::OPENEXR_VERSION_EXTRA.as_ptr() as *const i8).to_string_lossy()
    };
    assert_eq!(version.extra, openexr_version_extra);
}

#[test]
pub fn base_errors() -> ExrResult<()> {
    assert_ne!(get_default_error_message(ExrError::OutOfMemory)?, "");
    assert_ne!(
        get_default_error_message(ExrError::MissingContextArgument)?,
        ""
    );
    assert_ne!(get_default_error_message(ExrError::InvalidArgument)?, "");
    assert_ne!(get_default_error_message(ExrError::ArgumentOutOfRange)?, "");
    assert_ne!(get_default_error_message(ExrError::FileAccess)?, "");
    assert_ne!(get_default_error_message(ExrError::FileBadHeader)?, "");
    assert_ne!(get_default_error_message(ExrError::NotOpenRead)?, "");
    assert_ne!(get_default_error_message(ExrError::NotOpenWrite)?, "");
    assert_ne!(get_default_error_message(ExrError::HeaderNotWritten)?, "");
    assert_ne!(get_default_error_message(ExrError::ReadIo)?, "");
    assert_ne!(get_default_error_message(ExrError::WriteIo)?, "");
    assert_ne!(get_default_error_message(ExrError::NameTooLong)?, "");
    assert_ne!(get_default_error_message(ExrError::MissingReqAttr)?, "");
    assert_ne!(get_default_error_message(ExrError::InvalidAttr)?, "");
    assert_ne!(get_default_error_message(ExrError::NoAttributeByName)?, "");
    assert_ne!(
        get_default_error_message(ExrError::AttributeTypeMismatch)?,
        ""
    );
    assert_ne!(
        get_default_error_message(ExrError::AttributeSizeMismatch)?,
        ""
    );
    assert_ne!(get_default_error_message(ExrError::ScanTileMixedApi)?, "");
    assert_ne!(get_default_error_message(ExrError::TileScanMixedApi)?, "");
    assert_ne!(get_default_error_message(ExrError::ModifySizeChange)?, "");
    assert_ne!(
        get_default_error_message(ExrError::AlreadyWroteAttributes)?,
        ""
    );
    assert_ne!(get_default_error_message(ExrError::UseScanDeepWrite)?, "");
    assert_ne!(get_default_error_message(ExrError::UseTileDeepWrite)?, "");
    assert_ne!(
        get_default_error_message(ExrError::UseScanNonDeepWrite)?,
        ""
    );
    assert_ne!(
        get_default_error_message(ExrError::UseTileNonDeepWrite)?,
        ""
    );
    assert_ne!(get_default_error_message(ExrError::Unknown)?, "");

    Ok(())
}

#[test]
pub fn base_limits() {
    set_default_maximum_image_size(42, 42);
    let default_image_size = get_default_maximum_image_size();
    assert_eq!(
        default_image_size,
        (42, 42),
        "Unable to set_default_maximum_image_size"
    );

    set_default_maximum_image_size(-1, -1);
    let default_image_size = get_default_maximum_image_size();
    assert_eq!(
        default_image_size,
        (42, 42),
        "Invalid request not ignored to set_default_maximum_image_size"
    );

    set_default_maximum_tile_size(128, 128);
    let default_image_size = get_default_maximum_tile_size();
    assert_eq!(
        default_image_size,
        (128, 128),
        "Unable to set_default_maximum_tile_size"
    );

    set_default_maximum_tile_size(-1, -1);
    let default_image_size = get_default_maximum_tile_size();
    assert_eq!(
        default_image_size,
        (128, 128),
        "Invalid request not ignored to set_default_maximum_image_size"
    );

    set_default_maximum_image_size(0, 0);
    set_default_maximum_tile_size(0, 0);
}

#[test]
pub fn base_debug() {
    // NOTE: Not applicable to rust
}
