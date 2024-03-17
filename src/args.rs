use crate::types;

pub struct Args {
    raw: Vec<String>,
    parsed: bool,
    c_url: Option<String>,
    c_video_codec: Option<types::VideoCodec>,
    c_video_quality: Option<types::VideoQuality>,
    c_audio_format: Option<types::AudioFormat>,
    c_audio_only: Option<bool>,
    c_audio_muted: Option<bool>,
    c_twitter_gif: Option<bool>,
    out_filename: Option<String>
}
impl Args {
    pub fn get() -> Self {
        Self {
            raw: std::env::args().collect(),
            parsed: false,
            c_url: None,
            c_video_codec: None,
            c_video_quality: None,
            c_audio_format: None,
            c_audio_only: None,
            c_audio_muted: None,
            c_twitter_gif: None,
            out_filename: None
        }
    }
    pub fn parse(&self) -> Option<Self> {
        let has_url = false;

    }
}

