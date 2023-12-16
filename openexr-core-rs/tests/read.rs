#![feature(c_variadic)]
#![feature(assert_matches)]

mod shared;

use anyhow::Result;

use exr::{attributes::Storage, context::Context, context::ContextPart};
use openexr_core as exr;
use openexr_core_sys as sys;

use openexr_core::{ExrError, ExrResult, Initializer, ReadOptions};
use std::{assert_matches::assert_matches, os::raw::c_char};

#[test]
fn read_scanline() -> Result<(), Box<dyn std::error::Error>> {
    let path_ferris = shared::get_test_file_path("ferris.exr");

    let reader = exr::Reader::read(&path_ferris)?;
    assert_eq!(reader.file_name()?, path_ferris);

    assert_eq!(reader.count()?, 1);
    assert_eq!(reader.name(0)?, None);
    assert_eq!(reader.storage(0)?, Storage::Scanline);
    assert_eq!(reader.tile_levels(0), Err(exr::Error::TileScanMixedApi));
    assert_eq!(
        reader.tile_sizes(0, 0, 0),
        Err(exr::Error::TileScanMixedApi)
    );

    let chunk_count = reader.chunk_count(0)?;
    let scanlines_per_chunk = reader.scanlines_per_chunk(0)?;
    let chunk_unpacked_size = reader.chunk_unpacked_size(0)?;

    let attr_count = reader.attribute_count(0)?;
    assert_eq!(attr_count, 8);

    for i in 0..attr_count {
        let attr = reader.get_attribute_by_index(0, AttrListAccessMode::FileOrder, i)?;
        println!("Attribute {} - {}", i, attr.name());
    }

    // let attr_channels = reader.get_attribute_by_name(0, "channels")?;

    // assert_eq!(reader.get_attribute::<f32>(0, "screenWindowWidth")?, 1.0f32);
    // assert_eq!(
    //     reader.get_attribute::<exr::attr::Compression>(0, "compression")?,
    //     exr::attr::Compression::Piz
    // );

    // let dw = [0, 0, 1199, 799];
    // assert_eq!(reader.get_attribute::<[i32; 4]>(0, "dataWindow")?, dw);
    // assert_eq!(reader.get_attribute::<[i32; 4]>(0, "displayWindow")?, dw);

    // assert_eq!(reader.data_window::<[i32; 4]>(0)?, dw);
    // assert_eq!(reader.display_window::<[i32; 4]>(0)?, dw);

    // assert_eq!(reader.lineorder(0)?, exr::attr::LineOrder::IncreasingY);

    // assert_eq!(reader.pixel_aspect_ratio(0)?, 1.0f32);
    // assert_eq!(reader.screen_window_width(0)?, 1.0f32);
    // assert_eq!(reader.screen_window_center::<[f32; 2]>(0)?, [0.0, 0.0]);

    // let width = dw.width() as usize + 1;
    // let height = dw.height() as usize + 1;

    // let channels_to_read = ["R", "G", "B", "A"];
    // let nchan = channels_to_read.len();
    // let channel_bytes = 2;
    // let pixel_bytes = channels_to_read.len() * channel_bytes;
    // let scanline_bytes = pixel_bytes * width;

    // let num_chunk_lines = scanlines_per_chunk * chunk_count;

    // // we need to make the storage big enough to hold all chunks, then we'll
    // // truncate at the end. Alternative would be to allocate a chunk-sized
    // // buffer and copy into the result vec as each chunk is decoded
    // let mut pixel_data = vec![f16::from_f32(0.5); width * num_chunk_lines * nchan];

    // println!("width: {}, height: {}", width, height);

    // let mut chunk_scanline_start = 0;
    // let mut chunk_scanline_end = scanlines_per_chunk;

    // let chunk_info = reader.read_scanline_chunk_info(0, chunk_scanline_start as i32)?;
    // let mut decoder = exr::decode::DecodePipeline::default();

    // reader.decoding_initialize(0, &chunk_info, &mut decoder)?;

    // while chunk_scanline_end <= num_chunk_lines {
    //     let pixel_ptr = pixel_data
    //         [chunk_scanline_start * width * nchan..chunk_scanline_end * width * nchan]
    //         .as_mut_ptr();

    //     let chunk_info = reader.read_scanline_chunk_info(0, chunk_scanline_start as i32)?;

    //     reader.decoding_update(0, &chunk_info, &mut decoder)?;

    //     let mut chan_offset = 0;
    //     for req_chan_name in &channels_to_read {
    //         for decode_channel in decoder.channels_mut() {
    //             if decode_channel.name() == *req_chan_name {
    //                 unsafe {
    //                     decode_channel.set_decode_to(pixel_ptr.offset(chan_offset) as *mut u8);
    //                 }

    //                 decode_channel.set_user_bytes_per_element(2);
    //                 decode_channel.set_user_pixel_stride(pixel_bytes);
    //                 decode_channel.set_user_line_stride(scanline_bytes);
    //                 chan_offset += 1;
    //             }
    //         }
    //     }

    //     reader.decoding_choose_default_routines(0, &mut decoder)?;
    //     unsafe { reader.decoding_run(0, &mut decoder)? };

    //     chunk_scanline_start += scanlines_per_chunk;
    //     chunk_scanline_end += scanlines_per_chunk;
    // }

    // // finished with the decoder, clean up
    // reader.decoding_destroy(decoder)?;

    // // now truncate the pixels to the correct length and convert to u8
    // // to write out a png for comparison
    // let png_data = pixel_data
    //     .into_iter()
    //     .take(width * height * nchan)
    //     .map(|c| (f32::from(c) * 255.0).floor() as u8)
    //     .collect::<Vec<_>>();

    // let f = std::fs::File::create("read_scanline.png")?;
    // let ref mut w = std::io::BufWriter::new(f);
    // let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    // encoder.set_color(png::ColorType::RGBA);
    // encoder.set_depth(png::BitDepth::Eight);
    // let mut writer = encoder.write_header()?;
    // writer.write_image_data(&png_data)?;

    // todo!();

    Ok(())
}

