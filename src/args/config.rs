use std::path::PathBuf;

#[cfg(unix)]
const CONFIG_PATH: &str = "$CFG/.config/tcobalt.conf";
#[cfg(target_os = "windows")]
const CONFIG_PATH: &str = "$CFG/tcobalt.conf";

pub fn load_config_into(args: &mut Vec<String>, instance_list: &mut Vec<String>) {
    let path = PathBuf::from({
        if cfg!(target_os = "windows") {
            CONFIG_PATH.replace("$CFG", &std::env::var("LOCALAPPDATA").expect("no localappdata var"))
        } else {
            CONFIG_PATH.replace("$CFG", &std::env::var("HOME").expect("no home var"))
        }
    });

    if path.exists() {
        let options = get_config(&path, "default");
        for line in options.lines() {
            let option: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
            if option.len() != 2 {
                continue;
            }

            match option[0].to_lowercase().as_str() {
                "vcodec" => {
                    args.push("-c".into());
                    args.push(option[1].into())
                },
                "vquality" => {
                    args.push("-q".into());
                    args.push(option[1].into())
                },
                "aformat" => {
                    args.push("-f".into());
                    args.push(option[1].into())
                },
                "audio-only" => {
                    if option[1].to_lowercase().as_str() == "true" {
                        args.push("-a".into())
                    }
                },
                "mute-audio" => {
                    if option[1].to_lowercase().as_str() == "true" {
                        args.push("-m".into())
                    }
                },
                "twitter-gif" => {
                    if option[1].to_lowercase().as_str() == "true" {
                        args.push("-g".into())
                    }
                },
                "tt-full-audio" => {
                    if option[1].to_lowercase().as_str() == "true" {
                        args.push("-u".into())
                    }
                },
                "tt-h265" => {
                    if option[1].to_lowercase().as_str() == "true" {
                        args.push("-h".into())
                    }
                },
                "dublang" => {
                    if option[1].to_lowercase().as_str() != "none" {
                        args.push("-l".into());
                        args.push(option[1].into())
                    }
                },
                "no-metadata" => {
                    if option[1].to_lowercase().as_str() == "true" {
                        args.push("-n".into())
                    }
                },
                "fname-style" => {
                    args.push("-s".into());
                    args.push(option[1].into())
                },
                "instance" => {
                    args.push("-i".into());
                    args.push(option[1].into())
                },
                _ => ()
            }
        }
        drop(options);

        let instances = get_config(&path, "default.instances");
        for line in instances.lines() {
            let mut url = line.replace("https://", "");
            if let Some(idx) = url.find('/') {
                url.truncate(idx);
            }
            instance_list.push(format!("{url}\n"));
        }
    }
}

fn get_config(path: &PathBuf, symbol: &str) -> String {
    let text = std::fs::read_to_string(path).unwrap_or("".into());

    let mut string = String::new();
    let mut select = false;
    let mut brackets = 0;

    for line in text.lines().into_iter() {
        if brackets == 2 { break }
        if !select && crate::strings::remove_trailing_whitespace(line) != format!("[{symbol}]") { continue }
        select = true;
        if line.chars().collect::<Vec<char>>().first() == Some(&'[') { brackets += 1; continue }

        string.push_str(line);
        string.push('\n');
    }
    crate::strings::remove_trailing_whitespace(string)
}

