use std::path::PathBuf;

use bitflags::bitflags;
use openexr_core_sys as sys;

use crate::{
    attributes::{Attribute, AttributesList, Storage},
    ExrResult, Version,
};

pub trait ContextOptions {
    fn with_strict_header(self) -> Self;
    fn with_silent_header_parse(self) -> Self;
    fn with_disable_chunk_reconstruction(self) -> Self;

    fn without_strict_header(self) -> Self;
    fn without_silent_header_parse(self) -> Self;
    fn without_disable_chunk_reconstruction(self) -> Self;

    fn has_strict_header(&self) -> bool;
    fn has_silent_header_parse(&self) -> bool;
    fn has_disable_chunk_reconstruction(&self) -> bool;
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ContextFlags: u32 {
        const NONE = 0;
        const STRICT_HEADER = sys::EXR_CONTEXT_FLAG_STRICT_HEADER;
        const SILENT_HEADER_PARSE = sys::EXR_CONTEXT_FLAG_SILENT_HEADER_PARSE;
        const DISABLE_CHUNK_RECONSTRUCTION = sys::EXR_CONTEXT_FLAG_DISABLE_CHUNK_RECONSTRUCTION;
    }
}

impl ContextFlags {
    pub fn with_strict_header(self) -> ContextFlags {
        self | ContextFlags::STRICT_HEADER
    }

    pub fn with_silent_header_parse(self) -> ContextFlags {
        self | ContextFlags::SILENT_HEADER_PARSE
    }

    pub fn with_disable_chunk_reconstruction(self) -> ContextFlags {
        self | ContextFlags::DISABLE_CHUNK_RECONSTRUCTION
    }

    pub fn without_strict_header(self) -> ContextFlags {
        self & !ContextFlags::STRICT_HEADER
    }

    pub fn without_silent_header_parse(self) -> ContextFlags {
        self & !ContextFlags::SILENT_HEADER_PARSE
    }

    pub fn without_disable_chunk_reconstruction(self) -> ContextFlags {
        self & !ContextFlags::DISABLE_CHUNK_RECONSTRUCTION
    }

    pub fn has_strict_header(&self) -> bool {
        (*self & ContextFlags::STRICT_HEADER) == ContextFlags::STRICT_HEADER
    }

    pub fn has_silent_header_parse(&self) -> bool {
        (*self & ContextFlags::SILENT_HEADER_PARSE) == ContextFlags::SILENT_HEADER_PARSE
    }

    pub fn has_disable_chunk_reconstruction(&self) -> bool {
        (*self & ContextFlags::DISABLE_CHUNK_RECONSTRUCTION)
            == ContextFlags::DISABLE_CHUNK_RECONSTRUCTION
    }
}

impl Default for ContextFlags {
    fn default() -> Self {
        ContextFlags::NONE
    }
}

pub trait Context {
    type Part: ContextPart;
    type PartIterator: IntoIterator<Item = Self::Part>;

    fn file_name(&self) -> ExrResult<PathBuf>;

    fn is_multipart(&self) -> ExrResult<bool>;

    fn is_singlepart_tiled(&self) -> ExrResult<bool>;

    fn has_longnames(&self) -> ExrResult<bool>;

    fn has_nonimage_data(&self) -> ExrResult<bool>;

    fn num_parts(&self) -> ExrResult<usize>;

    fn parts(&self) -> Self::PartIterator;
}

pub trait ContextPart {
    fn index(&self) -> ExrResult<usize>;

    fn has_name(&self) -> ExrResult<bool>;

    fn version(&self) -> ExrResult<u32>;

    fn name(&self) -> ExrResult<Option<String>>;

    fn has_attributes(&self) -> ExrResult<bool>;

    fn attributes(&self) -> ExrResult<AttributesList>;

    fn has_part_type(&self) -> ExrResult<bool>;

    fn part_type(&self) -> ExrResult<Option<Attribute>>;

    fn storage(&self) -> ExrResult<Storage>;

    fn tile_levels(&self) -> ExrResult<usize>;

    fn tile_sizes(&self, level_x: u32, level_y: u32) -> ExrResult<usize>;

    fn chunk_count(&self, level_x: u32, level_y: u32) -> ExrResult<usize>;
}
