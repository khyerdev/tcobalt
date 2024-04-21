mod json;
mod args;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

use args as tcargs;
use args::Args;

const VERSION: &str = include_str!("version");

#[tokio::main]
async fn main() -> std::process::ExitCode {
    if std::env::args().len() == 1 {
        println!("tcobalt Command Line Utility; run `tc help` for help");
        return std::process::ExitCode::SUCCESS;
    }

    let _placeholder = "https://www.youtube.com/watch?v=zn5sTDXSp8E";
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
            let success = execute_get_media(args, 0).await;
            if !success {
                return std::process::ExitCode::FAILURE;
            }
        },
        args::types::Method::Bulk => {
            let failed: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
            let mut futures_array: Vec<Pin<Box<dyn std::future::Future<Output = ()>>>> = Vec::new();
            let mut i = 0;
            args.bulk_array.unwrap().iter().for_each(|a| {
                if args.same_filenames {
                    i += 1;
                }
                let args = a.clone();
                let switch = Arc::clone(&failed);
                let task = async move {
                    let success = execute_get_media(args, i).await;
                    if !success {
                        let mut lock = switch.write().await;
                        *lock = true;
                    }
                };
                futures_array.push(Box::pin(task));
            });

            futures::future::join_all(futures_array).await;

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
            let ver = match request.send().await {
                Ok(res) => res.text().await.unwrap_or("{\"version\":\"unknown\",\"commit\":\"unknown\",\"branch\":\"unknown\"}".to_string()),
                Err(e) => {
                    eprintln!("Cobalt server did not respond: {}", e.to_string());
                    return std::process::ExitCode::FAILURE;
                }
            };
            let stats = match json::parse(&ver) {
                Ok(j) => j,
                Err(e) => {
                    print_cobalt_error(e, ver);
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

async fn execute_get_media(args: Args, bulk: u16) -> bool {
    let json = cobalt_args(&args);
    let download_url: &str = args.c_url.as_ref().unwrap();

    let request = reqwest::Client::new().post("https://co.wuk.sh/api/json")
        .header("User-Agent", &format!("tcobalt {}", VERSION.trim()))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(json);

    match request.send().await {
        Ok(res) => {
            let body = res.text().await.unwrap();
            match json::parse(&body) {
                Ok(json) => {
                    match json.get("status".into()).unwrap().get_str().unwrap().as_str() {
                        "error" => {
                            let text = json.get("text").unwrap().get_str().unwrap();
                            eprintln!("Cobalt returned error: {text} (when downloading from {download_url})");
                            return false;
                        },
                        "stream" | "redirect" => {
                            let url = json.get("url").unwrap().get_str().unwrap();
                            let stream_request = reqwest::Client::new().get(url)
                                .header("User-Agent", &format!("tcobalt {}", VERSION.trim()));

                            match stream_request.send().await {
                                Ok(res) => {
                                    let filename = match args.out_filename {
                                        Some(name) => {
                                            if bulk > 0 {
                                                format!("{bulk}-{name}")
                                            } else {
                                                name
                                            }
                                        },
                                        None => {
                                            let disposition = res.headers().get("Content-Disposition").unwrap().to_str().unwrap();
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
                                            }
                                            filename
                                        }
                                    };
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
                                    match res.bytes().await {
                                        Ok(stream) => {
                                            println!("Data downloaded successfully! Writing {media} to {} ...", &filename);
                                            let path = std::env::current_dir().unwrap().join(&filename);
                                            match tokio::fs::write(path, stream).await {
                                                Ok(_) => {
                                                    println!("Your {media} is ready! >> {filename}")
                                                },
                                                Err(e) => {
                                                    eprintln!("Unable to write data to file: {} (when writing to {filename})", e.to_string());
                                                    return false;
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("Error decoding byte stream: {} (when downloading from {download_url})", e.to_string());
                                            return false;
                                        }
                                    }
                                },
                                Err(e) => {
                                    eprintln!("Live renderer did not respond: {} (when downloading from {download_url})", e.to_string());
                                    return false;
                                }
                            }
                        },
                        "rate-limit" => {
                            eprintln!("You are being rate limited by cobalt! Please try again later. (when downloading from {download_url})");
                            return false;
                        }
                        _ => unimplemented!()
                    }
                },
                Err(e) => {
                    print_cobalt_error(e, body);
                    return false;
                }
            }
        },
        Err(e) => {
            eprintln!("Cobalt server did not respond: {} (when downloading from {download_url})", e.to_string());
        }
    }
    true
}

fn print_cobalt_error(error: String, body: String) {
    eprintln!("Cobalt server returned improper JSON");
    eprintln!("JSON parse error: {error}");
    if std::env::var("TCOBALT_DEBUG").is_ok_and(|v| v == 1.to_string()) == true {
        eprintln!("\nCobalt returned response: {}\n\n", body);
        eprintln!("If this response isn't proper JSON, please contact wukko about this error.");
        eprintln!("If this looks like proper json, contact khyernet about his json parser not functioning right");
    } else {
        eprintln!("Contact wukko about this error. Run with TCOBALT_DEBUG=1 to see the incorrect response.")
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
