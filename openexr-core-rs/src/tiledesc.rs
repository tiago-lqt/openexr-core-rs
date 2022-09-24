use openexr_core_sys as sys;
pub use sys::Tiledesc;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileLevelMode {
    OneLevel,
    MipmapLevels,
    RipmapLevels,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileRoundMode {
    RoundDown,
    RoundUp,
}
