use crate::openexr_ffi::exr_error_code_t;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Copy, Clone)]
pub enum ExrError {
    #[error("Unable to allocate memory")]
    OutOfMemory, // EXR_ERR_OUT_OF_MEMORY,
    #[error("Context argument to function is not valid")]
    MissingContextArgument, // EXR_ERR_MISSING_CONTEXT_ARG,
    #[error("Invalid argument to function")]
    InvalidArgument, // EXR_ERR_INVALID_ARGUMENT,
    #[error("Argument to function out of valid range")]
    ArgumentOutOfRange, // EXR_ERR_ARGUMENT_OUT_OF_RANGE,
    #[error("Unable to open file (path does not exist or permission denied)")]
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
    #[error(
        "Attempt to modify a value when in update mode with different size"
    )]
    ModifySizeChange, // EXR_ERR_MODIFY_SIZE_CHANGE,
    #[error("File in write mode, but header already written, can no longer edit attributes")]
    AlreadyWroteAttributes, // EXR_ERR_ALREADY_WROTE_ATTRS,
    #[error(
        "Use deep scanline chunk writer with the sample count table arguments"
    )]
    UseScanDeepWrite, // EXR_ERR_USE_SCAN_DEEP_WRITE,
    #[error(
        "Use deep tile chunk writer with the sample count table arguments"
    )]
    UseTileDeepWrite, // EXR_ERR_USE_TILE_DEEP_WRITE,
    #[error("Use non-deep scanline chunk writer with the sample count table arguments")]
    UseScanNonDeepWrite, // EXR_ERR_USE_SCAN_NONDEEP_WRITE,
    #[error(
        "Use non-deep tile chunk writer with the sample count table arguments"
    )]
    UseTileNonDeepWrite, // EXR_ERR_USE_TILE_NONDEEP_WRITE,
    #[error(
        "Unexpected or corrupt values in data block leader vs computed value"
    )]
    BadChunkLeader, // EXR_ERR_BAD_CHUNK_LEADER,
    #[error("Corrupt data block data, unable to decode")]
    CorruptChunk, // EXR_ERR_CORRUPT_CHUNK,
    #[error(
        "Chunk offsets table not completely finished writing (incomplete file)"
    )]
    IncompleteChunkTable, // EXR_ERR_INCOMPLETE_CHUNK_TABLE,
    #[error("Previous part not yet finished writing")]
    IncorrectPart, // EXR_ERR_INCORRECT_PART,
    #[error("Invalid data block to write at this point")]
    IncorrectChunk, // EXR_ERR_INCORRECT_CHUNK,
    #[error("Invalid sample data table value")]
    InvalidSampleData, // EXR_ERR_INVALID_SAMPLE_DATA,
    #[error("Feature not yet implemented")]
    FeatureNotImplemented, // EXR_ERR_FEATURE_NOT_IMPLEMENTED,
    #[error("Unknown error code")]
    Unknown,
}

