use crate::{tcargs, args::Args, json};
use std::io::Write;

pub fn print_json_error(error: String, body: String) -> String {
    let mut text = String::new();
    text.push_str("Cobalt server returned improper JSON\n");
    text.push_str(&format!("JSON parse error: {error}\n"));
    if std::env::var("TCOBALT_DEBUG").is_ok_and(|v| v == 1.to_string()) == true {
        text.push_str(&format!("\n[DEBUG] Cobalt returned response:\n{body}\n\n"));
        text.push_str("[DEBUG] If this response isn't proper JSON, please contact wukko about this error.\n");
        text.push_str("[DEBUG] If this looks like proper json, contact khyernet/khyerdev about his json parser not functioning right.");
    } else {
        text.push_str("Contact wukko about this error. Run with TCOBALT_DEBUG=1 to see the incorrect response.")
    }
    text
}

pub fn get_url(args: &Args, status: &str, json: &std::collections::HashMap<String, json::JsonValue>) -> String {
    let media = if args.c_download_mode == tcargs::types::DownloadMode::Audio {
        "audio"
    } else {
        "video"
    };

    if status == "picker" {
        let urls = {
            let mut urls: Vec<String> = Vec::new();
            let picker_array = json.get("picker").unwrap().get_array().unwrap();
            for picker in picker_array.iter() {
                let picker = picker.get_object().unwrap();
                let cobalt_type = picker.get("type".into()).unwrap().get_str().unwrap();
                if cobalt_type == String::from("video") || cobalt_type == String::from("gif") {
                    urls.push(picker.get("url".into()).unwrap().get_str().unwrap());
                }
            }
            urls
        };

        let choice = if args.picker_choice == 0 {
            loop {
                let mut buf = String::new();
                print!("Choose which {media} to download [1-{}] >> ", urls.len());
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut buf).unwrap();
                if let Ok(int) = buf.trim().parse::<u8>() {
                    if int as usize <= urls.len() {
                        break int;
                    }
                }
                println!("Input must be an integer between 1 and {}", urls.len());
            }
        } else {
            args.picker_choice
        };

        urls.get((choice - 1) as usize).unwrap_or(&urls[0]).clone()
    } else {
        json.get("url").unwrap().get_str().unwrap()
    }
}

const POST_TEMPLATE: &str = "{
    \"url\": \"<url>\",
    \"youtubeVideoCodec\": \"<vcodec>\",
    \"videoQuality\": \"<vquality>\",
    \"audioFormat\": \"<aformat>\",
    \"audioBitrate\": \"<bitrate>\",
    \"filenameStyle\": \"<fname-style>\",
    \"downloadMode\": \"<download>\",
    \"tiktokFullAudio\": <tt-full-audio>,
    \"tiktokH265\": <tt-h265>,
    \"youtubeDubLang\": \"<dublang>\",
    \"disableMetadata\": <no-metadata>,
    \"twitterGif\": <twitter-gif>,
    \"alwaysProxy\": <proxy>
    }";
pub fn cobalt_args(args_in: &Args) -> String {
    POST_TEMPLATE.to_string()
        .replace("<url>", &args_in.c_url.clone().unwrap())
        .replace("<vcodec>", &args_in.c_video_codec.print())
        .replace("<vquality>", &args_in.c_video_quality.to_string())
        .replace("<aformat>", &args_in.c_audio_format.print())
        .replace("<fname-style>", &args_in.c_fname_style.print())
        .replace("<tt-full-audio>", &args_in.c_tt_full_audio.to_string())
        .replace("<tt-h265>", &args_in.c_tt_h265.to_string())
        .replace("<download>", &format!("{:?}", args_in.c_download_mode).to_lowercase())
        .replace("<dublang>", &args_in.accept_language)
        .replace("<no-metadata>", &args_in.c_disable_metadata.to_string())
        .replace("<twitter-gif>", &args_in.c_twitter_gif.to_string())
        .replace("<proxy>", &args_in.c_proxy.to_string())
        .replace("<bitrate>", &args_in.c_audio_bitrate.to_string())
}

#[macro_export]
macro_rules! attempt {
    ($try: expr, $error_msg_format: literal $(,$($extra:expr),*)?) => {{
        let result = $try;
        if result.is_err() {
            let e = result.unwrap_err().to_string();
            eprintln!($error_msg_format, e $(,$($extra)*)?);
            return false;
        }
        result.unwrap()
    }};
    ($try: expr, $error_string_generator: expr) => {{
        let result = $try;
        if result.is_err() {
            let e = result.unwrap_err().to_string();
            let diag = $error_string_generator;
            eprintln!("{}", diag.to_string().replace("{}", &e));
            return false;
        }
        result.unwrap()
    }};
}
