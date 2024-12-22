// src/test/OpenEXRCoreTest/read.cpp
// testReadMeta()

mod shared;

use assert_matches::assert_matches;
use openexr_core::attributes::values::Box2i;
use openexr_core::contexts::initializer::ContextInitializer;
use openexr_core::{ExrError, ExrResult};
use shared::errors::silent_error_callback;

#[test]
fn test_read_meta() -> ExrResult<()> {
    // exr_context_t f; # ctx
    // std::string   fn = ILM_IMF_TEST_IMAGEDIR;  # file_path

    // fn += "v1.7.test.1.exr";
    let file_path = shared::get_test_file_path("v1.7.test.1.exr");

    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;
    let init = ContextInitializer::default()
        .with_error_handler(Some(silent_error_callback));

    // exr_attribute_t* newattr;
    // uint32_t verflags;

    // EXRCORE_TEST_RVAL (exr_test_file_header (fn.c_str (), &cinit));
    openexr_core::test_file_header_with_init(&file_path, init.clone())?;

    // EXRCORE_TEST_RVAL (exr_start_read (&f, fn.c_str (), &cinit));
    let ctx = openexr_core::read_with_init(&file_path, init.clone())?;

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_MISSING_CONTEXT_ARG,
        //     exr_get_file_version_and_flags (NULL, NULL));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT,
        //     exr_get_file_version_and_flags (f, NULL));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL (
        //     exr_get_file_version_and_flags (f, &verflags));
        let version = ctx.file_version_and_flags()?;
        // EXRCORE_TEST (verflags == 2);
        assert_eq!(version, 2);
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_MISSING_CONTEXT_ARG,
        //     exr_get_chunk_table_offset (NULL, 0, NULL));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
        //     exr_get_chunk_table_offset (f, -1, NULL));
        let res = ctx.chunk_table_offset(-1);
        assert_matches!(res, Err(ExrError::ArgumentOutOfRange));
    }
    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
        //     exr_get_chunk_table_offset (f, 2, NULL));
        let res = ctx.chunk_table_offset(2);
        assert_matches!(res, Err(ExrError::ArgumentOutOfRange));
    }
    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT,
        //     exr_get_chunk_table_offset (f, 0, NULL));

        // Not applicable to rust
    }
    {
        // uint64_t cto;
        // EXRCORE_TEST_RVAL (
        //     exr_get_chunk_table_offset (f, 0, &cto));
        let cto = ctx.chunk_table_offset(0)?;

        // EXRCORE_TEST (cto == 331);
        assert_eq!(cto, 331);
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_NOT_OPEN_WRITE,
        //     exr_attr_declare_by_type (f, 0, "foo", "box2i", &newattr));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_NOT_OPEN_WRITE,
        //     exr_attr_declare (f, 0, "bar", EXR_ATTR_BOX2I, &newattr));

        // Not applicable to rust
    }

    {
        // int partidx;
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_NOT_OPEN_WRITE,
        //     exr_add_part (f, "beauty", EXR_STORAGE_TILED, &partidx));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_NOT_OPEN_WRITE, exr_set_longname_support (f, 0));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_NOT_OPEN_WRITE, exr_set_longname_support (f, 1));

        // Not applicable to rust
    }

    {
        // void* udata = (void*) 3;
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_MISSING_CONTEXT_ARG, exr_get_user_data (NULL, &udata));

        // Not applicable to rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT, exr_get_user_data (f, NULL));

        // Not applicable to rust
    }

    unsafe {
        // udata = (void*) 3;
        // EXRCORE_TEST_RVAL (exr_get_user_data (f, &udata));
        let user_data = ctx.get_user_data()?;

        // EXRCORE_TEST (udata == NULL);
        assert_matches!(user_data, None);
    }

    {
        let part = ctx.get_part(0)?;
        // int zlev = -2;
        // EXRCORE_TEST_RVAL (exr_get_zip_compression_level (f, 0, &zlev));
        let zlev = part.zip_compression_level()?;

        // EXRCORE_TEST (zlev == -1);
        assert_eq!(zlev, -1);
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_NOT_OPEN_WRITE, exr_set_zip_compression_level (f, 0, 4));

        // Not applicable to rust
    }

    {
        let part = ctx.get_part(0)?;
        // float dlev = -3.f;
        // EXRCORE_TEST_RVAL (exr_get_dwa_compression_level (f, 0, &dlev));
        let dlev = part.dwa_compression_level()?;
        // EXRCORE_TEST (dlev == 45.f);
        assert_eq!(dlev, 45.);
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_NOT_OPEN_WRITE, exr_set_dwa_compression_level (f, 0, 42.f));

        // Not applicable to rust
    }

    // exr_finish (&f);
    std::mem::drop(ctx);

    Ok(())
}

