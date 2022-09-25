#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings;

pub use bindings::*;

use thiserror::Error;

// impl exr_error_code_t {
//     pub fn ok<T>(&self, value: T) -> Result<T, Error> {
//         match *self {
//             exr_error_code_t::EXR_ERR_SUCCESS => Ok(value),
//             code => Err(code.into()),
//         }
//     }
// }

// impl exr_result_t {
//     pub fn ok<T>(&self, value: T) -> Result<T, Error> {
//         match self.0 {
//             0 => Ok(value),
//             code => Err((exr_error_code_t(code)).into()),
//         }
//     }
// }

impl From<exr_error_code_t> for Error {
    fn from(error_code: exr_error_code_t) -> Self {
        match error_code {
            exr_error_code_t::EXR_ERR_OUT_OF_MEMORY => Error::OutOfMemory,
            exr_error_code_t::EXR_ERR_MISSING_CONTEXT_ARG => Error::MissingContextArgument,
            exr_error_code_t::EXR_ERR_INVALID_ARGUMENT => Error::InvalidArgument,
            exr_error_code_t::EXR_ERR_ARGUMENT_OUT_OF_RANGE => Error::ArgumentOutOfRange,
            exr_error_code_t::EXR_ERR_FILE_ACCESS => Error::FileAccess,
            exr_error_code_t::EXR_ERR_NOT_OPEN_READ => Error::NotOpenRead,
            exr_error_code_t::EXR_ERR_NOT_OPEN_WRITE => Error::NotOpenWrite,
            exr_error_code_t::EXR_ERR_HEADER_NOT_WRITTEN => Error::HeaderNotWritten,
            exr_error_code_t::EXR_ERR_READ_IO => Error::ReadIo,
            exr_error_code_t::EXR_ERR_WRITE_IO => Error::WriteIo,
            exr_error_code_t::EXR_ERR_NAME_TOO_LONG => Error::NameTooLong,
            exr_error_code_t::EXR_ERR_MISSING_REQ_ATTR => Error::MissingReqAttr,
            exr_error_code_t::EXR_ERR_INVALID_ATTR => Error::InvalidAttr,
            exr_error_code_t::EXR_ERR_NO_ATTR_BY_NAME => Error::NoAttributeByName,
            exr_error_code_t::EXR_ERR_ATTR_TYPE_MISMATCH => Error::AttributeTypeMismatch,
            exr_error_code_t::EXR_ERR_ATTR_SIZE_MISMATCH => Error::AttributeSizeMismatch,
            exr_error_code_t::EXR_ERR_SCAN_TILE_MIXEDAPI => Error::ScanTileMixedApi,
            exr_error_code_t::EXR_ERR_TILE_SCAN_MIXEDAPI => Error::TileScanMixedApi,
            exr_error_code_t::EXR_ERR_MODIFY_SIZE_CHANGE => Error::ModifySizeChange,
            exr_error_code_t::EXR_ERR_ALREADY_WROTE_ATTRS => Error::AlreadyWroteAttributes,
            exr_error_code_t::EXR_ERR_USE_SCAN_DEEP_WRITE => Error::UseScanDeepWrite,
            exr_error_code_t::EXR_ERR_USE_TILE_DEEP_WRITE => Error::UseTileDeepWrite,
            exr_error_code_t::EXR_ERR_USE_SCAN_NONDEEP_WRITE => Error::UseScanNonDeepWrite,
            exr_error_code_t::EXR_ERR_USE_TILE_NONDEEP_WRITE => Error::UseTileNonDeepWrite,
            exr_error_code_t::EXR_ERR_UNKNOWN => Error::Unknown,
            _ => Error::Unknown,
        }
    }
}

impl From<Error> for exr_error_code_t {
    fn from(error: Error) -> Self {
        match error {
            Error::OutOfMemory => exr_error_code_t::EXR_ERR_OUT_OF_MEMORY,
            Error::MissingContextArgument => exr_error_code_t::EXR_ERR_MISSING_CONTEXT_ARG,
            Error::InvalidArgument => exr_error_code_t::EXR_ERR_INVALID_ARGUMENT,
            Error::ArgumentOutOfRange => exr_error_code_t::EXR_ERR_ARGUMENT_OUT_OF_RANGE,
            Error::FileAccess => exr_error_code_t::EXR_ERR_FILE_ACCESS,
            Error::FileBadHeader => exr_error_code_t::EXR_ERR_FILE_BAD_HEADER,
            Error::NotOpenRead => exr_error_code_t::EXR_ERR_NOT_OPEN_READ,
            Error::NotOpenWrite => exr_error_code_t::EXR_ERR_NOT_OPEN_WRITE,
            Error::HeaderNotWritten => exr_error_code_t::EXR_ERR_HEADER_NOT_WRITTEN,
            Error::ReadIo => exr_error_code_t::EXR_ERR_READ_IO,
            Error::WriteIo => exr_error_code_t::EXR_ERR_WRITE_IO,
            Error::NameTooLong => exr_error_code_t::EXR_ERR_NAME_TOO_LONG,
            Error::MissingReqAttr => exr_error_code_t::EXR_ERR_MISSING_REQ_ATTR,
            Error::InvalidAttr => exr_error_code_t::EXR_ERR_INVALID_ATTR,
            Error::NoAttributeByName => exr_error_code_t::EXR_ERR_NO_ATTR_BY_NAME,
            Error::AttributeTypeMismatch => exr_error_code_t::EXR_ERR_ATTR_TYPE_MISMATCH,
            Error::AttributeSizeMismatch => exr_error_code_t::EXR_ERR_ATTR_SIZE_MISMATCH,
            Error::ScanTileMixedApi => exr_error_code_t::EXR_ERR_SCAN_TILE_MIXEDAPI,
            Error::TileScanMixedApi => exr_error_code_t::EXR_ERR_TILE_SCAN_MIXEDAPI,
            Error::ModifySizeChange => exr_error_code_t::EXR_ERR_MODIFY_SIZE_CHANGE,
            Error::AlreadyWroteAttributes => exr_error_code_t::EXR_ERR_ALREADY_WROTE_ATTRS,
            Error::UseScanDeepWrite => exr_error_code_t::EXR_ERR_USE_SCAN_DEEP_WRITE,
            Error::UseTileDeepWrite => exr_error_code_t::EXR_ERR_USE_TILE_DEEP_WRITE,
            Error::UseScanNonDeepWrite => exr_error_code_t::EXR_ERR_USE_SCAN_NONDEEP_WRITE,
            Error::UseTileNonDeepWrite => exr_error_code_t::EXR_ERR_USE_TILE_NONDEEP_WRITE,
            Error::Unknown => exr_error_code_t::EXR_ERR_UNKNOWN,
        }
    }
}

