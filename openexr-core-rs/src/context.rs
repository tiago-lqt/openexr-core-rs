use bitflags::bitflags;
use openexr_core_sys as sys;

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
    pub struct ContextFlags: u32 {
        const NONE = 0;
        const STRICT_HEADER = sys::EXR_CONTEXT_FLAG_STRICT_HEADER;
        const SILENT_HEADER_PARSE = sys::EXR_CONTEXT_FLAG_SILENT_HEADER_PARSE;
        const DISABLE_CHUNK_RECONSTRUCTION = sys::EXR_CONTEXT_FLAG_DISABLE_CHUNK_RECONSTRUCTION;
    }
}

impl ContextFlags {
    pub fn default() -> ContextFlags {
        ContextFlags::NONE
    }

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
        ContextFlags::default()
    }
}
