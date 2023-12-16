use crate::ExrResult;
use openexr_core_sys as sys;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::ops::Deref;

pub trait AttributeRead: Sized {
    fn get<S: ContextState>(ctx: &Context<S>, part_index: usize, name: &str) -> ExrResult<Self>;
}

pub trait AttributeWrite: Sized {
    fn set(ctx: &WriteHeaderContext, part_index: usize, name: &str, value: &Self) -> ExrResult<()>;
}

impl AttributeRead for f32 {
    fn get<S: ContextState>(ctx: &Context<S>, part_index: usize, name: &str) -> ExrResult<Self> {
        let mut result = Default::default();
        unsafe {
            let c_name = CString::new(name).unwrap();
            sys::exr_attr_get_float(
                ctx.inner,
                part_index.try_into().unwrap(),
                c_name.as_ptr(),
                &mut result,
            )
            .ok(result)
        }
    }
}

impl AttributeRead for i32 {
    fn get<S: ContextState>(ctx: &Context<S>, part_index: usize, name: &str) -> Result<Self> {
        let mut result = Default::default();
        unsafe {
            let c_name = CString::new(name).unwrap();
            sys::exr_attr_get_int(
                ctx.inner,
                part_index.try_into().unwrap(),
                c_name.as_ptr(),
                &mut result,
            )
            .ok(result)
        }
    }
}

impl AttributeRead for &[f32] {
    fn get<S: ContextState>(ctx: &Context<S>, part_index: usize, name: &str) -> Result<Self> {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let mut sz = 0;
            let mut ptr = std::ptr::null();
            sys::exr_attr_get_float_vector(
                ctx.inner,
                part_index.try_into().unwrap(),
                c_name.as_ptr(),
                &mut sz,
                &mut ptr,
            )
            .ok(std::slice::from_raw_parts(ptr, sz as usize))
        }
    }
}

impl AttributeRead for Compression {
    fn get<S: ContextState>(ctx: &Context<S>, part_index: usize, name: &str) -> Result<Self> {
        let mut result = sys::exr_compression_t::EXR_COMPRESSION_LAST_TYPE;
        unsafe {
            let c_name = CString::new(name).unwrap();
            sys::exr_attr_get_compression(
                ctx.inner,
                part_index.try_into().unwrap(),
                c_name.as_ptr(),
                &mut result,
            )
            .ok(result.into())
        }
    }
}

impl AttributeRead for [i32; 4] {
    fn get<S: ContextState>(ctx: &Context<S>, part_index: usize, name: &str) -> Result<[i32; 4]> {
        let mut result = [0i32; 4];
        unsafe {
            let c_name = CString::new(name).unwrap();
            sys::exr_attr_get_box2i(
                ctx.inner,
                part_index.try_into().unwrap(),
                c_name.as_ptr(),
                result.as_mut_ptr() as *mut sys::exr_attr_box2i_t,
            )
            .ok(result.into())
        }
    }
}
