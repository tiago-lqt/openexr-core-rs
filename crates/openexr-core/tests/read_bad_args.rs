// src/test/OpenEXRCoreTest/read.cpp
// testReadBadArgs()

mod shared;

use assert_matches::assert_matches;
use openexr_core::contexts::initializer::ContextInitializer;
use openexr_core::{ExrError, ExrResult};
use shared::errors::error_callback;
#[test]
fn read_bad_args() -> ExrResult<()> {
    // exr_context_t             f;  # ctx
    // std::string               fn; # filename

    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;
    let init =
        ContextInitializer::default().with_error_handler(Some(error_callback));

    // exr_set_default_memory_routines (&failable_malloc, &failable_free);
    // openexr_core::set_default_memory_routines(Some(failable_malloc), Some(failable_free))?;

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT, exr_start_read (&f, NULL, NULL));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT, exr_start_read (&f, NULL, &cinit));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_FILE_ACCESS, exr_start_read (&f, "", &cinit));

        let res = openexr_core::read_with_init("", init.clone());
        assert_matches!(res, Err(ExrError::FileAccess));
    }

    let temp_dir = shared::get_temp_dir();
    // // windows fails on directory open, where under unix you can open
    // // the directory as a file handle but not read from it
    // #ifdef _WIN32
    #[cfg(windows)]
    {
        //     EXRCORE_TEST_RVAL_FAIL (
        //         EXR_ERR_FILE_ACCESS, exr_start_read (&f, fn.c_str (), &cinit));
        let res = openexr_core::read_with_init(&temp_dir, init.clone());
        assert_matches!(res, Err(ExrError::FileAccess));
    }
    #[cfg(not(windows))]
    {
        //     EXRCORE_TEST_RVAL_FAIL (
        //         EXR_ERR_READ_IO, exr_start_read (&f, fn.c_str (), &cinit));
        todo!()
    }

    //     fn.append ("invalid.exr");
    let file_path = temp_dir.join("invalid.exr");

    {
        //     EXRCORE_TEST_RVAL_FAIL (
        //         EXR_ERR_FILE_ACCESS, exr_start_read (&f, fn.c_str (), &cinit));

        let res = openexr_core::read_with_init(&file_path, init.clone());
        assert_matches!(res, Err(ExrError::FileAccess));
    }

    {
        //     EXRCORE_TEST_RVAL_FAIL_MALLOC (
        //         EXR_ERR_OUT_OF_MEMORY, exr_start_read (&f, fn.c_str (), &cinit));

        // TODO: Implement when there is a way to make malloc fail
    }

    openexr_core::reset_default_memory_routines()?;

    Ok(())
}
