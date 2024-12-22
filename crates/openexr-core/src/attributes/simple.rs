use openexr_core_sys as sys;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CompressionType {
    None = 0,
    RLE = 1,
    ZIPS = 2,
    ZIP = 3,
    PIZ = 4,
    PXR24 = 5,
    B44 = 6,
    B44A = 7,
    DWAA = 8,
    DWAB = 9,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EnvmapType {
    Latlong = 0,
    Cube,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StorageType {
    Scanline = 0,
    Tiled,
    DeepScanline,
    DeepTiled,
}

impl From<StorageType> for sys::exr_storage_t {
    fn from(storage: StorageType) -> Self {
        match storage {
            StorageType::Scanline => sys::exr_storage_t::EXR_STORAGE_SCANLINE,
            StorageType::Tiled => sys::exr_storage_t::EXR_STORAGE_TILED,
            StorageType::DeepScanline => {
                sys::exr_storage_t::EXR_STORAGE_DEEP_SCANLINE
            }
            StorageType::DeepTiled => {
                sys::exr_storage_t::EXR_STORAGE_DEEP_TILED
            }
        }
    }
}
