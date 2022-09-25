use openexr_core::{ExrError, ExrResult, Initializer, StorageType, WriteOptions, Writer};
use openexr_core_sys as sys;

#[test]
pub fn attributes_strings() -> ExrResult<()> {
    let context = create_dummy_file_writer("<string>")?;

    string_helper(context);

    Ok(())
}

fn create_dummy_file_writer(_name: &str) -> ExrResult<Writer> {
    let init = Initializer::default()
        // we won't actually write to this and so don't need a proper
        // stream but need a writable context to test with.
        .with_write_function(Some(dummy_write))
        .with_alloc_function(Some(failable_malloc));

    let options = WriteOptions::default().with_write_directly();

    // exr_start_write(&f, test, EXR_WRITE_FILE_DIRECTLY, &cinit);
    let mut writer = openexr_core::Writer::start_with_init_and_options("<string>", init, options)?;

    // exr_add_part(f, "dummy", EXR_STORAGE_SCANLINE, NULL);
    let part_index = writer.add_part("dummy", StorageType::Scanline)?;

    assert_eq!(part_index, 0);

    Ok(writer)
}

unsafe extern "C" fn dummy_write(
    _ctxt: sys::exr_const_context_t,
    _userdata: *mut ::std::os::raw::c_void,
    _buffer: *const ::std::os::raw::c_void,
    _sz: u64,
    _offset: u64,
    _error_cb: sys::exr_stream_error_func_ptr_t,
) -> i64 {
    todo!()
}

unsafe extern "C" fn failable_malloc(bytes: usize) -> *mut std::os::raw::c_void {
    use std::alloc::{alloc, Layout};

    let layout = Layout::from_size_align_unchecked(bytes, 1);
    let ptr = alloc(layout);

    ptr as *mut std::os::raw::c_void
}

