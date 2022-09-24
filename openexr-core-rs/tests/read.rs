use openexr_core::Error;

fn _get_test_file_path(filename: &str) -> String {
    //std::path::Path::new("../../openexr/src/test/OpenEXRTest/")
    std::path::Path::new("../openexr/src/test/OpenEXRTest/")
        // std::path::Path::new(
        //     option_env!("TEST_IMAGEDIR").expect("Please provide envvar 'TEST_IMAGEDIR'"),
        // )
        .join(filename)
        .canonicalize()
        .expect("Error canonicalizing Path")
        .to_str()
        .expect("Error converting Path to String")
        .to_owned()
}

// #[test]
// fn test_read_bad_args() {
//     let path = "invalid_non_existant.exr";

//     assert_eq!(
//         start_read(&path).err().expect("Expected error"),
//         Error::FileAccess
//     );
// }

// #[test]
// fn test_read_bad_files() {
//     let path = get_test_file_path("invalid.exr");

//     assert!(start_read(&path).is_err());
// }

// #[test]
// fn test_open_scans() {
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
// fn test_open_tiles() {
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
// fn test_open_multi_part() {}

// #[test]
// fn test_open_deep() {}

// #[test]
// fn test_read_scans() {}

#[test]
fn test_read_tiles() -> Result<(), Error> {
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

    {
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
    }

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

    Ok(())
}

// #[test]
// fn test_read_multi_part() {}

// #[test]
// fn test_read_deep() {}
