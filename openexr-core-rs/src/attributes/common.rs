use openexr_core_sys as sys;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Compression {
    None,
    Rle,
    Zips,
    Zip,
    Piz,
    Pxr24,
    B44,
    B44a,
    Dwaa,
    Dwab,
}

impl From<Compression> for sys::exr_compression_t {
    fn from(c: Compression) -> sys::exr_compression_t {
        match c {
            Compression::None => sys::exr_compression_t::EXR_COMPRESSION_NONE,
            Compression::Rle => sys::exr_compression_t::EXR_COMPRESSION_RLE,
            Compression::Zips => sys::exr_compression_t::EXR_COMPRESSION_ZIPS,
            Compression::Zip => sys::exr_compression_t::EXR_COMPRESSION_ZIP,
            Compression::Piz => sys::exr_compression_t::EXR_COMPRESSION_PIZ,
            Compression::Pxr24 => sys::exr_compression_t::EXR_COMPRESSION_PXR24,
            Compression::B44 => sys::exr_compression_t::EXR_COMPRESSION_B44,
            Compression::B44a => sys::exr_compression_t::EXR_COMPRESSION_B44A,
            Compression::Dwaa => sys::exr_compression_t::EXR_COMPRESSION_DWAA,
            Compression::Dwab => sys::exr_compression_t::EXR_COMPRESSION_DWAB,
        }
    }
}

