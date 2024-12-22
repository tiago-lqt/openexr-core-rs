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
