use crate::ContextFlags;
use openexr_core_sys as sys;

pub struct Initializer {
    // TODO: Research support for user data
    // pub user_data: *const c_void,
    pub alloc_fn: sys::exr_memory_allocation_func_t,
    pub destroy_fn: sys::exr_destroy_stream_func_ptr_t,
    pub error_handler_fn: sys::exr_error_handler_cb_t,
    pub free_fn: sys::exr_memory_free_func_t,
    pub read_fn: sys::exr_read_func_ptr_t,
    pub size_fn: sys::exr_query_size_func_ptr_t,
    pub write_fn: sys::exr_write_func_ptr_t,
    pub max_image_width: u32,
    pub max_image_height: u32,
    pub max_tile_width: u32,
    pub max_tile_height: u32,
    pub zip_level: i32,
    pub dwa_quality: f32,
    pub flags: ContextFlags,
}

impl Initializer {
    pub fn default() -> Initializer {
        Initializer {
            error_handler_fn: None,
            alloc_fn: None,
            free_fn: None,
            read_fn: None,
            size_fn: None,
            write_fn: None,
            destroy_fn: None,
            max_image_width: 0,
            max_image_height: 0,
            max_tile_width: 0,
            max_tile_height: 0,
            zip_level: -2,
            dwa_quality: -1.0,
            flags: ContextFlags::NONE,
        }
    }

    pub fn with_error_handler(
        mut self,
        error_callback: sys::exr_error_handler_cb_t,
    ) -> Initializer {
        self.error_handler_fn = error_callback;
        self
    }

    pub fn with_write_function(mut self, write_function: sys::exr_write_func_ptr_t) -> Self {
        self.write_fn = write_function;
        self
    }

    pub fn with_alloc_function(
        mut self,
        alloc_function: sys::exr_memory_allocation_func_t,
    ) -> Self {
        self.alloc_fn = alloc_function;
        self
    }
}

impl Default for Initializer {
    fn default() -> Self {
        Self::default()
    }
}

impl From<Initializer> for sys::exr_context_initializer_t {
    fn from(init: Initializer) -> Self {
        sys::exr_context_initializer_t {
            size: std::mem::size_of::<sys::exr_context_initializer_t>(),
            error_handler_fn: init.error_handler_fn,
            alloc_fn: init.alloc_fn,
            free_fn: init.free_fn,
            // TODO: Add support for user data
            user_data: std::ptr::null_mut(),
            read_fn: init.read_fn,
            size_fn: init.size_fn,
            write_fn: init.write_fn,
            destroy_fn: init.destroy_fn,
            max_image_width: init.max_image_width as i32,
            max_image_height: init.max_image_height as i32,
            max_tile_width: init.max_tile_width as i32,
            max_tile_height: init.max_tile_height as i32,
            zip_level: init.zip_level,
            dwa_quality: init.dwa_quality,
            flags: 0,
        }
    }
}