#[test]
fn read_bad_args() -> ExrResult<()> {
    // exr_context_t             f;
    // std::string               fn;
    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;

    let init = Initializer::default().with_error_handler(Some(error_callback));

    let _options = ReadOptions::default();

    // exr_start_write(&f, test, EXR_WRITE_FILE_DIRECTLY, &cinit);
    // let mut writer = openexr_core::Reader::start_with_init_and_options("<string>", init, options)?;

    // openexr_core::set_default_memory_routines(Some(failable_malloc), Some(failable_free))?;

    {
        // EXRCORE_TEST_RVAL_FAIL (
        //     EXR_ERR_INVALID_ARGUMENT, exr_start_read (NULL, fn.c_str (), NULL));

        let res = openexr_core::Reader::read_with_init("", init);
        assert_matches!(res, Err(ExrError::InvalidArgument));
    }

    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT, exr_start_read (&f, NULL, NULL));

    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT, exr_start_read (&f, NULL, &cinit));

    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT, exr_start_read (&f, "", &cinit));

    //     // windows fails on directory open, where under unix you can open
    //     // the directory as a file handle but not read from it
    // #ifdef _WIN32
    //     EXRCORE_TEST_RVAL_FAIL (
    //         EXR_ERR_FILE_ACCESS, exr_start_read (&f, fn.c_str (), &cinit));
    // #else
    //     EXRCORE_TEST_RVAL_FAIL (
    //         EXR_ERR_READ_IO, exr_start_read (&f, fn.c_str (), &cinit));
    // #endif

    //     fn.append ("invalid.exr");
    //     EXRCORE_TEST_RVAL_FAIL (
    //         EXR_ERR_FILE_ACCESS, exr_start_read (&f, fn.c_str (), &cinit));

    //     EXRCORE_TEST_RVAL_FAIL_MALLOC (
    //         EXR_ERR_OUT_OF_MEMORY, exr_start_read (&f, fn.c_str (), &cinit));

    openexr_core::reset_default_memory_routines()?;

    Ok(())
}

// TODO
// #[test]
// fn read_bad_files() -> Result<()> {
//     todo!()
// }

// TODO
// #[test]
// fn read_meta() -> Result<()> {
//     todo!()
// }

