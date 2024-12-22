// src/test/OpenEXRCoreTest/read.cpp
// testReadScans()

mod shared;

use assert_matches::assert_matches;
use openexr_core::attributes::values::Box2i;
use openexr_core::contexts::initializer::ContextInitializer;
use openexr_core::contexts::reader::ReaderContext;
use openexr_core::{ExrError, ExrResult};
use shared::errors::silent_error_callback;

fn read_test_file() -> ExrResult<ReaderContext> {
    let init = ContextInitializer::default()
        .with_error_handler(Some(silent_error_callback));
    let file_path = shared::get_test_file_path("v1.7.test.interleaved.exr");
    let ctx = openexr_core::read_with_init(&file_path, init.clone())?;

    Ok(ctx)
}

#[test]
fn test_read_file() -> ExrResult<()> {
    // exr_context_t             f;
    // std::string               fn    = ILM_IMF_TEST_IMAGEDIR;

    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;
    let init = ContextInitializer::default()
        .with_error_handler(Some(silent_error_callback));

    // fn += "v1.7.test.interleaved.exr";
    let file_path = shared::get_test_file_path("v1.7.test.interleaved.exr");

    // EXRCORE_TEST_RVAL (exr_start_read (&f, fn.c_str (), &cinit));
    let _ctx = openexr_core::read_with_init(&file_path, init.clone())?;

    Ok(())
}

#[test]
fn test_get_chunk_count() -> ExrResult<()> {
    let ctx = read_test_file()?;

    // int32_t ccount;
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

    Ok(())
}
#[test]
fn test_get_scanlines_per_chunk() -> ExrResult<()> {
    let ctx = read_test_file()?;

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

    Ok(())
}
#[test]
fn test_get_data_window() -> ExrResult<()> {
    let ctx = read_test_file()?;

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

    Ok(())
}
#[test]
fn test_read_scanline_chunk_info() -> ExrResult<()> {
    let ctx = read_test_file()?;

    // exr_chunk_info_t cinfo;
    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_MISSING_CONTEXT_ARG,
        //     exr_read_scanline_chunk_info (NULL, 0, 42, &cinfo));

        // Not applicable in rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
        //     exr_read_scanline_chunk_info (f, -1, 42, &cinfo));

        // Not applicable in rust
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
        //     exr_read_scanline_chunk_info (f, 1, 42, &cinfo));
        let res = ctx.get_part(1)?.scanline_chunk_info(42);
        assert_matches!(res, Err(ExrError::ArgumentOutOfRange));
    }

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT,
        //     exr_read_scanline_chunk_info (f, 0, 42, NULL));

        // Not applicable in rust
    }
    Ok(())
}
#[test]
fn test_read_tile_chunk_info() -> ExrResult<()> {
    let ctx = read_test_file()?;

    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_TILE_SCAN_MIXEDAPI,
    //     exr_read_tile_chunk_info (f, 0, 4, 2, 0, 0, &cinfo));

    Ok(())
}
#[test]
fn test_read_scanline_chunk_info_2() -> ExrResult<()> {
    let ctx = read_test_file()?;

    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT,
    //     exr_read_scanline_chunk_info (f, 0, dw.min.y - 1, &cinfo));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT,
    //     exr_read_scanline_chunk_info (f, 0, dw.max.y + 1, &cinfo));
    // EXRCORE_TEST_RVAL (exr_read_scanline_chunk_info (f, 0, dw.min.y, &cinfo));

    Ok(())
}
#[test]
fn test_get_chunk_unpacked_size() -> ExrResult<()> {
    let ctx = read_test_file()?;

    // uint64_t pchunksz = 0;
    // EXRCORE_TEST_RVAL (exr_get_chunk_unpacked_size (f, 0, &pchunksz));
    // EXRCORE_TEST (cinfo.type == EXR_STORAGE_SCANLINE);
    // EXRCORE_TEST (cinfo.compression == EXR_COMPRESSION_NONE);
    // EXRCORE_TEST (cinfo.packed_size == pchunksz);
    // EXRCORE_TEST (cinfo.unpacked_size == pchunksz);
    // EXRCORE_TEST (cinfo.sample_count_data_offset == 0);
    // EXRCORE_TEST (cinfo.sample_count_table_size == 0);

    Ok(())
}
#[test]
fn test_decoding_initialize() -> ExrResult<()> {
    let ctx = read_test_file()?;

    {
        // exr_decode_pipeline_t decoder;
        // EXRCORE_TEST_RVAL (exr_decoding_initialize (f, 0, &cinfo, &decoder));
    }

    Ok(())
}
#[test]
fn test_decoder_channels() -> ExrResult<()> {
    let ctx = read_test_file()?;

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

    Ok(())
}
#[test]
fn test_read_scans() -> ExrResult<()> {
    let ctx = read_test_file()?;

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

    Ok(())
}
#[test]
fn test_decoding_choose_default_routines() -> ExrResult<()> {
    let ctx = read_test_file()?;

    {
        // EXRCORE_TEST_RVAL (exr_decoding_choose_default_routines (f, 0, &decoder));
    }

    Ok(())
}
#[test]
fn test_decoding_run() -> ExrResult<()> {
    let ctx = read_test_file()?;

    {
        // EXRCORE_TEST_RVAL (exr_decoding_run (f, 0, &decoder));
    }

    Ok(())
}
#[test]
fn test_decoder_results() -> ExrResult<()> {
    let ctx = read_test_file()?;

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

    Ok(())
}
#[test]
fn test_decoding_destroy() -> ExrResult<()> {
    let ctx = read_test_file()?;

    // EXRCORE_TEST_RVAL (exr_decoding_destroy (f, &decoder));

    Ok(())
}
#[test]
fn test_decoding_initialize_2() -> ExrResult<()> {
    let ctx = read_test_file()?;

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

    Ok(())
}
#[test]
fn test_decoding_choose_default_routines_2() -> ExrResult<()> {
    let ctx = read_test_file()?;

    // EXRCORE_TEST_RVAL (exr_decoding_choose_default_routines (f, 0, &decoder));

    Ok(())
}
#[test]
fn test_decoding_run_2() -> ExrResult<()> {
    let ctx = read_test_file()?;

    {
        // EXRCORE_TEST_RVAL (exr_decoding_run (f, 0, &decoder));
    }

    Ok(())
}
#[test]
fn test_decoding_destroy_3() -> ExrResult<()> {
    let ctx = read_test_file()?;

    {
        // EXRCORE_TEST_RVAL (exr_decoding_destroy (f, &decoder));
    }

    // exr_finish (&f);

    Ok(())
}
