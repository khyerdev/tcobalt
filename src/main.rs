mod json;
mod args;
mod strings;
mod process;
use process as proc;
use std::hash::{Hash, Hasher};
use std::io::Write;
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
            args::types::Help::Help => println!("{}", strings::get_help()),
            args::types::Help::List => println!("{}", strings::get_str("usage", "list")),
            args::types::Help::Bulk => println!("{}", strings::get_str("usage", "bulk")),
            args::types::Help::Get => println!("{}", strings::get_str("usage", "get")),
            args::types::Help::Examples => println!("{}", strings::get_str("usage", "examples")),
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
        args::types::Method::List => println!("{}", strings::get_str("info", "supported")),
        args::types::Method::Help => unreachable!(),
        args::types::Method::Version => println!("{}", strings::get_str("info", "version").replace("{}", VERSION.trim())),
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
    let json = cobalt_args(&args);
    let download_url: &str = args.c_url.as_ref().unwrap();

    let request = reqwest::Client::new().post("https://co.wuk.sh/api/json")
        .header("User-Agent", &format!("tcobalt {}", VERSION.trim()))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Accept-Language", &args.accept_language)
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
        "stream" | "redirect" | "success" | "picker" => {
            if debug { eprintln!("[DEBUG {download_url}] Cobalt returned a response") };

            let url = get_url(&args, &status, &json);

            let media = if args.c_audio_only {
                "audio"
            } else {
                "video"
            };

            let stream_request = reqwest::Client::new().get(url)
                .header("User-Agent", &format!("tcobalt {}", VERSION.trim()));

            let res = attempt!(stream_request.send().await, "Live renderer did not respond:\n\"{}\"\n(when downloading from {download_url})");

            if debug { eprintln!("[DEBUG {download_url}] Response received from stream") };
            let filename = extract_filename(&args, res.headers(), bulk, debug);
            println!(
                "Downloading {} from {} ...", 
                media,
                download_url
            );
            let stream = attempt!(res.bytes().await, "Error decoding byte stream:\n\"{}\"\n(when downloading from {download_url})");

            let display = match filename.contains(' ') {
                true => {
                    format!("'{}'", &filename)
                },
                false => filename.clone(),
            };

            println!("Data downloaded successfully! Writing {media} to {} ...", &display);
            let path = std::env::current_dir().unwrap().join(&filename);
            attempt!(tokio::fs::write(path, stream).await, "Unable to write data to file:\n\"{}\"\n(when writing to {display})");

            println!("Your {media} is ready! >> {display}")
        },
        "rate-limit" => {
            eprintln!("You are being rate limited by cobalt! Please try again later.\n(when downloading from {download_url})");
            return false;
        }
        _ => unreachable!()
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

fn get_url(args: &Args, status: &str, json: &std::collections::HashMap<String, json::JsonValue>) -> String {
    let media = if args.c_audio_only {
        "audio"
    } else {
        "video"
    };

    if status == "picker" {
        let urls = {
            let mut urls: Vec<String> = Vec::new();
            let picker_array = json.get("picker").unwrap().get_array().unwrap();
            for picker in picker_array.iter() {
                urls.push(picker.get_object().unwrap().get("url").unwrap().get_str().unwrap());
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

const POST_TEMPLATE: &str = "{
    \"url\": \"<url>\",
    \"vCodec\": \"<vcodec>\",
    \"vQuality\": \"<vquality>\",
    \"aFormat\": \"<aformat>\",
    \"filenamePattern\": \"<fname-style>\",
    \"isAudioOnly\": <audio-only>,
    \"isTTFullAudio\": <tt-full-audio>,
    \"tiktokH265\": <tt-h265>,
    \"isAudioMuted\": <audio-muted>,
    \"dubLang\": <dublang>,
    \"disableMetadata\": <no-metadata>,
    \"twitterGif\": <twitter-gif>
}";
fn cobalt_args(args_in: &Args) -> String {
    POST_TEMPLATE.to_string()
        .replace("<url>", &args_in.c_url.clone().unwrap())
        .replace("<vcodec>", &args_in.c_video_codec.print())
        .replace("<vquality>", &args_in.c_video_quality.to_string())
        .replace("<aformat>", &args_in.c_audio_format.print())
        .replace("<fname-style>", &args_in.c_fname_style.print())
        .replace("<tt-full-audio>", &args_in.c_tt_full_audio.to_string())
        .replace("<tt-h265>", &args_in.c_tt_h265.to_string())
        .replace("<audio-only>", &args_in.c_audio_only.to_string())
        .replace("<audio-muted>", &args_in.c_audio_muted.to_string())
        .replace("<dublang>", &args_in.c_dublang.to_string())
        .replace("<no-metadata>", &args_in.c_disable_metadata.to_string())
        .replace("<twitter-gif>", &args_in.c_twitter_gif.to_string())
}

#[cfg(test)]
mod tests;
