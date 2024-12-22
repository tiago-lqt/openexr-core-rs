use crate::attributes::StorageType;
use crate::contexts::initializer::ContextInitializer;
use crate::contexts::traits::ContextFlags;
use crate::contexts::traits::ContextOptions;
use crate::ExrError;
use crate::ExrResult;
use crate::OkResult;
use openexr_core_sys as sys;
use std::ffi::CString;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum WriteMode {
    Directly,
    TempFile,
}

impl From<WriteMode> for sys::exr_default_write_mode_t {
    fn from(mode: WriteMode) -> Self {
        match mode {
            WriteMode::Directly => {
                sys::exr_default_write_mode::EXR_WRITE_FILE_DIRECTLY
            }
            WriteMode::TempFile => {
                sys::exr_default_write_mode::EXR_INTERMEDIATE_TEMP_FILE
            }
        }
    }
}

pub struct WriteOptions {
    pub mode: WriteMode,
    pub context_flags: ContextFlags,
}

impl WriteOptions {
    pub fn with_write_directly(mut self) -> WriteOptions {
        self.mode = WriteMode::Directly;
        self
    }

    pub fn with_write_temp_file(mut self) -> WriteOptions {
        self.mode = WriteMode::TempFile;
        self
    }
}

impl Default for WriteOptions {
    fn default() -> Self {
        WriteOptions {
            mode: WriteMode::Directly,
            context_flags: ContextFlags::default(),
        }
    }
}

pub struct Writer {
    context: sys::exr_context_t,
}

impl Writer {
    fn from_context(context: sys::exr_context_t) -> Writer {
        Writer { context }
    }

    pub fn inner(&self) -> sys::exr_context_t {
        self.context
    }

    pub fn inner_mut(&mut self) -> sys::exr_context_t {
        self.context
    }

    pub fn start(_filename: &str) -> ExrResult<Writer> {
        todo!()
        // let filename = CString::new(filename).unwrap();
        // let mut file = std::ptr::null_mut();
        // unsafe {
        // sys::exr_error_code_t(sys::exr_start_write(
        //     &mut file,
        //     filename.as_ptr(),
        //     use_temp_file,
        //     None,
        // ))
        // .ok(Writer { file })
        // }
    }

    pub fn start_with_init_and_options(
        file_name: impl AsRef<Path>,
        init: ContextInitializer,
        options: WriteOptions,
    ) -> ExrResult<Writer> {
        let mut context = std::ptr::null_mut();

        let path = file_name.as_ref().to_str().unwrap_or_default();

        // TODO: Figure out better way to handle this error
        let filename = CString::new(path).map_err(|_| ExrError::FileAccess)?;

        let default_mode = options.mode.into();

        let initalizer = init.into();

        unsafe {
            sys::exr_start_write(
                &mut context,
                filename.as_ptr(),
                default_mode,
                &initalizer,
            )
        }
        .ok()?;

        let writer = Writer::from_context(context);

        Ok(writer)
    }
}

impl Writer {
    pub fn add_part(
        &mut self,
        part_name: impl AsRef<str>,
        storage_type: StorageType,
    ) -> ExrResult<usize> {
        let mut new_index = 0;

        unsafe {
            let part_name = CString::new(part_name.as_ref())
                .map_err(|_| ExrError::FileAccess)?;

            sys::exr_add_part(
                self.context,
                part_name.as_ptr(),
                storage_type.into(),
                &mut new_index,
            )
        }
        .ok()?;

        Ok(new_index as usize)
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        unsafe {
            let mut inner = self.inner();

            sys::exr_finish((&mut inner) as *mut sys::exr_context_t);
        };
    }
}

impl ContextOptions for WriteOptions {
    fn with_strict_header(mut self) -> Self {
        self.context_flags = self.context_flags.with_strict_header();
        self
    }

    fn with_silent_header_parse(mut self) -> Self {
        self.context_flags = self.context_flags.with_silent_header_parse();
        self
    }

    fn with_disable_chunk_reconstruction(mut self) -> Self {
        self.context_flags =
            self.context_flags.with_disable_chunk_reconstruction();
        self
    }

    fn without_strict_header(mut self) -> Self {
        self.context_flags = self.context_flags.without_strict_header();
        self
    }

    fn without_silent_header_parse(mut self) -> Self {
        self.context_flags = self.context_flags.without_silent_header_parse();
        self
    }

    fn without_disable_chunk_reconstruction(mut self) -> Self {
        self.context_flags =
            self.context_flags.without_disable_chunk_reconstruction();
        self
    }

    fn has_strict_header(&self) -> bool {
        self.context_flags.has_strict_header()
    }

    fn has_silent_header_parse(&self) -> bool {
        self.context_flags.has_silent_header_parse()
    }

    fn has_disable_chunk_reconstruction(&self) -> bool {
        self.context_flags.has_disable_chunk_reconstruction()
    }
}
