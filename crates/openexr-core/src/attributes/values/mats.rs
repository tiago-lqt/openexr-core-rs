#[cfg(feature = "cgmath")]
mod cgmath {
    pub type M33f = cgmath::Matrix3<f32>;
    pub type M33d = cgmath::Matrix3<f64>;

    pub type M44f = cgmath::Matrix4<f32>;
    pub type M44d = cgmath::Matrix4<f64>;
}

#[cfg(feature = "nalgebra-glm")]
mod nalgebra_glm {
    pub type M33f = nalgebra_glm::Mat3x3;
    pub type M33d = nalgebra_glm::DMat3x3;
    pub type M44f = nalgebra_glm::Mat4x4;
    pub type M44d = nalgebra_glm::DMat4x4;
}

#[cfg(not(any(feature = "nalgebra-glm", feature = "nalgebra-glm")))]
mod internal_types {
    #[repr(C)]
    pub struct M33f {
        pub m: [f32; 9],
    }

    #[repr(C)]
    pub struct M33d {
        pub m: [f64; 9],
    }

    #[repr(C)]
    pub struct M44f {
        m: [f32; 16],
    }

    #[repr(C)]
    pub struct M44d {
        m: [f64; 16],
    }
}

#[cfg(feature = "cgmath")]
pub use cgmath::*;

#[cfg(feature = "nalgebra-glm")]
pub use nalgebra_glm::*;

#[cfg(not(any(feature = "nalgebra-glm", feature = "nalgebra-glm")))]
pub use internal_types::*;
