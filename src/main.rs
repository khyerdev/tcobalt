mod json;
mod args;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

use args as tcargs;
use args::Args;

const VERSION: &str = include_str!("version");

#[tokio::main]
async fn main() -> std::process::ExitCode {
    if std::env::args().len() == 1 {
        println!("tcobalt Command Line Utility; run `tcb help` for help");
        return std::process::ExitCode::SUCCESS;
    }

    let debug = std::env::var("TCOBALT_DEBUG").is_ok_and(|v| v == 1.to_string());
    if debug { eprintln!("[DEBUG] Parsing arguments ..") };
    let args = match Args::get().parse() {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Invalid command syntax");
            eprintln!("{}", err.print());
            return std::process::ExitCode::FAILURE;
        },
    };
    if let Some(help_flag) = args.help_flag {
        match help_flag {
            args::types::Help::Help => println!("{}", tcargs::strings::get_help()),
            args::types::Help::List => println!("{}", tcargs::strings::get_mod("list")),
            args::types::Help::Bulk => println!("{}", tcargs::strings::get_mod("bulk")),
            args::types::Help::Get => println!("{}", tcargs::strings::get_mod("get")),
            args::types::Help::Examples => println!("{}", tcargs::strings::get_mod("examples")),
        }
        return std::process::ExitCode::SUCCESS;
    }
    match args.method.clone().expect("Failed to catch invalid method early") {
        args::types::Method::Get => {
            if debug { eprintln!("[DEBUG] Executing GET method\n") };
            let success = execute_get_media(args, 0, debug).await;
            if debug { eprintln!("\n[DEBUG] GET method is complete") };
            if !success {
                return std::process::ExitCode::FAILURE;
            }
        },
        args::types::Method::Bulk => {
            let failed: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
            let mut futures_array: Vec<Pin<Box<dyn std::future::Future<Output = ()>>>> = Vec::new();
            let mut i = 0;
            if debug { eprintln!("[DEBUG] Collecting bulk tasks ...") };
            args.bulk_array.unwrap().iter().for_each(|a| {
                if args.same_filenames {
                    i += 1;
                }
                let args = a.clone();
                let switch = Arc::clone(&failed);
                let task = async move {
                    let success = execute_get_media(args, i, debug).await;
                    if !success {
                        let mut lock = switch.write().await;
                        *lock = true;
                    }
                };
                futures_array.push(Box::pin(task));
            });

            if debug { eprintln!("[DEBUG] Executing all tasks asynchronously ...\n") };
            futures::future::join_all(futures_array).await;
            if debug { eprintln!("\n[DEBUG] Execution has completed") };

            if failed.read().await.clone() == true {
                return std::process::ExitCode::FAILURE;
            }
        },
        args::types::Method::List => println!("{}", tcargs::strings::get_mod("supported")),
        args::types::Method::Help => unreachable!(),
        args::types::Method::Version => println!("{}", tcargs::strings::get_mod("version").replace("{}", VERSION.trim())),
        args::types::Method::CobaltVersion => {
            let request = reqwest::Client::new().get("https://co.wuk.sh/api/serverInfo")
                .header("User-Agent", &format!("tcobalt {}", VERSION.trim()));
            if debug { eprintln!("[DEBUG] Sending GET request to cobalt ...") };
            let ver = match request.send().await {
                Ok(res) => res.text().await.unwrap_or("{\"version\":\"unknown\",\"commit\":\"unknown\",\"branch\":\"unknown\"}".to_string()),
                Err(e) => {
                    eprintln!("Cobalt server did not respond: {}", e.to_string());
                    return std::process::ExitCode::FAILURE;
                }
            };
            if debug { eprintln!("[DEBUG] Response received, parsing json ...") };
            let stats = match json::parse(&ver) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("{}", print_cobalt_error(e, ver));
                    return std::process::ExitCode::FAILURE;
                }
            };
            let version = stats.get("version").unwrap().get_str().unwrap();
            let commit = stats.get("commit").unwrap().get_str().unwrap();
            let branch = stats.get("branch").unwrap().get_str().unwrap();
            println!("Cobalt (by wukko) version {version}");
            println!("Latest commit on branch \"{branch}\": {commit}");
        },
    }
    std::process::ExitCode::SUCCESS
}

