use openexr_core_sys as sys;
use std::convert::TryInto;
use std::ffi::CStr;
use std::ops::Deref;

use super::PixelType;

#[repr(transparent)]
pub struct Channel(sys::exr_attr_chlist_entry_t);

impl Channel {
    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.name.str_)
                .to_str()
                .expect("Failed to convert channel name")
        }
    }

    pub fn pixel_type(&self) -> PixelType {
        self.0.pixel_type.into()
    }

    pub fn p_linear(&self) -> bool {
        self.0.p_linear != 0
    }

    pub fn x_sampling(&self) -> i32 {
        self.0.x_sampling
    }

    pub fn y_sampling(&self) -> i32 {
        self.0.y_sampling
    }
}

#[repr(transparent)]
pub struct ChannelList(sys::exr_attr_chlist_t);

impl ChannelList {
    pub fn as_slice(&self) -> &[Channel] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.entries as *const Channel,
                self.0.num_channels.try_into().unwrap(),
            )
        }
    }
}

impl AsRef<[Channel]> for ChannelList {
    fn as_ref(&self) -> &[Channel] {
        self.as_slice()
    }
}

impl Deref for ChannelList {
    type Target = [Channel];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
