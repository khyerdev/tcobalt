use std::io::Read;

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
    pub c_video_quality: u16,
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
            c_video_quality: 1080,
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
                    self.method = Some(types::Method::Help);
                    match self.raw.get(2) {
                        Some(help) => match help.as_str() {
                            "get" | "g" => self.help_flag = Some(types::Help::Get),
                            "list" | "l" => self.help_flag = Some(types::Help::List),
                            "bulk" | "b" => self.help_flag = Some(types::Help::Bulk),
                            "help" | "h" => self.help_flag = Some(types::Help::Help),
                            "examples" | "e" => self.help_flag = Some(types::Help::Examples),
                            _ => self.help_flag = Some(types::Help::Help)
                        },
                        None => self.help_flag = Some(types::Help::Help),
                    }
                },
                "get" | "g" => {
                    self.method = Some(types::Method::Get);
                    let mut idx = 1;
                    let mut expected: Vec<ExpectedFlags> = Vec::new();
                    let mut stdin = false;
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
                                        self.c_url = Some(arg.clone());
                                        continue;
                                    }
                                    if self.c_url.is_some() && arg.contains("https://") {
                                        return Err(types::ParseError::throw_invalid("You cannot have 2 URLs in the same GET command"));
                                    }
                                    for c in arg.chars() {
                                        if !short {
                                            if c == '-' {
                                                short = true;
                                            } else if c == '+' {
                                                stdin = true;
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
                                        "144" | "480" | "720" | "1080" | "1440" | "2160" => self.c_video_quality = arg.parse().unwrap(),
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
                    if stdin {
                        let mut buf = String::new();
                        std::io::stdin().read_to_string(&mut buf).unwrap_or(0);
                        if self.c_url == None && buf.contains("https://") {
                            self.c_url = Some(buf.trim().to_string());
                        } else {
                            if self.c_url.is_some() {
                                return Err(types::ParseError::throw_invalid("You cannot have 2 URLs in the same GET command"));
                            }
                            return Err(types::ParseError::throw_invalid("URL from STDIN is invalid."))
                        }
                    }
                    if self.c_url == None {
                        return Err(types::ParseError::throw_incomplete("Missing URL from GET method"))
                    }
                    if expected.len() > 0 {
                        let mut missing = String::new();
                        for (i, v) in expected.iter().enumerate() {
                            if i > 0 {
                                missing.push_str(", ")
                            }
                            missing.push_str(&format!("{v:?}"));
                        }
                        return Err(types::ParseError::throw_incomplete(&format!("The following flags were specified but their values were not: {missing}")));
                    }
                },
                "bulk" | "b" => {
                    if let Some(action) = self.raw.get(2) {
                        self.method = Some(types::Method::Bulk);
                        match action.as_str() {
                            "get" | "g" => {
                                let mut url_list: Vec<String> = Vec::new();
                                let mut dummy_args = self.raw.clone();
                                let mut has_url = false;
                                (0..dummy_args.len()).rev().for_each(|i| {
                                    if dummy_args[i].contains("https://") {
                                        has_url = true;
                                        url_list.push(dummy_args[i].clone());
                                        dummy_args.remove(i);
                                    }
                                });
                                if !has_url {
                                    return Err(types::ParseError::throw_incomplete("Bulk get action is missing at least 1 URL"));
                                }
                                (0..=2).for_each(|_| {
                                    dummy_args.remove(0);
                                });
                                let get_flags = dummy_args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
                                let get_flags = Self::override_args(&[&["get", "https://"], get_flags.as_slice()].concat()).parse();
                                match get_flags {
                                    Ok(flags) => {
                                        let mut arg_array: Vec<Self> = Vec::new();
                                        for url in url_list {
                                            arg_array.push({
                                                let mut clone = flags.clone();
                                                clone.c_url = Some(url);
                                                clone
                                            });
                                        }
                                        self.c_bulk_array = Some(arg_array);
                                    },
                                    Err(e) => return Err(types::ParseError::throw_bulkerr(&format!("Invalid flags | {}", e.print()))),
                                }
                            },
                            "execute" | "exe" | "e" => {
                                if let Some(filename) = self.raw.get(3) {
                                    if let Ok(contents) = std::fs::read_to_string(filename) {
                                        let mut arg_array: Vec<Self> = Vec::new();
                                        for (i, line) in contents.lines().enumerate() {
                                            let mut args_raw = line.split(" ").collect::<Vec<&str>>();
                                            args_raw.insert(0, "get");
                                            match Self::override_args(args_raw.as_slice()).parse() {
                                                Ok(args) => arg_array.push(args),
                                                Err(e) => return Err(types::ParseError::throw_bulkerr(&format!("On line {} | {}", i+1, e.print())))
                                            }
                                        }
                                        self.c_bulk_array = Some(arg_array);
                                    } else {
                                        return Err(types::ParseError::throw_invalid("The file \"{filename}\" either doesnt exist, or doesn't have proper permissions"));
                                    }
                                } else {
                                    return Err(types::ParseError::throw_incomplete("Bulk execute action is missing the filename to execute commands from"));
                                }
                            },
                            _ => return Err(types::ParseError::throw_invalid(&format!("Invalid action: {action}")))
                        }
                    } else {
                        return Err(types::ParseError::throw_incomplete("Action is missing for bulk download"));
                    }
                },
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
        args.insert(0, "tcb".to_string());
        let mut template = Self::get();
        template.raw = args;
        template
    }
}

#[derive(Debug)]
enum ExpectedFlags {
    VideoCodec, VideoQuality, AudioFormat, Output,
}