// TODO
// #[test]
// fn open_scans() -> Result<()> {
//     todo!()
// }

// TODO
// #[test]
// fn open_tiles() -> Result<()> {
//     todo!()
// }

// TODO
// #[test]
// fn open_multi_part() -> Result<()> {
//     todo!()
// }

// TODO
// #[test]
// fn open_deep() -> Result<()> {
//     todo!()
// }

#[test]
fn read_scans() -> ExrResult<()> {
    let file_name = shared::get_test_file_path("v1.7.test.interleaved.exr");

    // exr_context_t             f;
    // exr_context_initializer_t cinit = EXR_DEFAULT_CONTEXT_INITIALIZER;
    // cinit.error_handler_fn          = &err_cb;

    let init = Initializer::default().with_error_handler(Some(error_callback));

    // EXRCORE_TEST_RVAL(exr_start_read(&f, file_name.c_str(), &cinit));
    let _context = openexr_core::Reader::read_with_init(file_name, init)?;

    // int32_t ccount;
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_MISSING_CONTEXT_ARG, exr_get_chunk_count (NULL, 0, &ccount));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_ARGUMENT_OUT_OF_RANGE, exr_get_chunk_count (f, -1, &ccount));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_ARGUMENT_OUT_OF_RANGE, exr_get_chunk_count (f, 11, &ccount));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT, exr_get_chunk_count (f, 0, NULL));
    // EXRCORE_TEST_RVAL (exr_get_chunk_count (f, 0, &ccount));

    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_MISSING_CONTEXT_ARG,
    //     exr_get_scanlines_per_chunk (NULL, 0, &ccount));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
    //     exr_get_scanlines_per_chunk (f, -1, &ccount));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_ARGUMENT_OUT_OF_RANGE,
    //     exr_get_scanlines_per_chunk (f, 11, &ccount));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT, exr_get_scanlines_per_chunk (f, 0, NULL));
    // EXRCORE_TEST_RVAL (exr_get_scanlines_per_chunk (f, 0, &ccount));
    // EXRCORE_TEST (ccount == 1);

    // exr_attr_box2i_t dw;
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_MISSING_CONTEXT_ARG, exr_get_data_window (NULL, 0, &dw));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_ARGUMENT_OUT_OF_RANGE, exr_get_data_window (f, -1, &dw));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_ARGUMENT_OUT_OF_RANGE, exr_get_data_window (f, 1, &dw));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT, exr_get_data_window (f, 0, NULL));
    // EXRCORE_TEST_RVAL (exr_get_data_window (f, 0, &dw));

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

    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_TILE_SCAN_MIXEDAPI,
    //     exr_read_tile_chunk_info (f, 0, 4, 2, 0, 0, &cinfo));

    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT,
    //     exr_read_scanline_chunk_info (f, 0, dw.min.y - 1, &cinfo));
    // EXRCORE_TEST_RVAL_FAIL (
    //     EXR_ERR_INVALID_ARGUMENT,
    //     exr_read_scanline_chunk_info (f, 0, dw.max.y + 1, &cinfo));
    // EXRCORE_TEST_RVAL (exr_read_scanline_chunk_info (f, 0, dw.min.y, &cinfo));

    // uint64_t pchunksz = 0;
    // EXRCORE_TEST_RVAL (exr_get_chunk_unpacked_size (f, 0, &pchunksz));
    // EXRCORE_TEST (cinfo.type == EXR_STORAGE_SCANLINE);
    // EXRCORE_TEST (cinfo.compression == EXR_COMPRESSION_NONE);
    // EXRCORE_TEST (cinfo.packed_size == pchunksz);
    // EXRCORE_TEST (cinfo.unpacked_size == pchunksz);
    // EXRCORE_TEST (cinfo.sample_count_data_offset == 0);
    // EXRCORE_TEST (cinfo.sample_count_table_size == 0);

    // exr_decode_pipeline_t decoder;
    // EXRCORE_TEST_RVAL (exr_decoding_initialize (f, 0, &cinfo, &decoder));

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

    // std::unique_ptr<uint8_t[]> rptr{ new uint8_t[178 * 2] };
    // std::unique_ptr<uint8_t[]> zptr{ new uint8_t[178 * 4] };
    // memset (rptr.get (), -1, 178 * 2);
    // memset (zptr.get (), -1, 178 * 4);
    // decoder.channels[0].decode_to_ptr     = rptr.get ();
    // decoder.channels[0].user_pixel_stride = 2;
    // decoder.channels[0].user_line_stride  = 2 * 178;
    // decoder.channels[1].decode_to_ptr     = zptr.get ();
    // decoder.channels[1].user_pixel_stride = 4;
    // decoder.channels[1].user_line_stride  = 4 * 178;

    // EXRCORE_TEST_RVAL (exr_decoding_choose_default_routines (f, 0, &decoder));

    // EXRCORE_TEST_RVAL (exr_decoding_run (f, 0, &decoder));

    // // it is compression: none
    // EXRCORE_TEST (decoder.packed_buffer == NULL);
    // // it is compression: none
    // EXRCORE_TEST (decoder.unpacked_buffer == NULL);
    // /* TODO: add actual comparison against C++ library */
    // const uint16_t* curr = reinterpret_cast<const uint16_t*> (rptr.get ());
    // const float*    curz = reinterpret_cast<const float*> (zptr.get ());
    // EXRCORE_TEST (*curr == 0);
    // EXRCORE_TEST (fabsf (*curz - 0.101991f) < 0.000001f);

    // EXRCORE_TEST_RVAL (exr_decoding_destroy (f, &decoder));

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

    // EXRCORE_TEST_RVAL (exr_decoding_choose_default_routines (f, 0, &decoder));

    // EXRCORE_TEST_RVAL (exr_decoding_run (f, 0, &decoder));

    // EXRCORE_TEST_RVAL (exr_decoding_destroy (f, &decoder));

    // exr_finish (&f);

    todo!()
}