// src/test/OpenEXRCoreTest/read.cpp > testOpenScans()
#[test]
fn test_open_scans() -> ExrResult<()> {
    // exr_context_t             f;  # ctx
    // std::string               fn; # filename

    // std::string   fn = ILM_IMF_TEST_IMAGEDIR;
    // fn += "v1.7.test.1.exr";

    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;
    let init = ContextInitializer::default()
        .with_error_handler(Some(silent_error_callback));
    {
        let file_path = shared::get_test_file_path("v1.7.test.1.exr");

        // EXRCORE_TEST_RVAL (exr_start_read (&f, fn.c_str (), &cinit));
        let ctx = openexr_core::read_with_init(&file_path, init.clone())?;

        // exr_finish (&f);
        std::mem::drop(ctx);
    }

    {
        // fn = ILM_IMF_TEST_IMAGEDIR;
        // fn += "v1.7.test.planar.exr";
        let file_path = shared::get_test_file_path("v1.7.test.planar.exr");

        // EXRCORE_TEST_RVAL (exr_start_read (&f, fn.c_str (), &cinit));
        let ctx = openexr_core::read_with_init(&file_path, init.clone())?;

        // exr_finish (&f);
        std::mem::drop(ctx);
    }

    {
        // fn = ILM_IMF_TEST_IMAGEDIR;
        // fn += "v1.7.test.interleaved.exr";
        let file_path = shared::get_test_file_path("v1.7.test.interleaved.exr");

        // EXRCORE_TEST_RVAL (exr_start_read (&f, fn.c_str (), &cinit));
        let ctx = openexr_core::read_with_init(&file_path, init.clone())?;

        // exr_finish (&f);
        std::mem::drop(ctx);
    }

    Ok(())
}

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