impl From<exr_error_code_t> for ExrError {
    fn from(error_code: exr_error_code_t) -> Self {
        match error_code {
            exr_error_code_t::EXR_ERR_SUCCESS => panic!("Unexpected error code 'EXR_ERR_SUCCESS' when converting 'exr_error_code_t' to 'ExrError'"),
            exr_error_code_t::EXR_ERR_OUT_OF_MEMORY => ExrError::OutOfMemory,
            exr_error_code_t::EXR_ERR_MISSING_CONTEXT_ARG => {
                ExrError::MissingContextArgument
            }
            exr_error_code_t::EXR_ERR_INVALID_ARGUMENT => {
                ExrError::InvalidArgument
            }
            exr_error_code_t::EXR_ERR_ARGUMENT_OUT_OF_RANGE => {
                ExrError::ArgumentOutOfRange
            }
            exr_error_code_t::EXR_ERR_FILE_BAD_HEADER => ExrError::FileBadHeader,
            exr_error_code_t::EXR_ERR_FILE_ACCESS => ExrError::FileAccess,
            exr_error_code_t::EXR_ERR_NOT_OPEN_READ => ExrError::NotOpenRead,
            exr_error_code_t::EXR_ERR_NOT_OPEN_WRITE => ExrError::NotOpenWrite,
            exr_error_code_t::EXR_ERR_HEADER_NOT_WRITTEN => {
                ExrError::HeaderNotWritten
            }
            exr_error_code_t::EXR_ERR_READ_IO => ExrError::ReadIo,
            exr_error_code_t::EXR_ERR_WRITE_IO => ExrError::WriteIo,
            exr_error_code_t::EXR_ERR_NAME_TOO_LONG => ExrError::NameTooLong,
            exr_error_code_t::EXR_ERR_MISSING_REQ_ATTR => {
                ExrError::MissingReqAttr
            }
            exr_error_code_t::EXR_ERR_INVALID_ATTR => ExrError::InvalidAttr,
            exr_error_code_t::EXR_ERR_NO_ATTR_BY_NAME => {
                ExrError::NoAttributeByName
            }
            exr_error_code_t::EXR_ERR_ATTR_TYPE_MISMATCH => {
                ExrError::AttributeTypeMismatch
            }
            exr_error_code_t::EXR_ERR_ATTR_SIZE_MISMATCH => {
                ExrError::AttributeSizeMismatch
            }
            exr_error_code_t::EXR_ERR_SCAN_TILE_MIXEDAPI => {
                ExrError::ScanTileMixedApi
            }
            exr_error_code_t::EXR_ERR_TILE_SCAN_MIXEDAPI => {
                ExrError::TileScanMixedApi
            }
            exr_error_code_t::EXR_ERR_MODIFY_SIZE_CHANGE => {
                ExrError::ModifySizeChange
            }
            exr_error_code_t::EXR_ERR_ALREADY_WROTE_ATTRS => {
                ExrError::AlreadyWroteAttributes
            }
            exr_error_code_t::EXR_ERR_USE_SCAN_DEEP_WRITE => {
                ExrError::UseScanDeepWrite
            }
            exr_error_code_t::EXR_ERR_USE_TILE_DEEP_WRITE => {
                ExrError::UseTileDeepWrite
            }
            exr_error_code_t::EXR_ERR_USE_SCAN_NONDEEP_WRITE => {
                ExrError::UseScanNonDeepWrite
            }
            exr_error_code_t::EXR_ERR_USE_TILE_NONDEEP_WRITE => {
                ExrError::UseTileNonDeepWrite
            }
            exr_error_code_t::EXR_ERR_BAD_CHUNK_LEADER => {
                ExrError::BadChunkLeader
            }
            exr_error_code_t::EXR_ERR_CORRUPT_CHUNK => {
                ExrError::CorruptChunk
            }
            exr_error_code_t::EXR_ERR_INCOMPLETE_CHUNK_TABLE => {
                ExrError::IncompleteChunkTable
            }
            exr_error_code_t::EXR_ERR_INCORRECT_PART => {
                ExrError::IncorrectPart
            }
            exr_error_code_t::EXR_ERR_INCORRECT_CHUNK => {
                ExrError::IncorrectChunk
            }
            exr_error_code_t::EXR_ERR_INVALID_SAMPLE_DATA => {
                ExrError::InvalidSampleData
            }
            exr_error_code_t::EXR_ERR_FEATURE_NOT_IMPLEMENTED => {
                ExrError::FeatureNotImplemented
            }
            exr_error_code_t::EXR_ERR_UNKNOWN => ExrError::Unknown,
            // _ => ExrError::Unknown,
        }
    }
}

