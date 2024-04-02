use std::collections::HashMap;

pub fn parse(json: impl ToString) -> Result<HashMap<String, JsonValue>, String> {
    let json = json.to_string();

    let mut ignore_whitespace = true;
    let mut escape = false;
    let mut reading = Reader::None;
    let mut global_set = false;
    let mut set_level: u16 = 0;
    let mut list_level: u16 = 0;
    let mut subset_history: Vec<bool> = Vec::new(); // set = true, list = false
    let mut key_level: u8 = 0;

    let mut key_buf: String = String::new();
    let mut key_list: Vec<String> = Vec::new();
    let mut str_buf = String::new();
    let mut num_buf = String::new();
    let mut bool_buf = String::new();
    let mut null_buf = String::new();
    let mut list_buf: Vec<Vec<JsonValue>> = Vec::new();
    let mut subset_buf: Vec<HashMap<String, JsonValue>> = Vec::new();

    let mut parsed_map: HashMap<String, JsonValue> = HashMap::new();
    for c in json.chars() {
        if !((c.is_control() || c == ' ') && ignore_whitespace) {
            if set_level == 0 {
                if global_set == false {
                    if c == '{' {
                        set_level = 1;
                        global_set = true;
                    } else {
                        return Err("First non-whitespace character has to be '{'".to_string())
                    }
                } else {
                    break
                }
            }
            if escape {
                escape = false;
                match reading {
                    Reader::Key => key_buf.push(c),
                    Reader::ValStr => str_buf.push(c),
                    _ => unreachable!()
                }
                continue
            }
            if c == '\\' && (reading == Reader::Key || reading == Reader::ValStr) {
                escape = true;
                continue;
            }
            match key_level {
                0 => match reading {
                    Reader::None => match c {
                        '"' => {
                            reading = Reader::Key;
                            ignore_whitespace = false;
                        },
                        _ => return Err(format!("Double quote expected, got '{c}'"))
                    },
                    Reader::Key => match c {
                        '"' => {
                            reading = Reader::None;
                            key_level = 1;
                            key_list.push(key_buf.clone());
                            key_buf.clear();
                            ignore_whitespace = true;
                        },
                        _ => {
                            key_buf.push(c);
                        }
                    },
                    _ => unreachable!()
                },
                1 => { 
                    if c == ':' {
                        key_level = 1
                    } else {
                        return Err(format!("Colon expected, got '{c}'"))
                    }
                },
                2 => match reading {
                    Reader::None => match c { // TODO: i have alot of work to do about nested sets and lists.
                        '{' => {
                            set_level += 1;
                            subset_history.push(true);
                        },
                        '}' => {
                            set_level -= 1;
                            subset_history.pop();
                        },
                        '[' => {
                            list_level += 1;
                            subset_history.push(false);
                        },
                        ']' => {
                            list_level -= 1;
                            subset_history.pop();
                        },
                        '"' => {
                            reading = Reader::ValStr;
                            ignore_whitespace = false;
                        },
                        '0'..='9' => { reading = Reader::ValNum; num_buf.push(c) },
                        't' | 'f' => { reading = Reader::ValBool; bool_buf.push(c) },
                        'n' => { reading = Reader::ValNull; null_buf.push(c) },
                        e => return Err(format!("Invalid character: {e}"))
                    }
                    Reader::Key => unreachable!(),
                    Reader::ValStr => match c {
                        '"' => {
                            reading = Reader::None;
                            ignore_whitespace = true;
                            // TODO: insert into correct hashmap/list
                            key_level = 3;
                        },
                        _ => str_buf.push(c)
                    },
                    Reader::ValNum => match c {
                        '0'..='9' | '.' => num_buf.push(c),
                        ',' => {

                        }
                        // TODO: yapping
                    },
                    Reader::ValBool => todo!(),
                    Reader::ValNull => todo!(),
                },
                3 => todo!(),
                _ => unreachable!()
            }
        }
    };

    Ok(parsed_map)
}

#[derive(PartialEq, Eq)]
enum Reader {
    None,
    Key,
    ValStr,
    ValNum,
    ValBool,
    ValNull
}

pub enum JsonValue {
    Str(String),
    Int(i128),
    Float(f64),
    Bool(bool),
    Null,
    List(Vec<JsonValue>),
    Set(HashMap<String, JsonValue>)
}
