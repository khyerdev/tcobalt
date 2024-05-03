use std::path::PathBuf;

#[cfg(unix)]
const CONFIG_PATH: &str = "$HOME/.config/tcobalt.conf";
#[cfg(target_os = "windows")]
const CONFIG_PATH: &str = "$LOCALAPPDATA/tcobalt.conf";

pub fn load_config_into(args: &mut Vec<String>, instance_list: &mut String) {
    let path = PathBuf::from(CONFIG_PATH);

    if path.exists() {
        let options = get_config("default");
        for line in options.lines() {
            let option: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
            if option.len() != 2 {
                continue;
            }

            match option[0] {
                "vcodec" => {},
                "vquality" => {},
                "aformat" => {},
                "audio-only" => {},
                "mute-audio" => {},
                "twitter-gif" => {},
                "tt-full-audio" => {},
                "tt-h265" => {},
                "dublang" => {},
                "no-metadata" => {},
                "fname-style" => {},
                "instance" => {},
                _ => ()
            }
        }
    }
}

fn get_config(symbol: &str) -> String {
    let text = CONFIG_PATH;

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

