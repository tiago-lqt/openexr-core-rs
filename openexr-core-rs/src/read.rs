pub struct DecodeChunkInfo {
    // pub(crate) chunk_info: sys::ChannelDecodeInfo,
}

impl DecodeChunkInfo {
    pub fn unpacked_size(&self) -> u64 {
        todo!();
        // self.chunk_info.unpacked.size
    }

    pub fn channel_count(&self) -> i16 {
        todo!();
        // self.chunk_info.channel_count
    }

    // pub fn channels(&self) -> &[ChannelDecodeInfo] {
    // todo!();
    // unsafe {
    //     std::slice::from_raw_parts(
    //         self.chunk_info.channels,
    //         self.chunk_info.channel_count as usize,
    //     )
    // }
    // }
}

impl Drop for DecodeChunkInfo {
    fn drop(&mut self) {
        todo!();
        // let _ = unsafe {
        // sys::exr_destroy_decode_chunk_info(
        //     &mut self.chunk_info as *mut sys::exr_decode_chunk_info__t,
        // )
        // };
    }
}