async fn execute_get_media(args: Args, bulk: u16, debug: bool) -> bool {
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

    let json = cobalt_args(&args);
    let download_url: &str = args.c_url.as_ref().unwrap();

    let request = reqwest::Client::new().post("https://co.wuk.sh/api/json")
        .header("User-Agent", &format!("tcobalt {}", VERSION.trim()))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(json);

    if debug { eprintln!("[DEBUG {download_url}] Sending POST request to cobalt server ...") };
    let res = attempt!(request.send().await, "Cobalt server did not respond:\n\"{}\"\n(when downloading from {download_url})");

    let body = res.text().await.unwrap();
    if debug { eprintln!("[DEBUG {download_url}] Response received, parsing json ...") };
    let json = attempt!(json::parse(&body), print_cobalt_error("{}".into(), body));

    let status = json.get("status".into()).unwrap().get_str().unwrap();
    match status.as_str() {
        "error" => {
            let text = json.get("text").unwrap().get_str().unwrap();
            eprintln!("Cobalt returned error:\n\"{text}\"\n(when downloading from {download_url})");
            return false;
        },
        "stream" | "redirect" => {
            if debug && status == "redirect" { eprintln!("[DEBUG {download_url}] Cobalt returned a redirect url, sending GET request ...") };
            if debug && status == "stream" { eprintln!("[DEBUG {download_url}] Cobalt returned a stream, sending GET request ...") };

            let url = json.get("url").unwrap().get_str().unwrap();
            let stream_request = reqwest::Client::new().get(url)
                .header("User-Agent", &format!("tcobalt {}", VERSION.trim()));

            let res = attempt!(stream_request.send().await, "Live renderer did not respond:\n\"{}\"\n(when downloading from {download_url})");

            if debug { eprintln!("[DEBUG {download_url}] Response received from stream") };
            let filename = extract_filename(&args, res.headers(), bulk, debug);
            let media = if args.c_audio_only {
                "audio"
            } else {
                "video"
            };
            println!(
                "Downloading {} from {} ...", 
                media,
                download_url
            );
            let stream = attempt!(res.bytes().await, "Error decoding byte stream:\n\"{}\"\n(when downloading from {download_url})");

            println!("Data downloaded successfully! Writing {media} to {} ...", &filename);
            let path = std::env::current_dir().unwrap().join(&filename);
            attempt!(tokio::fs::write(path, stream).await, "Unable to write data to file:\n\"{}\"\n(when writing to {filename})");

            println!("Your {media} is ready! >> {filename}")
        },
        "rate-limit" => {
            eprintln!("You are being rate limited by cobalt! Please try again later.\n(when downloading from {download_url})");
            return false;
        }
        _ => unimplemented!()
    }
    true
}

fn print_cobalt_error(error: String, body: String) -> String {
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

fn extract_filename(args: &Args, headers: &reqwest::header::HeaderMap, bulk: u16, debug: bool) -> String {
    match &args.out_filename {
        Some(name) => {
            if bulk > 0 {
                format!("{bulk}-{name}")
            } else {
                name.to_string()
            }
        },
        None => {
            let download_url = args.c_url.clone().unwrap();
            if debug { eprintln!("[DEBUG {download_url}] Obtaining filename from headers") };
            match headers.get("Content-Disposition") {
                Some(disposition) => {
                    let disposition = disposition.to_str().unwrap();
                    let mut pass: u8 = 0;
                    let mut filename = String::new();
                    for c in disposition.chars() {
                        if c == ';' || c == '\"' {
                            pass += 1;
                            continue;
                        }
                        if pass == 2 {
                            filename.push(c);
                        }
                        if pass == 3 {
                            break;
                        }
                    }
                    filename
                },
                None => {
                    if debug { eprintln!("[DEBUG {download_url}] No filename specified, generating random filename ...") };
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    download_url.hash(&mut hasher);
                    let mut hash = format!("{:x}", hasher.finish());
                    if args.c_twitter_gif {
                        hash.push_str(".gif");
                    } else {
                        match args.c_video_codec {
                            tcargs::types::VideoCodec::AV1 | tcargs::types::VideoCodec::H264 => {
                                hash.push_str(".mp4");
                            }
                            tcargs::types::VideoCodec::VP9 => {
                                hash.push_str(".webm");
                            }
                        }
                    }
                    hash
                }
            }
        }
    }
}

const POST_TEMPLATE: &str = "{
    \"url\": \"<url>\",
    \"vCodec\": \"<vcodec>\",
    \"vQuality\": \"<vquality>\",
    \"aFormat\": \"<aformat>\",
    \"filenamePattern\": \"classic\",
    \"isAudioOnly\": <audio-only>,
    \"isTTFullAudio\": false,
    \"isAudioMuted\": <audio-muted>,
    \"dubLang\": false,
    \"disableMetadata\": false,
    \"twitterGif\": <twitter-gif>,
    \"vimeoDash\": false
}";
fn cobalt_args(args_in: &Args) -> String {
    POST_TEMPLATE.to_string()
        .replace("<url>", &args_in.c_url.clone().unwrap())
        .replace("<vcodec>", &args_in.c_video_codec.print())
        .replace("<vquality>", &args_in.c_video_quality.to_string())
        .replace("<aformat>", &args_in.c_audio_format.print())
        .replace("<audio-only>", &args_in.c_audio_only.to_string())
        .replace("<audio-muted>", &args_in.c_audio_muted.to_string())
        .replace("<twitter-gif>", &args_in.c_twitter_gif.to_string())
}

#[cfg(test)]
mod tests;
