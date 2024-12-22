use crate::ExrResult;
use openexr_core_sys as sys;

pub fn set_default_memory_routines(
    alloc_func: sys::exr_memory_allocation_func_t,
    free_func: sys::exr_memory_free_func_t,
) -> ExrResult<()> {
    unsafe {
        sys::exr_set_default_memory_routines(alloc_func, free_func);
    }

    Ok(())
}

pub fn reset_default_memory_routines() -> ExrResult<()> {
    unsafe {
        sys::exr_set_default_memory_routines(None, None);
    }

    Ok(())
}
