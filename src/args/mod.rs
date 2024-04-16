pub mod types;
pub mod strings;

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
        self.raw = self.raw.iter().map(|str| str.to_lowercase()).collect::<Vec<String>>();
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
                    let mut idx = 2;
                    let mut expected: Vec<ExpectedFlags> = Vec::new();
                    while let Some(arg) = self.raw.get(idx) {
                        if expected.len() == 0 {
                            let mut short = false;
                            match arg.to_lowercase().as_str() {
                                "--vcodec" => expected.push(ExpectedFlags::VideoCodec),
                                "--vquality" => expected.push(ExpectedFlags::VideoQuality),
                                "--aformat" => expected.push(ExpectedFlags::AudioFormat),
                                "--audio-only" => self.c_audio_only = true,
                                "--mute-aduio" => self.c_audio_muted = true,
                                "--twitter-gif" => self.c_twitter_gif = true,
                                "--output" => expected.push(ExpectedFlags::Output),
                                _ => {
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
                                    match arg.to_lowercase().as_str() {
                                        "h264" => {},
                                        "av1" => {},
                                        "vp9" => {},
                                        _ => return Err(types::ParseError::throw_invalid(&format!("Invalid video codec: {arg}")))
                                    }
                                },
                                ExpectedFlags::VideoQuality => todo!(),
                                ExpectedFlags::AudioFormat => todo!(),
                                ExpectedFlags::Output => todo!(),
                            }
                        }
                        idx += 1;
                    }
                },
                "bulk" | "b" => todo!(),
                "list" | "l" => self.method = Some(types::Method::List),
                "version" | "v" | "-v" | "--version" => self.method = Some(types::Method::Version),
                "cobalt-version" | "cv" => self.method = Some(types::Method::CobaltVersion),

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
