const USAGE_TXT: &str = include_str!("usage.txt");
const INFO_TXT: &str = include_str!("info.txt");

pub fn get_help() -> String {
    let mut string = String::new();
    for line in USAGE_TXT.lines().into_iter() {
        if line.chars().collect::<Vec<char>>().first() == Some(&'[') { break }
        string.push_str(line);
        string.push('\n');
    }
    remove_trailing_whitespace(string)
}

pub fn get_str(module: &str, symbol: &str) -> String {
    let text = match module {
        "usage" => USAGE_TXT,
        "info" => INFO_TXT,
        _ => unreachable!()
    };

    let mut string = String::new();
    let mut select = false;
    let mut brackets = 0;
    for line in text.lines().into_iter() {
        if brackets == 2 { break }
        if !select && remove_trailing_whitespace(line) != format!("[{symbol}]") { continue }
        select = true;
        if line.chars().collect::<Vec<char>>().first() == Some(&'[') { brackets += 1; continue }

        string.push_str(line);
        string.push('\n');
    }
    remove_trailing_whitespace(string)
}

pub fn remove_trailing_whitespace(string: impl ToString) -> String {
    let mut string = string.to_string();
    if string.len() == 0 { return string }
    let chars = string.chars().collect::<Vec<char>>();
    let mut i = string.len()-1;
    while chars.get(i) == Some(&'\n') || chars.get(i) == Some(&' ') {
        string.pop();
        i -= 1;
    }
    string
}
