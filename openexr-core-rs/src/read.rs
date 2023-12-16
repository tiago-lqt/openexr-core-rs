use crate::attributes::{Attribute, AttributesList, Storage};
use crate::context::{Context, ContextPart};
use crate::{ExrError, ExrResult, Initializer, OkResult, Version};

use openexr_core_sys as sys;
pub use sys::Error;

use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};

/// Reader is a File opened for reading by start_read(). It is safe to call
/// any methods on Reader concurrently.
#[derive(Debug)]
pub struct Reader {
    context: sys::exr_context_t,
}

impl Reader {
    fn from_context(context: sys::exr_context_t) -> Reader {
        Reader { context }
    }

    pub fn inner(&self) -> sys::exr_context_t {
        self.context
    }

    pub fn read(file_name: impl AsRef<Path>) -> ExrResult<Reader> {
        Self::read_with_init(file_name, Initializer::default())

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

    pub fn read_with_init(file_name: impl AsRef<Path>, init: Initializer) -> ExrResult<Reader> {
        let mut context = std::ptr::null_mut();

        let path = file_name.as_ref().to_str().unwrap_or_default();

        // TODO: Figure out better way to handle this error
        let file_name = CString::new(path).map_err(|_| ExrError::FileAccess)?;

        let initalizer = init.into();

        unsafe { sys::exr_start_read(&mut context, file_name.as_ptr(), &initalizer) }.ok()?;

        let writer = Reader::from_context(context);

        Ok(writer)
    }
}

impl Context for Reader {
    type Part = PartReader;

    type PartIterator = PartListReader;

    fn file_name(&self) -> ExrResult<PathBuf> {
        let str = unsafe {
            let mut ptr = std::ptr::null();

            sys::exr_get_file_name(self.context, &mut ptr).ok()?;

            // TODO: Figure out better way to handle this error
            CStr::from_ptr(ptr)
                .to_str()
                .map_err(|_| ExrError::Unknown)?
        };

        Ok(PathBuf::from(str))
    }

    fn is_multipart(&self) -> ExrResult<bool> {
        // TODO: Check if the API exposes a better way to do this
        Ok(self.num_parts()? > 1)
    }

    fn num_parts(&self) -> ExrResult<usize> {
        let mut count = 0;
        unsafe {
            sys::exr_get_count(self.context, &mut count).ok()?;
        }
        Ok(count as usize)
    }

    fn parts(&self) -> PartListReader {
        todo!()
    }

    fn is_singlepart_tiled(&self) -> ExrResult<bool> {
        todo!()
    }

    fn has_longnames(&self) -> ExrResult<bool> {
        todo!()
    }

    fn has_nonimage_data(&self) -> ExrResult<bool> {
        todo!()
    }
}

pub struct PartReader;

impl ContextPart for PartReader {
    fn index(&self) -> ExrResult<usize> {
        todo!()
    }

    fn has_name(&self) -> ExrResult<bool> {
        todo!()
    }

    fn version(&self) -> ExrResult<u32> {
        todo!()
    }

    fn name(&self) -> ExrResult<Option<String>> {
        todo!()
    }

    fn has_attributes(&self) -> ExrResult<bool> {
        todo!()
    }

    fn attributes(&self) -> ExrResult<AttributesList> {
        todo!()
    }

    fn has_part_type(&self) -> ExrResult<bool> {
        todo!()
    }

    fn part_type(&self) -> ExrResult<Option<Attribute>> {
        todo!()
    }

    fn storage(&self) -> ExrResult<Storage> {
        todo!()
    }

    fn tile_levels(&self) -> ExrResult<usize> {
        todo!()
    }

    fn tile_sizes(&self, _level_x: u32, _level_y: u32) -> ExrResult<usize> {
        todo!()
    }

    fn chunk_count(&self, _level_x: u32, _level_y: u32) -> ExrResult<usize> {
        todo!()
    }
}

pub struct PartListReader;

impl IntoIterator for PartListReader {
    type Item = PartReader;

    type IntoIter = PartReaderIterator;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct PartReaderIterator;

impl Iterator for PartReaderIterator {
    type Item = PartReader;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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

#[derive(Default)]
pub struct ReadOptions {}

impl ReadOptions {}

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
