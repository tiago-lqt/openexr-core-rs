use crate::ExrResult;
use crate::Initializer;
use openexr_core_sys as sys;
use std::path::Path;
pub use sys::Error;

/// Reader is a File opened for reading by start_read(). It is safe to call
/// any methods on Reader concurrently.
pub struct Reader {
    file: sys::exr_context_t,
}

impl Reader {
    pub fn inner(&self) -> sys::exr_context_t {
        self.file
    }
}

unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            let mut inner = self.inner();

            sys::exr_finish((&mut inner) as *mut sys::exr_context_t);
        };
    }
}

pub fn start_read(_filename: &str) -> ExrResult<Reader> {
    todo!();

    // let filename = CString::new(filename).unwrap();
    // let mut file = std::ptr::null_mut();
    // unsafe {
    //     sys::exr_error_code_t(sys::exr_start_read(
    //         &mut file,
    //         filename.as_ptr(),
    //         std::ptr::null(),
    //     ))
    //     .ok(Reader { file })
    // }
}

pub fn start_read_with_initializer(
    _file_name: impl AsRef<Path>,
    _init: Initializer,
) -> ExrResult<Reader> {
    todo!()
}

pub struct DecodeChunkInfo {
    // pub(crate) chunk_info: sys::ChannelDecodeInfo,
}

impl DecodeChunkInfo {
    pub fn unpacked_size(&self) -> u64 {
        todo!();
        // self.chunk_info.unpacked.size
    }

    pub fn channel_count(&self) -> i16 {
        todo!();
        // self.chunk_info.channel_count
    }

    // pub fn channels(&self) -> &[ChannelDecodeInfo] {
    // todo!();
    // unsafe {
    //     std::slice::from_raw_parts(
    //         self.chunk_info.channels,
    //         self.chunk_info.channel_count as usize,
    //     )
    // }
    // }
}

impl Drop for DecodeChunkInfo {
    fn drop(&mut self) {
        todo!();
        // let _ = unsafe {
        // sys::exr_destroy_decode_chunk_info(
        //     &mut self.chunk_info as *mut sys::exr_decode_chunk_info__t,
        // )
        // };
    }
}
