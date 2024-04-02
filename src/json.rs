use std::collections::HashMap;

pub fn parse(json: impl ToString) -> Result<HashMap<String, JsonValue>, String> {
    let json = json.to_string();

    let mut ignore_whitespace = true;
    let mut escape = false;
    let mut reading = Reader::None;
    let mut global_set = false;
    let mut set_level: u16 = 0;
    let mut list_level: u16 = 0;
    let mut history: Vec<ReadHistory> = Vec::new();
    let mut key_level: u8 = 0;

    let mut key_buf: String = String::new();
    let mut key_list: Vec<String> = Vec::new();
    let mut str_buf = String::new();
    let mut num_buf = String::new();
    let mut is_float = false;
    let mut bool_buf = String::new();
    let mut null_buf = String::new();
    let mut list_buf: Vec<Vec<JsonValue>> = Vec::new();
    let mut current_list: Vec<JsonValue> = Vec::new();
    let mut subset_buf: Vec<HashMap<String, JsonValue>> = Vec::new();
    let mut current_subset: HashMap<String, JsonValue> = HashMap::new();

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
                2 => match reading { // NOTE: add comma and closing bracket checking beforehand
                    Reader::None => match c { // TODO: i have alot of work to do about nested sets and lists.
                        '{' => {
                            set_level += 1;
                        },
                        '}' => {
                            set_level -= 1;
                        },
                        '[' => {
                            list_level += 1;
                        },
                        ']' => {
                            list_level -= 1;
                        },
                        '"' => {
                            reading = Reader::ValStr;
                            ignore_whitespace = false;
                        },
                        '0'..='9' => { reading = Reader::ValNum; num_buf.push(c) },
                        '.' => { reading = Reader::ValNum; num_buf.push('0'); num_buf.push(c) }
                        't' | 'f' => { reading = Reader::ValBool; bool_buf.push(c); ignore_whitespace = false },
                        'n' => { reading = Reader::ValNull; null_buf.push(c); ignore_whitespace = false },
                        e => return Err(format!("Invalid character: {e}"))
                    }
                    Reader::Key => unreachable!(),
                    Reader::ValStr => match c {
                        '"' => {
                            reading = Reader::None;
                            ignore_whitespace = true;
                            match set_level {
                                1 => {
                                    parsed_map.insert(key_buf.clone(), JsonValue::Str(str_buf.clone()));
                                },
                                _ => {
                                    current_subset.insert(key_buf.clone(), JsonValue::Str(str_buf.clone()));
                                }
                            }
                            key_buf.clear();
                            str_buf.clear();
                            key_level = 3;
                        },
                        _ => str_buf.push(c)
                    },
                    Reader::ValNum => match c {
                        '0'..='9' => num_buf.push(c),
                        '.' => {
                            if !is_float {
                                is_float = true;
                                num_buf.push(c);
                            } else {
                                return Err("Floats can only have one decimal character".to_string())
                            }
                        },
                        ',' => {
                            match set_level {
                                1 => {
                                    match is_float {
                                        true => {
                                            parsed_map.insert(key_buf.clone(), JsonValue::Float(num_buf.clone().parse().unwrap()));
                                        },
                                        false => {
                                            parsed_map.insert(key_buf.clone(), JsonValue::Int(num_buf.clone().parse().unwrap()));
                                        },
                                    }
                                },
                                _ => {
                                    match is_float {
                                        true => {
                                            current_subset.insert(key_buf.clone(), JsonValue::Float(num_buf.clone().parse().unwrap()));
                                        },
                                        false => {
                                            current_subset.insert(key_buf.clone(), JsonValue::Int(num_buf.clone().parse().unwrap()));
                                        },
                                    }
                                }
                            }
                            is_float = false;
                            key_buf.clear();
                            num_buf.clear();
                            key_level = 0;
                        }
                        // TODO: add closing bracket
                        o => return Err(format!("Invalid character '{o}' found in number"))
                    },
                    Reader::ValBool => match bool_buf.as_str() {
                        "t" => if c == 'r' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                        "tr" => if c == 'u' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                        "tru" => if c == 'e' { ignore_whitespace = true; bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                        "f" => if c == 'a' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                        "fa" => if c == 'l' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                        "fal" => if c == 's' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                        "fals" => if c == 'e' { ignore_whitespace = true; bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                        "true" | "false" => {
                            if c == ',' {
                                match set_level {
                                    1 => {
                                        parsed_map.insert(key_buf.clone(), JsonValue::Bool(bool_buf.clone().parse().unwrap()));
                                    }
                                    _ => {
                                        current_subset.insert(key_buf.clone(), JsonValue::Bool(bool_buf.clone().parse().unwrap()));
                                    }
                                }
                                ignore_whitespace = true;
                                key_buf.clear();
                                bool_buf.clear();
                                key_level = 0;
                            } // TODO: i also need to add else
                        },
                        _ => unreachable!()
                    },
                    Reader::ValNull => match null_buf.as_str() {
                        "n" => if c == 'u' { bool_buf.push(c) } else { return Err(format!("Invalid character in nullval: {c}")) }
                        "nu" => if c == 'l' { bool_buf.push(c) } else { return Err(format!("Invalid character in nullval: {c}")) }
                        "nul" => if c == 'l' { bool_buf.push(c) } else { return Err(format!("Invalid character in nullval: {c}")) }
                        "null" => {
                            if c == ',' {
                                match set_level {
                                    1 => {
                                        parsed_map.insert(key_buf.clone(), JsonValue::Null);
                                    }
                                    _ => {
                                        current_subset.insert(key_buf.clone(), JsonValue::Null);
                                    }
                                }
                                ignore_whitespace = true;
                                key_buf.clear();
                                null_buf.clear();
                                key_level = 0;
                            }
                        },
                        _ => unreachable!()
                    }
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
enum ReadHistory {
    Str, Num, Bool, Null, List, Set
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
