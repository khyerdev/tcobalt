use std::io::Read;

pub mod types;
mod config;

#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub raw: Vec<String>,
    pub parsed: bool,
    pub method: Option<types::Method>,
    pub c_url: Option<String>,
    pub bulk_array: Option<Vec<Args>>,
    pub c_video_codec: types::VideoCodec,
    pub c_video_quality: u16,
    pub c_audio_format: types::AudioFormat,
    pub c_audio_bitrate: u16,
    pub c_download_mode: types::DownloadMode,
    pub c_twitter_gif: bool,
    pub c_tt_full_audio: bool,
    pub c_tt_h265: bool,
    pub c_disable_metadata: bool,
    pub accept_language: String,
    pub out_filename: Option<String>,
    pub c_fname_style: types::FilenamePattern,
    pub same_filenames: bool,
    pub picker_choice: u8,
    pub cobalt_instance: String,
    pub help_flag: Option<types::Help>,
    pub c_proxy: bool
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
            c_download_mode: types::DownloadMode::Auto,
            c_twitter_gif: false,
            out_filename: None,
            same_filenames: false,
            c_audio_bitrate: 128,
            help_flag: None,
            method: None,
            bulk_array: None,
            picker_choice: 0,
            c_fname_style: types::FilenamePattern::Classic,
            c_tt_full_audio: false,
            c_tt_h265: false,
            c_disable_metadata: false,
            c_proxy: false,
            cobalt_instance: String::from("api.cobalt.tools"),
            accept_language: String::from("en")
        }
    }

    pub fn parse(mut self) -> Result<Self, types::ParseError> {
        self.parsed = true;
        self.raw = self.raw.iter().map(|str| {
            if !str.contains("https://") {
                str.to_lowercase()
            } else {
                str.into()
            }
        }).collect();

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
                            "gen-config" | "gc" => self.help_flag = Some(types::Help::GenConfig),
                            "config" | "c" => self.help_flag = Some(types::Help::Config),
                            _ => self.help_flag = Some(types::Help::Help)
                        },
                        None => self.help_flag = Some(types::Help::Help),
                    }
                },
                "get" | "g" => {
                    self.method = Some(types::Method::Get);

                    let mut instance_list: Vec<String> = Vec::new();
                    let mut default_args: Vec<String> = Vec::new();
                    config::load_config_into(&mut default_args, &mut instance_list);
                    let (pre_args, added_args) = self.raw.split_at(2);
                    self.raw = [pre_args, &default_args, added_args].concat().to_vec();

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
                                "--audio-only" => self.c_download_mode = types::DownloadMode::Audio,
                                "--mute-audio" => self.c_download_mode = types::DownloadMode::Mute,
                                "--auto" => self.c_download_mode = types::DownloadMode::Auto,
                                "--twitter-gif" => self.c_twitter_gif = !self.c_twitter_gif,
                                "--tt-full-audio" => self.c_tt_full_audio = !self.c_tt_full_audio,
                                "--tt-h265" => self.c_tt_h265 = !self.c_tt_h265,
                                "--dublang" => expected.push(ExpectedFlags::Language),
                                "--no-metadata" => self.c_disable_metadata = !self.c_disable_metadata,
                                "--output" => expected.push(ExpectedFlags::Output),
                                "--fname-style" => expected.push(ExpectedFlags::FilenamePattern),
                                "--pick" => expected.push(ExpectedFlags::Picker),
                                "--instance" => expected.push(ExpectedFlags::Instance),
                                "--bitrate" => expected.push(ExpectedFlags::Bitrate),
                                "--proxy" => self.c_proxy = !self.c_proxy,
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
                                            'a' => self.c_download_mode = types::DownloadMode::Audio,
                                            'm' => self.c_download_mode = types::DownloadMode::Mute,
                                            'g' => self.c_twitter_gif = !self.c_twitter_gif,
                                            'u' => self.c_tt_full_audio = !self.c_tt_full_audio,
                                            'h' => self.c_tt_h265 = !self.c_tt_h265,
                                            'l' => {
                                                expected.push(ExpectedFlags::Language);
                                            },
                                            'n' => self.c_disable_metadata = !self.c_disable_metadata,
                                            'o' => expected.push(ExpectedFlags::Output),
                                            's' => expected.push(ExpectedFlags::FilenamePattern),
                                            'p' => expected.push(ExpectedFlags::Picker),
                                            'i' => expected.push(ExpectedFlags::Instance),
                                            'x' => self.c_proxy = !self.c_proxy,
                                            '=' => self.c_download_mode = types::DownloadMode::Auto,
                                            'b' => expected.push(ExpectedFlags::Bitrate),
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
                                    if arg.contains(".mp3") || arg.contains(".ogg") || arg.contains(".wav") || arg.contains(".opus") || arg.contains(".mp4") || arg.contains(".webm") || arg.contains(".gif") {
                                        self.out_filename = Some(arg.clone())
                                    } else {
                                        return Err(types::ParseError::throw_invalid("Output filename must be a video file type (supported: mp4/webm/gif), or an audio file type (supported: mp3/ogg/wav/opus)\nMake sure you choose the right file type for the chosen codec/format!"));
                                    }
                                },
                                ExpectedFlags::FilenamePattern => {
                                    match arg.as_str() {
                                        "classic" | "c" => self.c_fname_style = types::FilenamePattern::Classic,
                                        "pretty" | "p" => self.c_fname_style = types::FilenamePattern::Pretty,
                                        "basic" | "b" => self.c_fname_style = types::FilenamePattern::Basic,
                                        "nerdy" | "n" => self.c_fname_style = types::FilenamePattern::Nerdy,
                                        _ => return Err(types::ParseError::throw_invalid(&format!("Invalid filename style: {arg}")))
                                    }
                                },
                                ExpectedFlags::Picker => {
                                    if let Ok(int) = arg.parse::<u8>() {
                                        self.picker_choice = int;
                                    } else {
                                        return Err(types::ParseError::throw_invalid("Picker choice must be an integer between 0 and 255"));
                                    }
                                },
                                ExpectedFlags::Language => {
                                    self.accept_language = arg.clone();
                                },
                                ExpectedFlags::Instance => {
                                    let mut url = if let Ok(choice) = arg.parse::<u8>() {
                                        if let Some(url) = instance_list.get((choice-1) as usize) {
                                            url.clone()
                                        } else {
                                            return Err(types::ParseError::throw_invalid("Invalid instance quick-choice"))
                                        }
                                    } else {
                                        arg.clone()
                                    };
                                    url = url.replace("https://", "");
                                    if let Some(idx) = url.find('/') {
                                        url.truncate(idx);
                                    }
                                    self.cobalt_instance = url;
                                },
                                ExpectedFlags::Bitrate => {
                                    if arg == "320" || arg == "256" || arg == "128" || arg == "96" || arg == "64" || arg == "8" {
                                        self.c_audio_bitrate = arg.parse::<u16>().unwrap();
                                    } else {
                                        return Err(types::ParseError::throw_invalid("Make sure you select a valid bitrate! (320/256/128/96/64/8)"));
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
                                        self.bulk_array = Some(arg_array);
                                        if flags.out_filename.is_some() {
                                            self.same_filenames = true;
                                        }
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
                                        self.bulk_array = Some(arg_array);
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
                "cobalt-version" | "cv" | "c" => {
                    if self.raw.get(2).is_some() {
                        self.method = Some(types::Method::CobaltVersion(self.raw[2].clone()))
                    } else {
                        self.method = Some(types::Method::CobaltVersion(String::from("api.cobalt.tools")))
                    }
                },
                "gen-config" | "gc" => self.method = Some(types::Method::GenConfig),

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
    VideoCodec, VideoQuality, AudioFormat, Output, FilenamePattern, Picker, Language, Instance, Bitrate
}
