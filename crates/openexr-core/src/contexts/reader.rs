use crate::attributes::values::Box2i;
use crate::attributes::{Attribute, AttributesList, Storage};
use crate::{ExrError, ExrResult, OkResult, UserData};
use openexr_core_sys as sys;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::path::PathBuf;

use super::traits::{Context, ContextPart};

/// Reader is a File opened for reading by start_read(). It is safe to call
/// any methods on Reader concurrently.
#[derive(Debug)]
pub struct ReaderContext {
    context: sys::exr_const_context_t,
}

impl ReaderContext {
    pub fn from_context(context: sys::exr_const_context_t) -> ReaderContext {
        ReaderContext { context }
    }

    pub fn inner(&self) -> sys::exr_const_context_t {
        self.context
    }

    pub fn file_version_and_flags(&self) -> ExrResult<u32> {
        unsafe {
            let mut flags = 0;
            sys::exr_get_file_version_and_flags(self.context, &mut flags)
                .ok()?;
            Ok(flags)
        }
    }

    pub fn chunk_table_offset(&self, offset: i32) -> ExrResult<u64> {
        unsafe {
            let mut res = 0;
            sys::exr_get_chunk_table_offset(self.context, offset, &mut res)
                .ok()?;
            Ok(res)
        }
    }

    pub unsafe fn get_user_data(&self) -> ExrResult<Option<UserData>> {
        unsafe {
            let mut ptr = std::ptr::null_mut();

            sys::exr_get_user_data(self.context, &mut ptr).ok()?;

            if ptr.is_null() {
                Ok(None)
            } else {
                Ok(Some((ptr).into()))
            }
        }
    }

    pub fn get_part(&self, index: usize) -> ExrResult<PartReader> {
        let reader = PartReader {
            context: self.context,
            part_index: index,
        };

        Ok(reader)
    }
}

impl Context for ReaderContext {
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

pub struct PartReader {
    context: sys::exr_const_context_t,
    part_index: usize,
}

impl PartReader {
    pub fn chunk_count(&self) -> ExrResult<usize> {
        unsafe {
            let mut count = 0;
            sys::exr_get_chunk_count(
                self.context,
                self.part_index as i32,
                &mut count,
            )
            .ok()?;
            Ok(count as usize)
        }
    }

    pub fn scanlines_per_chunk(&self) -> ExrResult<usize> {
        unsafe {
            let mut count = 0;
            sys::exr_get_scanlines_per_chunk(
                self.context,
                self.part_index as i32,
                &mut count,
            )
            .ok()?;
            Ok(count as usize)
        }
    }

    pub fn scanline_chunk_info(&self, y: i32) -> ExrResult<()> {
        unsafe {
            let mut value = MaybeUninit::uninit();

            sys::exr_read_scanline_chunk_info(
                self.context,
                self.part_index as i32,
                y,
                value.as_mut_ptr(),
            )
            .ok()?;

            Ok(value.assume_init().into())
        }
    }

    pub fn zip_compression_level(&self) -> ExrResult<i32> {
        unsafe {
            let mut level = 0;
            sys::exr_get_zip_compression_level(
                self.context,
                self.part_index as i32,
                &mut level,
            )
            .ok()?;
            Ok(level)
        }
    }

    pub fn dwa_compression_level(&self) -> ExrResult<f32> {
        unsafe {
            let mut level = 0.;
            sys::exr_get_dwa_compression_level(
                self.context,
                self.part_index as i32,
                &mut level,
            )
            .ok()?;
            Ok(level)
        }
    }

    pub fn data_window(&self) -> ExrResult<Box2i> {
        unsafe {
            let mut value: MaybeUninit<sys::exr_attr_box2i_t> =
                MaybeUninit::uninit();

            sys::exr_get_data_window(
                self.context,
                self.part_index as i32,
                value.as_mut_ptr(),
            )
            .ok()?;

            Ok(value.assume_init().into())
        }
    }
}

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

    fn chunk_count(&self) -> ExrResult<usize> {
        Self::chunk_count(self)
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

unsafe impl Send for ReaderContext {}
unsafe impl Sync for ReaderContext {}

impl Drop for ReaderContext {
    fn drop(&mut self) {
        unsafe {
            // TODO: Drop might be unsound in case reader is Send.
            // TODO: Cast is turning a const pointer into a mutable one
            let mut inner = self.inner() as sys::exr_context_t;
            sys::exr_finish(&mut inner);
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
