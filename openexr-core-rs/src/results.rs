use openexr_core_sys as sys;
use std::ffi::NulError;
use thiserror::Error;

pub type ExrResult<T> = Result<T, ExrError>;

#[derive(Error, Debug, PartialEq, Eq, Copy, Clone)]
pub enum ExrError {
    #[error("Unable to allocate memory")]
    OutOfMemory,
    #[error("Context argument to function is not valid")]
    MissingContextArgument,
    #[error("Invalid argument to function")]
    InvalidArgument,
    #[error("Argument to function out of valid range")]
    ArgumentOutOfRange,
    #[error("Unable to open file (path does not exist or permission denied")]
    FileAccess,
    #[error("File is not an OpenEXR file or has a bad header")]
    FileBadHeader,
    #[error("File not opened for read")]
    NotOpenRead,
    #[error("File not opened for write")]
    NotOpenWrite,
    #[error("File opened for write, but header not yet written")]
    HeaderNotWritten,
    #[error("Error reading from stream")]
    ReadIo,
    #[error("Error writing to stream")]
    WriteIo,
    #[error("Text too long for file flags")]
    NameTooLong,
    #[error("Missing required attribute in part header")]
    MissingReqAttr,
    #[error("Invalid attribute in part header")]
    InvalidAttr,
    #[error("No attribute by that name in part header")]
    NoAttributeByName,
    #[error("Attribute type mismatch")]
    AttributeTypeMismatch,
    #[error("Attribute type vs. size mismatch")]
    AttributeSizeMismatch,
    #[error("Attempt to use a scanline accessor function for a tiled image")]
    ScanTileMixedApi,
    #[error("Attempt to use a tiled accessor function for a scanline image")]
    TileScanMixedApi,
    #[error("Attempt to modify a value when in update mode with different size")]
    ModifySizeChange,
    #[error("File in write mode, but header already written, can no longer edit attributes")]
    AlreadyWroteAttributes,
    #[error("Use deep scanline chunk writer with the sample count table arguments")]
    UseScanDeepWrite,
    #[error("Use deep tile chunk writer with the sample count table arguments")]
    UseTileDeepWrite,
    #[error("Use non-deep scanline chunk writer with the sample count table arguments")]
    UseScanNonDeepWrite,
    #[error("Use non-deep tile chunk writer with the sample count table arguments")]
    UseTileNonDeepWrite,
    #[error("Unkown error code")]
    Unknown,
}

// impl From<sys::Error> for ExrError {
//     fn from(err: sys::Error) -> Self {
//         match err {
//             sys::Error::OutOfMemory => ExrError::OutOfMemory,
//             sys::Error::MissingContextArgument => ExrError::MissingContextArgument,
//             sys::Error::InvalidArgument => ExrError::InvalidArgument,
//             sys::Error::ArgumentOutOfRange => ExrError::ArgumentOutOfRange,
//             sys::Error::FileAccess => ExrError::FileAccess,
//             sys::Error::FileBadHeader => ExrError::FileBadHeader,
//             sys::Error::NotOpenRead => ExrError::NotOpenRead,
//             sys::Error::NotOpenWrite => ExrError::NotOpenWrite,
//             sys::Error::HeaderNotWritten => ExrError::HeaderNotWritten,
//             sys::Error::ReadIo => ExrError::ReadIo,
//             sys::Error::WriteIo => ExrError::WriteIo,
//             sys::Error::NameTooLong => ExrError::NameTooLong,
//             sys::Error::MissingReqAttr => ExrError::MissingReqAttr,
//             sys::Error::InvalidAttr => ExrError::InvalidAttr,
//             sys::Error::NoAttributeByName => ExrError::NoAttributeByName,
//             sys::Error::AttributeTypeMismatch => ExrError::AttributeTypeMismatch,
//             sys::Error::AttributeSizeMismatch => ExrError::AttributeSizeMismatch,
//             sys::Error::ScanTileMixedApi => ExrError::ScanTileMixedApi,
//             sys::Error::TileScanMixedApi => ExrError::TileScanMixedApi,
//             sys::Error::ModifySizeChange => ExrError::ModifySizeChange,
//             sys::Error::AlreadyWroteAttributes => ExrError::AlreadyWroteAttributes,
//             sys::Error::UseScanDeepWrite => ExrError::UseScanDeepWrite,
//             sys::Error::UseTileDeepWrite => ExrError::UseTileDeepWrite,
//             sys::Error::UseScanNonDeepWrite => ExrError::UseScanNonDeepWrite,
//             sys::Error::UseTileNonDeepWrite => ExrError::UseTileNonDeepWrite,
//             _ => ExrError::Unknown,
//         }
//     }
// }