// TODO
// #[test]
// fn read_tiles() -> Result<()> {
//     todo!()
// }

// TODO
// #[test]
// fn read_multi_part() -> Result<()> {
//     todo!()
// }

// TODO
// #[test]
// fn read_unpack() -> Result<()> {
//     todo!()
// }

unsafe extern "C" fn error_callback(
    _context: sys::exr_const_context_t,
    _code: sys::exr_result_t,
    _format: *const c_char,
) {
    todo!()
}

// #[test]
// fn read_bad_args() {
//     let path = "invalid_non_existant.exr";

//     assert_eq!(
//         start_read(&path).err().expect("Expected error"),
//         Error::FileAccess
//     );
// }

// #[test]
// fn read_bad_files() {
//     let path = get_test_file_path("invalid.exr");

//     assert!(start_read(&path).is_err());
// }

// #[test]
// fn open_scans() {
// TODO: Implement after adding error callback support

// exr_file_t *f;
// std::string fn = ILM_IMF_TEST_IMAGEDIR;
// fn += "v1.7.test.1.exr";
// int rv;
// rv = exr_start_read( &f, fn.c_str(), &error_callback );
// assert( rv == EXR_ERR_SUCCESS );
// exr_close( &f );

// fn = ILM_IMF_TEST_IMAGEDIR;
// fn += "v1.7.test.planar.exr";
// rv = exr_start_read( &f, fn.c_str(), &error_callback );
// assert( rv == EXR_ERR_SUCCESS );
// exr_close( &f );

// fn = ILM_IMF_TEST_IMAGEDIR;
// fn += "v1.7.test.interleaved.exr";
// rv = exr_start_read( &f, fn.c_str(), &error_callback );
// assert( rv == EXR_ERR_SUCCESS );
// exr_close( &f );
// }

// #[test]
// fn open_tiles() {
// TODO: Implement after adding error callback support

// exr_file_t *f;
// std::string fn = ILM_IMF_TEST_IMAGEDIR;

// fn += "tiled.exr";
// int rv;
// rv = exr_start_read( &f, fn.c_str(), &error_callback );
// assert( rv == EXR_ERR_SUCCESS );
// exr_close( &f );

// fn = ILM_IMF_TEST_IMAGEDIR;
// fn += "v1.7.test.tiled.exr";
// rv = exr_start_read( &f, fn.c_str(), &error_callback );
// assert( rv == EXR_ERR_SUCCESS );
// exr_close( &f );
// }

