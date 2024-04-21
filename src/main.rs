mod json;
mod args;
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
            let json = cobalt_args(&args);

            let request = reqwest::Client::new().post("https://co.wuk.sh/api/json")
                .header("User-Agent", &format!("tcobalt {}", VERSION.trim()))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .body(json);

            match request.send().await {
                Ok(res) => {
                    match json::parse(res.text().await.unwrap()) {
                        Ok(json) => {
                            match json.get("status".into()).unwrap().get_str().unwrap().as_str() {
                                "error" => {
                                    let text = json.get("text").unwrap().get_str().unwrap();
                                    eprintln!("Cobalt returned error: {text}");
                                    return std::process::ExitCode::FAILURE;
                                },
                                "stream" | "redirect" => {
                                    let url = json.get("url").unwrap().get_str().unwrap();
                                    let stream_request = reqwest::Client::new().get(url)
                                        .header("User-Agent", &format!("tcobalt {}", VERSION.trim()));
                                    
                                    match stream_request.send().await {
                                        Ok(res) => {
                                            let filename = match args.out_filename {
                                                Some(name) => name,
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
                                            match res.bytes().await {
                                                Ok(stream) => {
                                                    let path = std::env::current_dir().unwrap().join(filename.clone());
                                                    match std::fs::write(path, stream) {
                                                        Ok(_) => {
                                                            eprintln!("File downloaded successfully! {filename}");
                                                        },
                                                        Err(e) => {
                                                            eprintln!("Unable to write data to file: {}", e.to_string());
                                                            return std::process::ExitCode::FAILURE;
                                                        }
                                                    }
                                                },
                                                Err(e) => {
                                                    eprintln!("Error decoding byte stream: {}", e.to_string());
                                                    return std::process::ExitCode::FAILURE;
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("Live renderer did not respond: {}", e.to_string());
                                            return std::process::ExitCode::FAILURE;
                                        }
                                    }
                                },
                                _ => unimplemented!()
                            }
                        },
                        Err(e) => {
                            print_cobalt_error(e);
                            return std::process::ExitCode::FAILURE;
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Cobalt server did not respond: {}", e.to_string());
                }
            }
        },
        args::types::Method::Bulk => {
            println!("{:#?}", args);
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
            let stats = match json::parse(ver/* .clone() */) {
                Ok(j) => j,
                Err(e) => {
                    print_cobalt_error(e);
                    // eprintln!("Cobalt returned response: {:?}", ver);
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

fn print_cobalt_error(error: String) {
    eprintln!("Cobalt server returned improper JSON");
    eprintln!("JSON parse error: {error}");
    eprintln!("Either Cobalt is down, or you somehow got blocked specifically in this application.\n");
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
