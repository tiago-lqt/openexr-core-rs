// src/test/OpenEXRCoreTest/read.cpp
// testReadBadFiles()

mod shared;

use assert_matches::assert_matches;
use openexr_core::contexts::initializer::ContextInitializer;
use openexr_core::{ExrError, ExrResult};
use shared::errors::silent_error_callback;
#[test]
fn test_read_bad_files() -> ExrResult<()> {
    // exr_context_t             f;  # ctx
    // std::string               fn; # filename

    // std::string   fn = ILM_IMF_TEST_IMAGEDIR;
    // fn += "invalid.exr";
    let file_path = shared::get_test_file_path("invalid.exr");

    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;
    let init = ContextInitializer::default()
        .with_error_handler(Some(silent_error_callback));

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_FILE_BAD_HEADER, exr_start_read (&f, fn.c_str (), &cinit));

        let res = openexr_core::read_with_init(file_path, init.clone());
        assert_matches!(res, Err(ExrError::FileBadHeader));
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_FILE_ACCESS,
        //     exr_test_file_header ("somenonexistentexrfile.exr", &cinit));

        let res = openexr_core::read_with_init(
            "somenonexistentexrfile.exr",
            init.clone(),
        );
        assert_matches!(res, Err(ExrError::FileAccess));
    }

    Ok(())
}
