pub mod types;
pub mod strings;

pub struct Args {
    pub raw: Vec<String>,
    pub parsed: bool,
    pub c_url: Option<String>,
    pub c_video_codec: Option<types::VideoCodec>,
    pub c_video_quality: Option<types::VideoQuality>,
    pub c_audio_format: Option<types::AudioFormat>,
    pub c_audio_only: Option<bool>,
    pub c_audio_muted: Option<bool>,
    pub c_twitter_gif: Option<bool>,
    pub out_filename: Option<String>,
    pub help_flag: Option<types::Help>
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
            out_filename: None,
            help_flag: None
        }
    }
    pub fn parse(mut self) -> Result<Self, types::ParseError> {
        self.parsed = true;
        self.raw = self.raw.iter().map(|str| str.to_lowercase()).collect::<Vec<String>>();
        match self.raw.get(1) {
            Some(method) => match method.as_str() {
                "help" | "-h" | "--help" | "h" => match self.raw.get(2) {
                    Some(help) => match help.as_str() {
                        "get" | "g" => self.help_flag = Some(types::Help::Get),
                        "list" | "l" => self.help_flag = Some(types::Help::List),
                        "bulk" | "b" => self.help_flag = Some(types::Help::Bulk),
                        "help" | "h" => self.help_flag = Some(types::Help::Help),
                        _ => self.help_flag = Some(types::Help::Help)
                    },
                    None => self.help_flag = Some(types::Help::Help),
                },
                "get" | "g" => todo!(),
                "list" | "l" => todo!(),
                "bulk" | "b" => todo!(),

                unknown => return Err(types::ParseError::throw_invalid(&format!("Unrecognized tcobalt method: {}", unknown)))
            },
            None => unreachable!() 
        }
        Ok(self)
    }
    pub fn _override_args(args: &[&str]) -> Self {
        let mut args = args.to_vec().iter().map(|str| str.to_string()).collect::<Vec<String>>();
        args.insert(0, "tc".to_string());
        Self {
            raw: args,
            parsed: false,
            c_url: None,
            c_video_codec: None,
            c_video_quality: None,
            c_audio_format: None,
            c_audio_only: None,
            c_audio_muted: None,
            c_twitter_gif: None,
            out_filename: None,
            help_flag: None
        }
    }
}