impl From<sys::exr_compression_t> for Compression {
    fn from(c: sys::exr_compression_t) -> Compression {
        match c {
            sys::exr_compression_t::EXR_COMPRESSION_NONE => Compression::None,
            sys::exr_compression_t::EXR_COMPRESSION_RLE => Compression::Rle,
            sys::exr_compression_t::EXR_COMPRESSION_ZIPS => Compression::Zips,
            sys::exr_compression_t::EXR_COMPRESSION_ZIP => Compression::Zip,
            sys::exr_compression_t::EXR_COMPRESSION_PIZ => Compression::Piz,
            sys::exr_compression_t::EXR_COMPRESSION_PXR24 => Compression::Pxr24,
            sys::exr_compression_t::EXR_COMPRESSION_B44 => Compression::B44,
            sys::exr_compression_t::EXR_COMPRESSION_B44A => Compression::B44a,
            sys::exr_compression_t::EXR_COMPRESSION_DWAA => Compression::Dwaa,
            sys::exr_compression_t::EXR_COMPRESSION_DWAB => Compression::Dwab,
            _ => panic!("Unhandled compression variant"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Envmap {
    Latlong,
    Cube,
}

impl From<Envmap> for sys::exr_envmap_t {
    fn from(e: Envmap) -> Self {
        match e {
            Envmap::Latlong => sys::exr_envmap_t::EXR_ENVMAP_LATLONG,
            Envmap::Cube => sys::exr_envmap_t::EXR_ENVMAP_CUBE,
        }
    }
}

impl From<sys::exr_envmap_t> for Envmap {
    fn from(e: sys::exr_envmap_t) -> Self {
        match e {
            sys::exr_envmap_t::EXR_ENVMAP_LATLONG => Envmap::Latlong,
            sys::exr_envmap_t::EXR_ENVMAP_CUBE => Envmap::Cube,
            _ => panic!("Unhandled envmap variant"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineOrder {
    IncreasingY,
    DecreasingY,
    RandomY,
}

impl From<LineOrder> for sys::exr_lineorder_t {
    fn from(e: LineOrder) -> Self {
        match e {
            LineOrder::IncreasingY => sys::exr_lineorder_t::EXR_LINEORDER_INCREASING_Y,
            LineOrder::DecreasingY => sys::exr_lineorder_t::EXR_LINEORDER_DECREASING_Y,
            LineOrder::RandomY => sys::exr_lineorder_t::EXR_LINEORDER_RANDOM_Y,
        }
    }
}

impl From<sys::exr_lineorder_t> for LineOrder {
    fn from(e: sys::exr_lineorder_t) -> Self {
        match e {
            sys::exr_lineorder_t::EXR_LINEORDER_INCREASING_Y => LineOrder::IncreasingY,
            sys::exr_lineorder_t::EXR_LINEORDER_DECREASING_Y => LineOrder::DecreasingY,
            sys::exr_lineorder_t::EXR_LINEORDER_RANDOM_Y => LineOrder::RandomY,
            _ => panic!("Unhandled lineorder variant"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Storage {
    Scanline,
    Tiled,
    DeepScanline,
    DeepTiled,
}

impl From<Storage> for sys::exr_storage_t {
    fn from(s: Storage) -> sys::exr_storage_t {
        match s {
            Storage::Scanline => sys::exr_storage_t::EXR_STORAGE_SCANLINE,
            Storage::Tiled => sys::exr_storage_t::EXR_STORAGE_TILED,
            Storage::DeepScanline => sys::exr_storage_t::EXR_STORAGE_DEEP_SCANLINE,
            Storage::DeepTiled => sys::exr_storage_t::EXR_STORAGE_DEEP_TILED,
        }
    }
}

impl From<sys::exr_storage_t> for Storage {
    fn from(s: sys::exr_storage_t) -> Storage {
        match s {
            sys::exr_storage_t::EXR_STORAGE_SCANLINE => Storage::Scanline,
            sys::exr_storage_t::EXR_STORAGE_TILED => Storage::Tiled,
            sys::exr_storage_t::EXR_STORAGE_DEEP_SCANLINE => Storage::DeepScanline,
            sys::exr_storage_t::EXR_STORAGE_DEEP_TILED => Storage::DeepTiled,
            _ => {
                panic!("unhandled exr_storage_t value")
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LevelMode {
    OneLevel,
    MipmapLevels,
    RipmapLevels,
}

impl From<LevelMode> for sys::exr_tile_level_mode_t {
    fn from(s: LevelMode) -> sys::exr_tile_level_mode_t {
        match s {
            LevelMode::OneLevel => sys::exr_tile_level_mode_t::EXR_TILE_ONE_LEVEL,
            LevelMode::MipmapLevels => sys::exr_tile_level_mode_t::EXR_TILE_MIPMAP_LEVELS,
            LevelMode::RipmapLevels => sys::exr_tile_level_mode_t::EXR_TILE_RIPMAP_LEVELS,
        }
    }
}

impl From<sys::exr_tile_level_mode_t> for LevelMode {
    fn from(s: sys::exr_tile_level_mode_t) -> LevelMode {
        match s {
            sys::exr_tile_level_mode_t::EXR_TILE_ONE_LEVEL => LevelMode::OneLevel,
            sys::exr_tile_level_mode_t::EXR_TILE_MIPMAP_LEVELS => LevelMode::MipmapLevels,
            sys::exr_tile_level_mode_t::EXR_TILE_RIPMAP_LEVELS => LevelMode::RipmapLevels,
            _ => {
                panic!("unhandled exr_tile_level_mode_t value")
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileRoundMode {
    RoundDown,
    RoundUp,
}

impl From<TileRoundMode> for sys::exr_tile_round_mode_t {
    fn from(e: TileRoundMode) -> Self {
        match e {
            TileRoundMode::RoundDown => sys::exr_tile_round_mode_t::EXR_TILE_ROUND_DOWN,
            TileRoundMode::RoundUp => sys::exr_tile_round_mode_t::EXR_TILE_ROUND_UP,
        }
    }
}

impl From<sys::exr_tile_round_mode_t> for TileRoundMode {
    fn from(e: sys::exr_tile_round_mode_t) -> Self {
        match e {
            sys::exr_tile_round_mode_t::EXR_TILE_ROUND_DOWN => TileRoundMode::RoundDown,
            sys::exr_tile_round_mode_t::EXR_TILE_ROUND_UP => TileRoundMode::RoundUp,
            _ => {
                panic!("unhandled exr_tile_round_mode_t value")
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PixelType {
    Uint,
    Half,
    Float,
}

impl From<PixelType> for sys::exr_pixel_type_t {
    fn from(e: PixelType) -> Self {
        match e {
            PixelType::Uint => sys::exr_pixel_type_t::EXR_PIXEL_UINT,
            PixelType::Half => sys::exr_pixel_type_t::EXR_PIXEL_HALF,
            PixelType::Float => sys::exr_pixel_type_t::EXR_PIXEL_FLOAT,
        }
    }
}

impl From<sys::exr_pixel_type_t> for PixelType {
    fn from(e: sys::exr_pixel_type_t) -> Self {
        match e {
            sys::exr_pixel_type_t::EXR_PIXEL_UINT => PixelType::Uint,
            sys::exr_pixel_type_t::EXR_PIXEL_HALF => PixelType::Half,
            sys::exr_pixel_type_t::EXR_PIXEL_FLOAT => PixelType::Float,
            _ => panic!("Unhandled exr_pixel_type_t value"),
        }
    }
}
