mod json;
mod args;
mod strings;
mod process;

use process as proc;
use args as tcargs;
use args::Args;

use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

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
            args::types::Help::GenConfig => println!("{}", strings::get_str("usage", "gen-config")),
            args::types::Help::Config => println!("{}", strings::get_str("usage", "config")),
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
        args::types::Method::CobaltVersion(api_url) => {
            let request = reqwest::Client::new().get(format!("https://{}/", api_url))
                .header("User-Agent", &format!("tcobalt {}", VERSION.trim()));
            if debug { eprintln!("[DEBUG] Sending GET request to cobalt ...") };
            let ver = match request.send().await {
                Ok(res) => {
                    res.text().await.expect("cobalt returned nothing")
                },
                Err(e) => {
                    eprintln!("Cobalt server did not respond: {}", e.to_string());
                    return std::process::ExitCode::FAILURE;
                }
            };
            if debug { eprintln!("[DEBUG] Response received, parsing json ...") };
            let stats = match json::parse(&ver) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("{}", proc::print_json_error(e, ver));
                    return std::process::ExitCode::FAILURE;
                }
            };
            let cobalt = stats.get("cobalt").unwrap().get_object().unwrap();
            let git = stats.get("git").unwrap().get_object().unwrap();
            let version = cobalt.get("version").unwrap().get_str().unwrap();
            let commit = git.get("commit").unwrap().get_str().unwrap();
            let branch = git.get("branch").unwrap().get_str().unwrap();
            println!("Cobalt (by wukko and jj) version {version}");
            println!("Latest commit on branch \"{branch}\": {commit}");
        },
        args::types::Method::GenConfig => {
            let text = strings::get_str("info", "default-config").replace("\\", "");
            let path = std::path::PathBuf::from({
                if cfg!(target_os = "windows") {
                    "$CFG\\tcobalt.conf".replace("$CFG", &std::env::var("LOCALAPPDATA").expect("no localappdata var"))
                } else {
                    "$CFG/.config/tcobalt.conf".replace("$CFG", &std::env::var("HOME").expect("no home var"))
                }
            });

            if let Ok(()) = std::fs::write(&path, text) {
                println!("Wrote default config to {}", path.to_string_lossy());
                return std::process::ExitCode::SUCCESS
            } else {
                return std::process::ExitCode::FAILURE
            }
        }
    }
    std::process::ExitCode::SUCCESS
}

async fn execute_get_media(args: Args, bulk: u16, debug: bool) -> bool {
    let json = proc::cobalt_args(&args);
    let download_url: &str = args.c_url.as_ref().unwrap();

    let request = reqwest::Client::new().post(format!("https://{}/", &args.cobalt_instance))
        .header("User-Agent", &format!("tcobalt {}", VERSION.trim()))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(json);

    if debug { eprintln!("[DEBUG {download_url}] Sending POST request to cobalt server ...") };
    let res = attempt!(request.send().await, "Cobalt server did not respond:\n\"{}\"\n(when downloading from {download_url})");

    let body = res.text().await.unwrap();
    if debug { eprintln!("[DEBUG {download_url}] Response received, parsing json ...") };
    let json = attempt!(json::parse(&body), proc::print_json_error("{}".into(), body));

    let status = json.get("status".into()).unwrap().get_str().unwrap();
    match status.as_str() {
        "error" => {
            let text = json.get("error".into()).unwrap().get_object().unwrap().get("code".into()).unwrap().get_str().unwrap();
            eprintln!("Cobalt returned error: \"{text}\" (when downloading from {download_url})");
            return false;
        },
        "tunnel" | "redirect" | "picker" => {
            if debug { eprintln!("[DEBUG {download_url}] Cobalt returned a response") };

            let url = proc::get_url(&args, &status, &json);

            let media = if args.c_download_mode == tcargs::types::DownloadMode::Audio {
                "audio"
            } else {
                "video"
            };

            let stream_request = reqwest::Client::new().get(url)
                .header("User-Agent", &format!("tcobalt {}", VERSION.trim()));

            let res = attempt!(stream_request.send().await, "Live renderer did not respond:\n\"{}\"\n(when downloading from {download_url})");

            if debug { eprintln!("[DEBUG {download_url}] Response received from stream") };
            let mut filename = json.get("filename".into()).unwrap().get_str().unwrap();
            if let Some(custom_name) = args.out_filename {
                filename = custom_name;
            }
            if bulk > 0 {
                filename = format!("{bulk}_{filename}");
            }
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
        _ => unreachable!()
    }
    true
}


#[cfg(test)]
mod tests;