impl From<sys::exr_error_code_t> for ExrError {
    fn from(error_code: sys::exr_error_code_t) -> Self {
        match error_code {
            sys::exr_error_code_t::EXR_ERR_OUT_OF_MEMORY => ExrError::OutOfMemory,
            sys::exr_error_code_t::EXR_ERR_MISSING_CONTEXT_ARG => ExrError::MissingContextArgument,
            sys::exr_error_code_t::EXR_ERR_INVALID_ARGUMENT => ExrError::InvalidArgument,
            sys::exr_error_code_t::EXR_ERR_ARGUMENT_OUT_OF_RANGE => ExrError::ArgumentOutOfRange,
            sys::exr_error_code_t::EXR_ERR_FILE_ACCESS => ExrError::FileAccess,
            sys::exr_error_code_t::EXR_ERR_NOT_OPEN_READ => ExrError::NotOpenRead,
            sys::exr_error_code_t::EXR_ERR_NOT_OPEN_WRITE => ExrError::NotOpenWrite,
            sys::exr_error_code_t::EXR_ERR_HEADER_NOT_WRITTEN => ExrError::HeaderNotWritten,
            sys::exr_error_code_t::EXR_ERR_READ_IO => ExrError::ReadIo,
            sys::exr_error_code_t::EXR_ERR_WRITE_IO => ExrError::WriteIo,
            sys::exr_error_code_t::EXR_ERR_NAME_TOO_LONG => ExrError::NameTooLong,
            sys::exr_error_code_t::EXR_ERR_MISSING_REQ_ATTR => ExrError::MissingReqAttr,
            sys::exr_error_code_t::EXR_ERR_INVALID_ATTR => ExrError::InvalidAttr,
            sys::exr_error_code_t::EXR_ERR_NO_ATTR_BY_NAME => ExrError::NoAttributeByName,
            sys::exr_error_code_t::EXR_ERR_ATTR_TYPE_MISMATCH => ExrError::AttributeTypeMismatch,
            sys::exr_error_code_t::EXR_ERR_ATTR_SIZE_MISMATCH => ExrError::AttributeSizeMismatch,
            sys::exr_error_code_t::EXR_ERR_SCAN_TILE_MIXEDAPI => ExrError::ScanTileMixedApi,
            sys::exr_error_code_t::EXR_ERR_TILE_SCAN_MIXEDAPI => ExrError::TileScanMixedApi,
            sys::exr_error_code_t::EXR_ERR_MODIFY_SIZE_CHANGE => ExrError::ModifySizeChange,
            sys::exr_error_code_t::EXR_ERR_ALREADY_WROTE_ATTRS => ExrError::AlreadyWroteAttributes,
            sys::exr_error_code_t::EXR_ERR_USE_SCAN_DEEP_WRITE => ExrError::UseScanDeepWrite,
            sys::exr_error_code_t::EXR_ERR_USE_TILE_DEEP_WRITE => ExrError::UseTileDeepWrite,
            sys::exr_error_code_t::EXR_ERR_USE_SCAN_NONDEEP_WRITE => ExrError::UseScanNonDeepWrite,
            sys::exr_error_code_t::EXR_ERR_USE_TILE_NONDEEP_WRITE => ExrError::UseTileNonDeepWrite,
            sys::exr_error_code_t::EXR_ERR_UNKNOWN => ExrError::Unknown,
            _ => ExrError::Unknown,
        }
    }
}

impl From<NulError> for ExrError {
    fn from(_err: NulError) -> Self {
        todo!()
    }
}

pub trait OkResult {
    type Result;

    fn ok(self) -> Self::Result;
}

impl OkResult for sys::exr_result_t {
    type Result = ExrResult<()>;

    fn ok(self) -> Self::Result {
        match self.0 {
            0 => Ok(()),
            code => Err(sys::exr_error_code_t(code).into()),
        }
    }
}
