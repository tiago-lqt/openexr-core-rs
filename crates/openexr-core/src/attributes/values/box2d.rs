use super::{V2f, V2i};

use openexr_core_sys as sys;

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Box2i {
    pub min: V2i,
    pub max: V2i,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Box2f {
    pub min: V2f,
    pub max: V2f,
}

impl Box2i {
    pub fn new(min: impl Into<V2i>, max: impl Into<V2i>) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
        }
    }
}

impl From<sys::exr_attr_box2i_t> for Box2i {
    fn from(value: sys::exr_attr_box2i_t) -> Self {
        Self {
            min: value.min.into(),
            max: value.max.into(),
        }
    }
}

impl From<sys::exr_attr_box2f_t> for Box2f {
    fn from(value: sys::exr_attr_box2f_t) -> Self {
        Self {
            min: value.min.into(),
            max: value.max.into(),
        }
    }
}
