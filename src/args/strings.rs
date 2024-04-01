const RAW: &str = include_str!("strings.txt");

pub fn get_help() -> String {
    let mut string = String::new();
    for line in RAW.lines().into_iter() {
        if line.chars().collect::<Vec<char>>().first() == Some(&'[') { break }
        string.push_str(line);
        string.push('\n');
    }
    remove_trailing_whitespace(string)
}

pub fn get_mod(help_mod: &str) -> String {
    let mut string = String::new();
    let mut select = false;
    let mut brackets = 0;
    for line in RAW.lines().into_iter() {
        if brackets == 2 { break }
        if !select && remove_trailing_whitespace(line) != format!("[{help_mod}]") { continue }
        select = true;
        if line.chars().collect::<Vec<char>>().first() == Some(&'[') { brackets += 1; continue }

        string.push_str(line);
        string.push('\n');
    }
    remove_trailing_whitespace(string)
}

fn remove_trailing_whitespace(string: impl ToString) -> String {
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
