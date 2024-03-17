pub enum VideoCodec {
    h264, av1, vp9
}
impl Default for VideoCodec {
    fn default() -> Self {
        Self::h264
    }
}

pub struct VideoQuality {
    quality: u16
}
impl Default for VideoQuality {
    fn default() -> Self {
        Self { quality: 1080 }
    }
}

pub enum AudioFormat {
    BEST, MP3, OGG, WAV, OPUS
}
impl Default for AudioFormat {
    fn default() -> Self {
        Self::MP3
    }
}
