#![allow(non_camel_case_types)]

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
#[repr(C, packed)]
pub struct exr_attr_v2i_t {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C, packed)]
pub struct exr_attr_v2f_t {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C, packed)]
pub struct exr_attr_v2d_t {
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
#[repr(C, packed)]
pub struct exr_attr_v3i_t {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C, packed)]
pub struct exr_attr_v3f_t {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C, packed)]
pub struct exr_attr_v3d_t {
    x: f64,
    y: f64,
    z: f64,
}