impl From<ExrError> for exr_error_code_t {
    fn from(error: ExrError) -> Self {
        match error {
            ExrError::OutOfMemory => exr_error_code_t::EXR_ERR_OUT_OF_MEMORY,
            ExrError::MissingContextArgument => {
                exr_error_code_t::EXR_ERR_MISSING_CONTEXT_ARG
            }
            ExrError::InvalidArgument => {
                exr_error_code_t::EXR_ERR_INVALID_ARGUMENT
            }
            ExrError::ArgumentOutOfRange => {
                exr_error_code_t::EXR_ERR_ARGUMENT_OUT_OF_RANGE
            }
            ExrError::FileAccess => exr_error_code_t::EXR_ERR_FILE_ACCESS,
            ExrError::FileBadHeader => {
                exr_error_code_t::EXR_ERR_FILE_BAD_HEADER
            }
            ExrError::NotOpenRead => exr_error_code_t::EXR_ERR_NOT_OPEN_READ,
            ExrError::NotOpenWrite => exr_error_code_t::EXR_ERR_NOT_OPEN_WRITE,
            ExrError::HeaderNotWritten => {
                exr_error_code_t::EXR_ERR_HEADER_NOT_WRITTEN
            }
            ExrError::ReadIo => exr_error_code_t::EXR_ERR_READ_IO,
            ExrError::WriteIo => exr_error_code_t::EXR_ERR_WRITE_IO,
            ExrError::NameTooLong => exr_error_code_t::EXR_ERR_NAME_TOO_LONG,
            ExrError::MissingReqAttr => {
                exr_error_code_t::EXR_ERR_MISSING_REQ_ATTR
            }
            ExrError::InvalidAttr => exr_error_code_t::EXR_ERR_INVALID_ATTR,
            ExrError::NoAttributeByName => {
                exr_error_code_t::EXR_ERR_NO_ATTR_BY_NAME
            }
            ExrError::AttributeTypeMismatch => {
                exr_error_code_t::EXR_ERR_ATTR_TYPE_MISMATCH
            }
            ExrError::AttributeSizeMismatch => {
                exr_error_code_t::EXR_ERR_ATTR_SIZE_MISMATCH
            }
            ExrError::ScanTileMixedApi => {
                exr_error_code_t::EXR_ERR_SCAN_TILE_MIXEDAPI
            }
            ExrError::TileScanMixedApi => {
                exr_error_code_t::EXR_ERR_TILE_SCAN_MIXEDAPI
            }
            ExrError::ModifySizeChange => {
                exr_error_code_t::EXR_ERR_MODIFY_SIZE_CHANGE
            }
            ExrError::AlreadyWroteAttributes => {
                exr_error_code_t::EXR_ERR_ALREADY_WROTE_ATTRS
            }
            ExrError::UseScanDeepWrite => {
                exr_error_code_t::EXR_ERR_USE_SCAN_DEEP_WRITE
            }
            ExrError::UseTileDeepWrite => {
                exr_error_code_t::EXR_ERR_USE_TILE_DEEP_WRITE
            }
            ExrError::UseScanNonDeepWrite => {
                exr_error_code_t::EXR_ERR_USE_SCAN_NONDEEP_WRITE
            }
            ExrError::UseTileNonDeepWrite => {
                exr_error_code_t::EXR_ERR_USE_TILE_NONDEEP_WRITE
            }
            ExrError::BadChunkLeader => {
                exr_error_code_t::EXR_ERR_BAD_CHUNK_LEADER
            }
            ExrError::CorruptChunk => exr_error_code_t::EXR_ERR_CORRUPT_CHUNK,
            ExrError::IncompleteChunkTable => {
                exr_error_code_t::EXR_ERR_INCOMPLETE_CHUNK_TABLE
            }
            ExrError::IncorrectPart => exr_error_code_t::EXR_ERR_INCORRECT_PART,
            ExrError::IncorrectChunk => {
                exr_error_code_t::EXR_ERR_INCORRECT_CHUNK
            }
            ExrError::InvalidSampleData => {
                exr_error_code_t::EXR_ERR_INVALID_SAMPLE_DATA
            }
            ExrError::FeatureNotImplemented => {
                exr_error_code_t::EXR_ERR_FEATURE_NOT_IMPLEMENTED
            }

            ExrError::Unknown => exr_error_code_t::EXR_ERR_UNKNOWN,
        }
    }
}

impl exr_error_code_t {
    pub fn from_value(value: i32) -> exr_error_code_t {
        match value {
            0 => exr_error_code_t::EXR_ERR_SUCCESS,
            value
                if value > 0
                    && value < (exr_error_code_t::EXR_ERR_UNKNOWN as i32) =>
            unsafe { std::mem::transmute::<i32, exr_error_code_t>(value) },
            _ => exr_error_code_t::EXR_ERR_UNKNOWN,
        }
    }
}

#[cfg(test)]
mod test_errors {
    use super::*;

    #[test]
    fn test_error_conversion() {
        for value in 0..(exr_error_code_t::EXR_ERR_UNKNOWN as i32) {
            let error: exr_error_code_t =
                unsafe { std::mem::transmute::<i32, exr_error_code_t>(value) };

            let conversion = exr_error_code_t::from_value(value);

            assert_eq!(error, conversion);
        }
    }

    #[test]
    fn test_error_code_to_exr_error_coverage() {
        for value in 1..(exr_error_code_t::EXR_ERR_UNKNOWN as i32) {
            let error: exr_error_code_t =
                unsafe { std::mem::transmute::<i32, exr_error_code_t>(value) };

            let exr_error: ExrError = error.into();

            assert_ne!(exr_error, ExrError::Unknown, "Error code {:?} unexpectedly converted to ExrError::Unknown error.", error);
        }
    }
}
