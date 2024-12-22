use openexr_core_sys as sys;

#[derive(Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct V2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
#[repr(C, packed)]
pub struct V2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq)]
#[repr(C, packed)]
pub struct V2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct V3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, PartialEq)]
#[repr(C, packed)]
pub struct V3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq)]
#[repr(C, packed)]
pub struct V3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<sys::exr_attr_v2d_t> for V2d {
    fn from(value: sys::exr_attr_v2d_t) -> Self {
        unsafe { std::mem::transmute::<sys::exr_attr_v2d_t, V2d>(value) }
    }
}

impl From<(f64, f64)> for V2d {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<sys::exr_attr_v2f_t> for V2f {
    fn from(value: sys::exr_attr_v2f_t) -> Self {
        unsafe { std::mem::transmute::<sys::exr_attr_v2f_t, V2f>(value) }
    }
}

impl From<(f32, f32)> for V2f {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<sys::exr_attr_v2i_t> for V2i {
    fn from(value: sys::exr_attr_v2i_t) -> Self {
        unsafe { std::mem::transmute::<sys::exr_attr_v2i_t, V2i>(value) }
    }
}

impl From<(i32, i32)> for V2i {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<sys::exr_attr_v3d_t> for V3d {
    fn from(value: sys::exr_attr_v3d_t) -> Self {
        unsafe { std::mem::transmute::<sys::exr_attr_v3d_t, V3d>(value) }
    }
}

impl From<(f64, f64, f64)> for V3d {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.1,
        }
    }
}

impl From<sys::exr_attr_v3f_t> for V3f {
    fn from(value: sys::exr_attr_v3f_t) -> Self {
        unsafe { std::mem::transmute::<sys::exr_attr_v3f_t, V3f>(value) }
    }
}

impl From<(f32, f32, f32)> for V3f {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.1,
        }
    }
}

impl From<sys::exr_attr_v3i_t> for V3i {
    fn from(value: sys::exr_attr_v3i_t) -> Self {
        unsafe { std::mem::transmute::<sys::exr_attr_v3i_t, V3i>(value) }
    }
}

impl From<(i32, i32, i32)> for V3i {
    fn from(value: (i32, i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.1,
        }
    }
}
