#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VideoCodec {
    H264, AV1, VP9
}
impl Default for VideoCodec {
    fn default() -> Self {
        Self::H264
    }
}
impl VideoCodec {
    pub fn print(&self) -> String {
        format!("{self:?}").to_lowercase()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AudioFormat {
    BEST, MP3, OGG, WAV, OPUS
}
impl Default for AudioFormat {
    fn default() -> Self {
        Self::MP3
    }
}
impl AudioFormat {
    pub fn print(&self) -> String {
        format!("{self:?}").to_lowercase()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FilenamePattern {
    Classic, Pretty, Basic, Nerdy
}
impl Default for FilenamePattern {
    fn default() -> Self {
        Self::Classic
    }
}
impl FilenamePattern {
    pub fn print(&self) -> String {
        format!("{self:?}").to_lowercase()
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DownloadMode {
    Auto, Audio, Mute
}
impl Default for DownloadMode {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Help {
    Get, List, Bulk, Help, Examples, Config, GenConfig
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Method {
    Get, List, Bulk, Help, Version, CobaltVersion(String), GenConfig
}

#[derive(Debug, PartialEq, Eq)]
enum ParseErrType {
    InvalidArg,
    Incomplete,
    BulkParseError
}
#[derive(Debug)]
pub struct ParseError {
    err_type: ParseErrType,
    message: String
}
impl ParseError {
    pub fn throw_incomplete(message: &str) -> Self {
        Self {
            err_type: ParseErrType::Incomplete,
            message: message.to_string()
        }
    }
    pub fn throw_invalid(message: &str) -> Self {
        Self {
            err_type: ParseErrType::InvalidArg,
            message: message.to_string()
        }
    }
    pub fn throw_bulkerr(message: &str) -> Self {
        Self {
            err_type: ParseErrType::BulkParseError,
            message: message.to_string()
        }
    }
    pub fn print(&self) -> String {
        format!("{:?}: {}", self.err_type , &self.message)
    }
}
