use self::types::VideoQuality;

pub mod types;
pub mod strings;

#[derive(Debug, Clone)]
pub struct Args {
    pub raw: Vec<String>,
    pub parsed: bool,
    pub method: Option<types::Method>,
    pub c_url: Option<String>,
    pub c_bulk_array: Option<Vec<Args>>,
    pub c_video_codec: types::VideoCodec,
    pub c_video_quality: types::VideoQuality,
    pub c_audio_format: types::AudioFormat,
    pub c_audio_only: bool,
    pub c_audio_muted: bool,
    pub c_twitter_gif: bool,
    pub out_filename: Option<String>,
    pub help_flag: Option<types::Help>
}
impl Args {
    pub fn get() -> Self {
        Self {
            raw: std::env::args().collect(),
            parsed: false,
            c_url: None,
            c_video_codec: types::VideoCodec::H264,
            c_video_quality: types::VideoQuality::default(),
            c_audio_format: types::AudioFormat::MP3,
            c_audio_only: false,
            c_audio_muted: false,
            c_twitter_gif: false,
            out_filename: None,
            help_flag: None,
            method: None,
            c_bulk_array: None,
        }
    }

    pub fn parse(mut self) -> Result<Self, types::ParseError> {
        self.parsed = true;
        self.raw = self.raw.iter().map(|str| str.to_lowercase()).collect();
        match self.raw.get(1) {
            Some(method) => match method.as_str() {
                "help" | "-h" | "--help" | "h" => {
                    self.method = Some(types::Method::Version);
                    match self.raw.get(2) {
                        Some(help) => match help.as_str() {
                            "get" | "g" => self.help_flag = Some(types::Help::Get),
                            "list" | "l" => self.help_flag = Some(types::Help::List),
                            "bulk" | "b" => self.help_flag = Some(types::Help::Bulk),
                            "help" | "h" => self.help_flag = Some(types::Help::Help),
                            _ => self.help_flag = Some(types::Help::Help)
                        },
                        None => self.help_flag = Some(types::Help::Help),
                    }
                },
                "get" | "g" => {
                    self.method = Some(types::Method::Get);
                    let mut idx = 1;
                    let mut expected: Vec<ExpectedFlags> = Vec::new();
                    while let Some(arg) = self.raw.get(idx+1) {
                        idx += 1;
                        if expected.len() == 0 {
                            let mut short = false;
                            match arg.as_str() {
                                "--vcodec" => expected.push(ExpectedFlags::VideoCodec),
                                "--vquality" => expected.push(ExpectedFlags::VideoQuality),
                                "--aformat" => expected.push(ExpectedFlags::AudioFormat),
                                "--audio-only" => self.c_audio_only = true,
                                "--mute-audio" => self.c_audio_muted = true,
                                "--twitter-gif" => self.c_twitter_gif = true,
                                "--output" => expected.push(ExpectedFlags::Output),
                                _ => {
                                    if self.c_url == None && arg.contains("https://") {
                                        self.c_url = Some(arg.clone()); // TODO: pipe url through stdin
                                        continue;
                                    } else if self.c_url.is_some() && arg.contains("https://") {
                                        return Err(types::ParseError::throw_invalid("You cannot have 2 URLs in the same GET command"));
                                    }
                                    for c in arg.chars() {
                                        if !short {
                                            if c == '-' {
                                                short = true;
                                            } else {
                                                return Err(types::ParseError::throw_invalid(&format!("Unrecognized argument: {arg}")));
                                            }
                                            continue;
                                        }
                                        match c {
                                            'c' => expected.push(ExpectedFlags::VideoCodec),
                                            'q' => expected.push(ExpectedFlags::VideoQuality),
                                            'f' => expected.push(ExpectedFlags::AudioFormat),
                                            'a' => self.c_audio_only = true,
                                            'm' => self.c_audio_muted = true,
                                            'g' => self.c_twitter_gif = true,
                                            'o' => expected.push(ExpectedFlags::Output),
                                            _ => return Err(types::ParseError::throw_invalid(&format!("Invalid character {c} in multi-flag argument: {arg}")))
                                        }
                                    }
                                }
                            }
                        } else {
                            match expected.remove(0) {
                                ExpectedFlags::VideoCodec => {
                                    match arg.as_str() {
                                        "h264" => self.c_video_codec = types::VideoCodec::H264,
                                        "av1" => self.c_video_codec = types::VideoCodec::AV1,
                                        "vp9" => self.c_video_codec = types::VideoCodec::VP9,
                                        _ => return Err(types::ParseError::throw_invalid(&format!("Invalid video codec: {arg}")))
                                    }
                                },
                                ExpectedFlags::VideoQuality => {
                                    match arg.as_str() {
                                        "144" | "480" | "720" | "1080" | "1440" | "2160" => self.c_video_quality = VideoQuality { quality: arg.parse().unwrap() },
                                        _ => return Err(types::ParseError::throw_invalid(&format!("Invalid video quality: {arg}")))
                                    }
                                },
                                ExpectedFlags::AudioFormat => {
                                    match arg.as_str() {
                                        "best" => self.c_audio_format = types::AudioFormat::BEST,
                                        "mp3" => self.c_audio_format = types::AudioFormat::MP3,
                                        "ogg" => self.c_audio_format = types::AudioFormat::OGG,
                                        "wav" => self.c_audio_format = types::AudioFormat::WAV,
                                        "opus" => self.c_audio_format = types::AudioFormat::OPUS,
                                        _ => return Err(types::ParseError::throw_invalid(&format!("Invalid audio format: {arg}")))
                                    }
                                },
                                ExpectedFlags::Output => {
                                    if arg.contains(".mp3") || arg.contains(".ogg") || arg.contains(".wav") || arg.contains(".opus") || arg.contains(".mp4") || arg.contains(".webm") {
                                        self.out_filename = Some(arg.clone())
                                    } else {
                                        return Err(types::ParseError::throw_invalid("Output filename must be a video file type (supported: mp4/webm), or an audio file type (supported: mp3/ogg/wav/opus)\nMake sure you choose the right file type for the chosen codec/format!"));
                                    }
                                }
                            }
                        }
                    }
                },
                "bulk" | "b" => todo!(),
                "list" | "l" => self.method = Some(types::Method::List),
                "version" | "v" | "-v" | "--version" => self.method = Some(types::Method::Version),
                "cobalt-version" | "cv" | "c" => self.method = Some(types::Method::CobaltVersion),

                unknown => return Err(types::ParseError::throw_invalid(&format!("Unrecognized tcobalt method: {}", unknown)))
            },
            None => unreachable!() 
        }
        Ok(self)
    }

    pub fn override_args(args: &[&str]) -> Self {
        let mut args = args.to_vec().iter().map(|str| str.to_string()).collect::<Vec<String>>();
        args.insert(0, "tc".to_string());
        let mut template = Self::get();
        template.raw = args;
        template
    }
}

enum ExpectedFlags {
    VideoCodec, VideoQuality, AudioFormat, Output,
}