// src/test/OpenEXRCoreTest/read.cpp > testReadScans()
#[test]
fn test_read_scans() -> ExrResult<()> {
    // exr_context_t             f;
    // std::string               fn    = ILM_IMF_TEST_IMAGEDIR;

    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;
    let init = ContextInitializer::default()
        .with_error_handler(Some(silent_error_callback));

    // fn += "v1.7.test.interleaved.exr";
    let file_path = shared::get_test_file_path("v1.7.test.interleaved.exr");

    // EXRCORE_TEST_RVAL (exr_start_read (&f, fn.c_str (), &cinit));
    let ctx = openexr_core::read_with_init(&file_path, init.clone())?;

    // int32_t ccount;
    {
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_MISSING_CONTEXT_ARG, exr_get_chunk_count (NULL, 0, &ccount));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_ARGUMENT_OUT_OF_RANGE, exr_get_chunk_count (f, -1, &ccount));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_ARGUMENT_OUT_OF_RANGE, exr_get_chunk_count (f, 11, &ccount));

            let res = ctx.get_part(11)?.chunk_count();
            assert_matches!(res, Err(ExrError::ArgumentOutOfRange));
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_INVALID_ARGUMENT, exr_get_chunk_count (f, 0, NULL));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL (exr_get_chunk_count (f, 0, &ccount));

            let count = ctx.get_part(0)?.chunk_count()?;
            assert_eq!(count, 155);
        }
    }

    {
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_MISSING_CONTEXT_ARG,
            //     exr_get_scanlines_per_chunk (NULL, 0, &ccount));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
            //     exr_get_scanlines_per_chunk (f, -1, &ccount));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
            //     exr_get_scanlines_per_chunk (f, 11, &ccount));
            let res = ctx.get_part(11)?.scanlines_per_chunk();
            assert_matches!(res, Err(ExrError::ArgumentOutOfRange));
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_INVALID_ARGUMENT, exr_get_scanlines_per_chunk (f, 0, NULL));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL (exr_get_scanlines_per_chunk (f, 0, &ccount));
            let count = ctx.get_part(0)?.scanlines_per_chunk()?;

            // EXRCORE_TEST (ccount == 1);
            assert_eq!(count, 1);
        }
    }

    {
        {
            // exr_attr_box2i_t dw;{
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_MISSING_CONTEXT_ARG, exr_get_data_window (NULL, 0, &dw));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_ARGUMENT_OUT_OF_RANGE, exr_get_data_window (f, -1, &dw));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_ARGUMENT_OUT_OF_RANGE, exr_get_data_window (f, 1, &dw));
            let res = ctx.get_part(1)?.data_window();
            assert_matches!(res, Err(ExrError::ArgumentOutOfRange));
        }
        {
            // EXRCORE_TEST_RVAL_FAIL (
            //     EXR_ERR_INVALID_ARGUMENT, exr_get_data_window (f, 0, NULL));

            // Not applicable in rust
        }
        {
            // EXRCORE_TEST_RVAL (exr_get_data_window (f, 0, &dw));
            let count = ctx.get_part(0)?.data_window()?;
            assert_eq!(count, Box2i::new((20, 20), (197, 174)));
        }
    }

    {
        // exr_chunk_info_t cinfo;
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_MISSING_CONTEXT_ARG,
        //     exr_read_scanline_chunk_info (NULL, 0, 42, &cinfo));
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
        //     exr_read_scanline_chunk_info (f, -1, 42, &cinfo));
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
        //     exr_read_scanline_chunk_info (f, 1, 42, &cinfo));
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT,
        //     exr_read_scanline_chunk_info (f, 0, 42, NULL));
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_TILE_SCAN_MIXEDAPI,
        //     exr_read_tile_chunk_info (f, 0, 4, 2, 0, 0, &cinfo));
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT,
        //     exr_read_scanline_chunk_info (f, 0, dw.min.y - 1, &cinfo));
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT,
        //     exr_read_scanline_chunk_info (f, 0, dw.max.y + 1, &cinfo));
        // EXRCORE_TEST_RVAL (exr_read_scanline_chunk_info (f, 0, dw.min.y, &cinfo));
    }

    {
        // uint64_t pchunksz = 0;
        // EXRCORE_TEST_RVAL (exr_get_chunk_unpacked_size (f, 0, &pchunksz));
        // EXRCORE_TEST (cinfo.type == EXR_STORAGE_SCANLINE);
        // EXRCORE_TEST (cinfo.compression == EXR_COMPRESSION_NONE);
        // EXRCORE_TEST (cinfo.packed_size == pchunksz);
        // EXRCORE_TEST (cinfo.unpacked_size == pchunksz);
        // EXRCORE_TEST (cinfo.sample_count_data_offset == 0);
        // EXRCORE_TEST (cinfo.sample_count_table_size == 0);
    }

    {
        // exr_decode_pipeline_t decoder;
        // EXRCORE_TEST_RVAL (exr_decoding_initialize (f, 0, &cinfo, &decoder));
    }

    {
        // EXRCORE_TEST (decoder.channel_count == 2);
        // EXRCORE_TEST (!strcmp (decoder.channels[0].channel_name, "R"));
        // EXRCORE_TEST (decoder.channels[0].bytes_per_element == 2);
        // EXRCORE_TEST (decoder.channels[0].data_type == EXR_PIXEL_HALF);
        // EXRCORE_TEST (decoder.channels[0].width == 178);
        // EXRCORE_TEST (decoder.channels[0].height == 1);
        // EXRCORE_TEST (decoder.channels[0].x_samples == 1);
        // EXRCORE_TEST (decoder.channels[0].y_samples == 1);
        // EXRCORE_TEST (!strcmp (decoder.channels[1].channel_name, "Z"));
        // EXRCORE_TEST (decoder.channels[1].bytes_per_element == 4);
        // EXRCORE_TEST (decoder.channels[1].data_type == EXR_PIXEL_FLOAT);
        // EXRCORE_TEST (decoder.channels[1].width == 178);
        // EXRCORE_TEST (decoder.channels[1].height == 1);
        // EXRCORE_TEST (decoder.channels[1].x_samples == 1);
        // EXRCORE_TEST (decoder.channels[1].y_samples == 1);
    }

    {
        // std::unique_ptr<uint8_t[]> rptr{new uint8_t[178 * 2]};
        // std::unique_ptr<uint8_t[]> zptr{new uint8_t[178 * 4]};
        // memset (rptr.get (), -1, 178 * 2);
        // memset (zptr.get (), -1, 178 * 4);
        // decoder.channels[0].decode_to_ptr     = rptr.get ();
        // decoder.channels[0].user_pixel_stride = 2;
        // decoder.channels[0].user_line_stride  = 2 * 178;
        // decoder.channels[1].decode_to_ptr     = zptr.get ();
        // decoder.channels[1].user_pixel_stride = 4;
        // decoder.channels[1].user_line_stride  = 4 * 178;
    }

    {
        // EXRCORE_TEST_RVAL (exr_decoding_choose_default_routines (f, 0, &decoder));
    }

    {
        // EXRCORE_TEST_RVAL (exr_decoding_run (f, 0, &decoder));
    }

    {
        // // it is compression: none
        // EXRCORE_TEST (decoder.packed_buffer == NULL);
        // // it is compression: none
        // EXRCORE_TEST (decoder.unpacked_buffer == NULL);
        // /* TODO: add actual comparison against C++ library */
        // const uint16_t* curr = reinterpret_cast<const uint16_t*> (rptr.get ());
        // const float*    curz = reinterpret_cast<const float*> (zptr.get ());
        // EXRCORE_TEST (*curr == 0);
        // EXRCORE_TEST (fabsf (*curz - 0.101991f) < 0.000001f);
    }

    {
        // EXRCORE_TEST_RVAL (exr_decoding_destroy (f, &decoder));
    }

    {
        // EXRCORE_TEST_RVAL (exr_decoding_initialize (f, 0, &cinfo, &decoder));
        // rptr.reset (new uint8_t[178 * 4]);
        // decoder.channels[0].decode_to_ptr          = rptr.get ();
        // decoder.channels[0].user_pixel_stride      = 4;
        // decoder.channels[0].user_line_stride       = 4 * 178;
        // decoder.channels[0].user_bytes_per_element = 4;
        // decoder.channels[0].user_data_type         = EXR_PIXEL_FLOAT;
        // decoder.channels[1].decode_to_ptr          = zptr.get ();
        // decoder.channels[1].user_pixel_stride      = 4;
        // decoder.channels[1].user_line_stride       = 4 * 178;
    }

    {
        // EXRCORE_TEST_RVAL (exr_decoding_choose_default_routines (f, 0, &decoder));
    }

    {
        // EXRCORE_TEST_RVAL (exr_decoding_run (f, 0, &decoder));
    }

    {
        // EXRCORE_TEST_RVAL (exr_decoding_destroy (f, &decoder));
    }

    // exr_finish (&f);

    Ok(())
}
