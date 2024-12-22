mod shared;

use openexr_core::contexts::initializer::ContextInitializer;
use openexr_core::ExrResult;
use shared::errors::silent_error_callback;

// src/test/OpenEXRCoreTest/read.cpp > testOpenTiles()
#[test]
fn test_open_tiles() -> ExrResult<()> {
    // exr_context_t             f;  # ctx
    // std::string               fn; # filename

    // std::string   fn = ILM_IMF_TEST_IMAGEDIR;
    // fn += "tiled.exr";

    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;
    let init = ContextInitializer::default()
        .with_error_handler(Some(silent_error_callback));
    {
        let file_path = shared::get_test_file_path("tiled.exr");

        // EXRCORE_TEST_RVAL (exr_start_read (&f, fn.c_str (), &cinit));
        let ctx = openexr_core::read_with_init(&file_path, init.clone())?;

        // exr_finish (&f);
        std::mem::drop(ctx);
    }

    {
        // fn = ILM_IMF_TEST_IMAGEDIR;
        // fn += "v1.7.test.tiled.exr";
        let file_path = shared::get_test_file_path("v1.7.test.tiled.exr");

        // EXRCORE_TEST_RVAL (exr_start_read (&f, fn.c_str (), &cinit));
        let ctx = openexr_core::read_with_init(&file_path, init.clone())?;

        // exr_finish (&f);
        std::mem::drop(ctx);
    }

    Ok(())
}
