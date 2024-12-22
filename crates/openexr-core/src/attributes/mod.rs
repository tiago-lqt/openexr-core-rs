mod channel;
mod common;
mod float_vector;
mod preview;
mod simple;
mod string;
mod string_vector;

pub mod values;

pub use common::*;
pub use float_vector::*;
pub use preview::*;
pub use simple::*;
pub use string::*;
pub use string_vector::*;

use openexr_core_sys as sys;
use std::ffi::CStr;

// pub use sys::exr_attr_chromaticities_t as AttrChromaticities;
// pub use sys::exr_attr_keycode_t as AttrKeycode;

// pub use sys::exr_attr_m33d_t as AttrM33d;
// pub use sys::exr_attr_m33f_t as AttrM33f;

// pub use sys::exr_attr_m44d_t as AttrM44d;
// pub use sys::exr_attr_m44f_t as AttrM44f;

// pub use sys::exr_attr_rational_t as AttrRational;
// pub use sys::exr_attr_timecode_t as AttrTimecode;

// pub use sys::exr_attr_v2d_t as AttrV2d;
// pub use sys::exr_attr_v2f_t as AttrV2f;
// pub use sys::exr_attr_v2i_t as AttrV2i;

// pub use sys::exr_attr_v3d_t as AttrV3d;
// pub use sys::exr_attr_v3f_t as AttrV3f;
// pub use sys::exr_attr_v3i_t as AttrV3i;

// pub use sys::exr_attr_box2f_t as AttrBox2f;
// pub use sys::exr_attr_box2i_t as AttrBox2i;

// pub use sys::exr_attr_tiledesc_t as AttrTiledesc;

#[repr(transparent)]
pub struct Attribute(pub(crate) sys::exr_attribute_t);

impl Attribute {
    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.name)
                .to_str()
                .expect("Could not convert name string")
        }
    }

    pub fn set_name(&mut self, name: &CStr) {
        self.0.name = name.as_ptr();
    }
}

pub struct AttributesList;

impl IntoIterator for AttributesList {
    type Item = Attribute;

    type IntoIter = AttributesIterator;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct AttributesIterator;

impl Iterator for AttributesIterator {
    type Item = Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