impl From<Error> for exr_result_t {
    fn from(error: Error) -> exr_result_t {
        let error_code: exr_error_code_t = error.into();
        exr_result_t(error_code.0)
    }
}

pub type SysResult<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
    #[error("Unable to allocate memory")]
    OutOfMemory, // EXR_ERR_OUT_OF_MEMORY,
    #[error("Context argument to function is not valid")]
    MissingContextArgument, // EXR_ERR_MISSING_CONTEXT_ARG,
    #[error("Invalid argument to function")]
    InvalidArgument, // EXR_ERR_INVALID_ARGUMENT,
    #[error("Argument to function out of valid range")]
    ArgumentOutOfRange, // EXR_ERR_ARGUMENT_OUT_OF_RANGE,
    #[error("Unable to open file (path does not exist or permission denied")]
    FileAccess, // EXR_ERR_FILE_ACCESS,
    #[error("File is not an OpenEXR file or has a bad header")]
    FileBadHeader, // EXR_ERR_FILE_BAD_HEADER,
    #[error("File not opened for read")]
    NotOpenRead, // EXR_ERR_NOT_OPEN_READ,
    #[error("File not opened for write")]
    NotOpenWrite, // EXR_ERR_NOT_OPEN_WRITE,
    #[error("File opened for write, but header not yet written")]
    HeaderNotWritten, // EXR_ERR_HEADER_NOT_WRITTEN,
    #[error("Error reading from stream")]
    ReadIo, // EXR_ERR_READ_IO,
    #[error("Error writing to stream")]
    WriteIo, // EXR_ERR_WRITE_IO,
    #[error("Text too long for file flags")]
    NameTooLong, // EXR_ERR_NAME_TOO_LONG,
    #[error("Missing required attribute in part header")]
    MissingReqAttr, // EXR_ERR_MISSING_REQ_ATTR,
    #[error("Invalid attribute in part header")]
    InvalidAttr, // EXR_ERR_INVALID_ATTR,
    #[error("No attribute by that name in part header")]
    NoAttributeByName, // EXR_ERR_NO_ATTR_BY_NAME,
    #[error("Attribute type mismatch")]
    AttributeTypeMismatch, // EXR_ERR_ATTR_TYPE_MISMATCH,
    #[error("Attribute type vs. size mismatch")]
    AttributeSizeMismatch, // EXR_ERR_ATTR_SIZE_MISMATCH,
    #[error("Attempt to use a scanline accessor function for a tiled image")]
    ScanTileMixedApi, // EXR_ERR_SCAN_TILE_MIXEDAPI,
    #[error("Attempt to use a tiled accessor function for a scanline image")]
    TileScanMixedApi, // EXR_ERR_TILE_SCAN_MIXEDAPI,
    #[error("Attempt to modify a value when in update mode with different size")]
    ModifySizeChange, // EXR_ERR_MODIFY_SIZE_CHANGE,
    #[error("File in write mode, but header already written, can no longer edit attributes")]
    AlreadyWroteAttributes, // EXR_ERR_ALREADY_WROTE_ATTRS,
    #[error("Use deep scanline chunk writer with the sample count table arguments")]
    UseScanDeepWrite, // EXR_ERR_USE_SCAN_DEEP_WRITE,
    #[error("Use deep tile chunk writer with the sample count table arguments")]
    UseTileDeepWrite, // EXR_ERR_USE_TILE_DEEP_WRITE,
    #[error("Use non-deep scanline chunk writer with the sample count table arguments")]
    UseScanNonDeepWrite, // EXR_ERR_USE_SCAN_NONDEEP_WRITE,
    #[error("Use non-deep tile chunk writer with the sample count table arguments")]
    UseTileNonDeepWrite, // EXR_ERR_USE_TILE_NONDEEP_WRITE,
    #[error("Unkown error code")]
    Unknown,
}

pub type Chromaticities = exr_attr_chromaticities_t;

pub type Keycode = exr_attr_keycode_t;

pub type Rational = exr_attr_rational_t;

pub type Timecode = exr_attr_timecode_t;

pub type Tiledesc = exr_attr_tiledesc_t;