// #[test]
// fn open_multi_part() {}

// #[test]
// fn open_deep() {}

// #[test]
// fn read_scans() {}

// #[test]
// fn read_tiles() -> Result<(), Error> {
// let file_path = get_test_file_path("v1.7.test.tiled.exr");

// let file = start_read(file_path.as_str())?;

// {
//     assert_eq!(file.part_storage(0)?, StorageType::Tiled);
// }

// {
//     assert_eq!(file.tile_levels(1), Err(Error::InvalidArgument));
// }

// {
//     let tile_levels = file.tile_levels(0)?;
//     assert_eq!(tile_levels, (1, 1));
// }

// {
//     let tile_sizes = file.tile_sizes(0, 0, 0)?;
//     assert_eq!(tile_sizes, (12, 24));
// }

// {
//     assert_eq!(
//         file.decode_chunk_init_scanline(0, 42)
//             .err()
//             .expect("Expected error"),
//         Error::ScanTileMixedApi
//     );
// }

// {
// actually read a tile...
// let decode_chunk = file.decode_chunk_init_tile(0, 4, 2, 0, 0)?;

// let chunk_unpacked_size = decode_chunk.unpacked_size();
// let file_unpacked_size = file.chunk_unpacked_size(0)?;
// assert_eq!(chunk_unpacked_size, file_unpacked_size);
// assert_eq!(decode_chunk.channel_count(), 2);

// let chunk_channels = decode_chunk.channels();
// assert_ne!(chunk_channels[0].channel_name()?, "G");
// assert_eq!(chunk_channels[0].bytes_per_pel, 2);
// assert_eq!(chunk_channels[0].width, 12);
// assert_eq!(chunk_channels[0].height, 24);
// assert_eq!(chunk_channels[0].x_samples, 1);
// assert_eq!(chunk_channels[0].y_samples, 1);

// assert_ne!(chunk_channels[1].channel_name()?, "Z");
// assert_eq!(chunk_channels[1].bytes_per_pel, 4);
// assert_eq!(chunk_channels[1].width, 12);
// assert_eq!(chunk_channels[1].height, 24);
// assert_eq!(chunk_channels[1].x_samples, 1);
// assert_eq!(chunk_channels[1].y_samples, 1);
// }

// TODO: Convert C++ test code
// std::unique_ptr<uint8_t []> gptr
//     {
//         new uint8_t[24*12*2]
//     };
// std::unique_ptr<uint8_t []> zptr
//     {
//         new uint8_t[24*12*4]
//     };
// memset( gptr.get(), 0, 24*12*2 );
// memset( zptr.get(), 0, 24*12*4 );
// chunk.channels[0].data_ptr = gptr.get();
// chunk.channels[0].output_pixel_stride = 2;
// chunk.channels[0].output_line_stride = 2 * 12;
// chunk.channels[1].data_ptr = zptr.get();
// chunk.channels[1].output_pixel_stride = 4;
// chunk.channels[1].output_line_stride = 4 * 12;

// rv = exr_read_chunk( f, &chunk );
// assert( rv == EXR_ERR_SUCCESS );
// assert( chunk.packed.buffer != NULL );
// assert( chunk.unpacked.buffer != NULL );
// /* TODO: add actual comparison against C++ library */
// const uint16_t *curg = reinterpret_cast<const uint16_t *>( gptr.get() );
// const float *curz = reinterpret_cast<const float *>( zptr.get() );
// assert( *curg == 0x33d5 );
// assert( fabsf( *curz - 0.244778f ) < 0.000001f );
// //for ( int y = 0; y < 24; ++y )
// //{
// //    for ( int x = 0; x < 12; ++x )
// //        std::cout << ' ' << std::hex << std::setw( 4 ) << std::setfill( '0' ) << *curg++ << std::dec << " (" << *curz++ << " )";
// //    std::cout << std::endl;
// //}

// exr_destroy_decode_chunk_info( &chunk );
// exr_close( &f );

//     Ok(())
// }

// #[test]
// fn read_multi_part() {}

// #[test]
// fn read_deep() {}