fn string_helper(context: Writer) {
    // exr_attr_string_t s, nil = { 0 };

    // EXRCORE_TEST_RVAL(exr_attr_string_init(f, &s, 4));
    // EXRCORE_TEST(s.str != NULL);
    // EXRCORE_TEST(s.length == 4);
    // EXRCORE_TEST(s.alloc_size == 5);
    // EXRCORE_TEST_RVAL(exr_attr_string_destroy(f, &s));
    // EXRCORE_TEST(s.str == NULL);
    // EXRCORE_TEST(s.length == 0);
    // EXRCORE_TEST(s.alloc_size == 0);

    // EXRCORE_TEST_RVAL_FAIL (        EXR_ERR_INVALID_ARGUMENT, exr_attr_string_create (f, NULL, "exr"));
    // EXRCORE_TEST_RVAL_FAIL (        EXR_ERR_MISSING_CONTEXT_ARG, exr_attr_string_create (NULL, &s, "exr"));

    // EXRCORE_TEST_RVAL (exr_attr_string_create (f, &s, NULL));
    // EXRCORE_TEST (s.str != NULL && s.str[0] == '\0');
    // EXRCORE_TEST (s.length == 0);
    // EXRCORE_TEST (s.alloc_size == 1);
    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));

    // EXRCORE_TEST_RVAL (exr_attr_string_create_with_length (f, &s, NULL, 10));
    // EXRCORE_TEST (s.str != NULL && s.str[0] == '\0');
    // EXRCORE_TEST (s.length == 10);
    // EXRCORE_TEST (s.alloc_size == 11);
    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));

    // EXRCORE_TEST_RVAL_FAIL_MALLOC (        EXR_ERR_OUT_OF_MEMORY, exr_attr_string_create (f, &s, "exr"));
    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));

    // EXRCORE_TEST_RVAL (exr_attr_string_create (f, &s, "exr"));
    // EXRCORE_TEST (s.str != NULL && !strcmp (s.str, "exr"));
    // EXRCORE_TEST (s.length == 3);
    // EXRCORE_TEST (s.alloc_size == 4);

    // EXRCORE_TEST_RVAL_FAIL (        EXR_ERR_INVALID_ARGUMENT, exr_attr_string_set (f, NULL, "exr"));
    // EXRCORE_TEST_RVAL_FAIL (        EXR_ERR_MISSING_CONTEXT_ARG, exr_attr_string_set (NULL, &s, "exr"));

    // EXRCORE_TEST_RVAL (exr_attr_string_set (f, &s, "openexr"));
    // EXRCORE_TEST (s.str != NULL && !strcmp (s.str, "openexr"));
    // EXRCORE_TEST (s.length == 7);
    // EXRCORE_TEST (s.alloc_size == 8);

    // EXRCORE_TEST_RVAL (exr_attr_string_set_with_length (f, &s, "exr", 3));
    // EXRCORE_TEST (s.str != NULL && !strcmp (s.str, "exr"));
    // EXRCORE_TEST (s.length == 3);
    // EXRCORE_TEST (s.alloc_size == 8);

    // EXRCORE_TEST_RVAL (        exr_attr_string_set_with_length (f, &s, "exropenexr", 3));
    // EXRCORE_TEST (s.str != NULL && !strcmp (s.str, "exr"));
    // EXRCORE_TEST (s.length == 3);
    // EXRCORE_TEST (s.alloc_size == 8);

    // EXRCORE_TEST_RVAL (exr_attr_string_set_with_length (f, &s, NULL, 3));
    // EXRCORE_TEST (        s.str != NULL && s.str[0] == '\0' && s.str[1] == '\0' &&        s.str[2] == '\0');
    // EXRCORE_TEST (s.length == 3);
    // EXRCORE_TEST (s.str[s.length] == '\0');
    // EXRCORE_TEST (s.alloc_size == 8);

    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));

    // EXRCORE_TEST_RVAL (exr_attr_string_create_with_length (f, &s, "exr", 6));
    // EXRCORE_TEST (s.str != NULL && !strcmp (s.str, "exr"));
    // EXRCORE_TEST (s.length == 6);
    // EXRCORE_TEST (s.alloc_size == 7);
    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));

    // EXRCORE_TEST_RVAL (        exr_attr_string_create_with_length (f, &s, "openexr", 3));
    // EXRCORE_TEST (s.str != NULL && !strcmp (s.str, "ope"));
    // EXRCORE_TEST (s.length == 3);
    // EXRCORE_TEST (s.alloc_size == 4);
    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));

    // EXRCORE_TEST_RVAL (exr_attr_string_init_static (f, &s, "exr"));
    // EXRCORE_TEST (s.str != NULL && !strcmp (s.str, "exr"));
    // EXRCORE_TEST (s.length == 3);
    // EXRCORE_TEST (s.alloc_size == 0);
    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));
    // EXRCORE_TEST (s.str == NULL);
    // EXRCORE_TEST (s.length == 0);
    // EXRCORE_TEST (s.alloc_size == 0);

    // EXRCORE_TEST_RVAL (        exr_attr_string_init_static_with_length (f, &s, "openexr", 7));
    // EXRCORE_TEST (s.str != NULL && !strcmp (s.str, "openexr"));
    // EXRCORE_TEST (s.length == 7);
    // EXRCORE_TEST (s.alloc_size == 0);
    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));
    // // make sure we can re-delete something?
    // EXRCORE_TEST_RVAL (exr_attr_string_destroy (f, &s));

    // size_t nbytes = (size_t) INT32_MAX + 1;
    // char*  tmp    = (char*) malloc (nbytes + 1);

    // // might be on a low memory machine, in which case just skip the test
    // if (tmp)
    // {
    //     memset (tmp, 'B', nbytes);
    //     tmp[nbytes] = '\0';

    //     EXRCORE_TEST_RVAL_FAIL (            EXR_ERR_MISSING_CONTEXT_ARG,            exr_attr_string_init_static (NULL, &s, tmp));
    //     EXRCORE_TEST_RVAL_FAIL (            EXR_ERR_INVALID_ARGUMENT, exr_attr_string_init_static (f, &s, tmp));

    //     EXRCORE_TEST_RVAL_FAIL (            EXR_ERR_MISSING_CONTEXT_ARG,            exr_attr_string_create (NULL, &s, tmp));
    //     EXRCORE_TEST_RVAL_FAIL (            EXR_ERR_INVALID_ARGUMENT, exr_attr_string_create (f, &s, tmp));

    //     EXRCORE_TEST_RVAL_FAIL (            EXR_ERR_MISSING_CONTEXT_ARG, exr_attr_string_set (NULL, &s, tmp));
    //     EXRCORE_TEST_RVAL_FAIL (            EXR_ERR_INVALID_ARGUMENT, exr_attr_string_set (f, &s, tmp));

    //     free (tmp);
    // }

    todo!()
}

#[test]
pub fn attributes_string_vectors() {}

#[test]
pub fn attributes_float_vectors() {}

#[test]
pub fn attributes_chlists() {}

#[test]
pub fn attributes_preview() {}

#[test]
pub fn attributes_opaque() {}

#[test]
pub fn attributes_handler() {}

#[test]
pub fn attributes_lists() {}
