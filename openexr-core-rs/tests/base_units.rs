use openexr_core::*;

#[test]
fn test_base() {
    let version = openexr_core::get_library_version();

    // Hardcoded constant from OpenEXR Core
    // Matching constant COMP_EXTRA or OPENEXR_VERSION_EXTRA;
    const COMP_EXTRA: &str = "";

    assert_eq!(version.major, openexr_core_sys::OPENEXR_VERSION_MAJOR);

    assert_eq!(version.minor, openexr_core_sys::OPENEXR_VERSION_MINOR);

    assert_eq!(version.patch, openexr_core_sys::OPENEXR_VERSION_PATCH);

    assert_eq!(version.extra, COMP_EXTRA);
}

#[test]
fn test_base_errors() -> ExrResult<()> {
    {
        let error_msg = get_default_error_message(ExrError::OutOfMemory)?;
        assert_ne!(error_msg, "");
    }

    {
        let error_msg = get_default_error_message(ExrError::Unknown)?;
        assert_ne!(error_msg, "");
    }

    Ok(())
}

#[test]
fn test_base_limits() {
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
fn test_base_debug() {
    // NOTE: Not applicable to rust
}
