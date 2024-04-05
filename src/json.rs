use std::collections::HashMap;

pub fn parse(json: impl ToString) -> Result<HashMap<String, JsonValue>, String> {
    let json = json.to_string();

    let mut ignore_whitespace = true;
    let mut escape = false;
    let mut can_continue = false;
    let mut closing_expect = false;
    let mut str_finish = false;
    let mut reading = Reader::None;
    let mut global_object = false;
    let mut object_level: u16 = 0;
    let mut array_level: u16 = 0;
    let mut history: Vec<ReadHistory> = Vec::new();
    let mut key_level: u8 = 0;

    let mut key_buf: String = String::new();
    let mut key_array: Vec<String> = Vec::new();
    let mut str_buf = String::new();
    let mut num_buf = String::new();
    let mut is_float = false;
    let mut bool_buf = String::new();
    let mut null_buf = String::new();
    let mut array_storage: Vec<Vec<JsonValue>> = Vec::new();
    let mut temp_array: Vec<JsonValue> = Vec::new();
    let mut current_array: Vec<JsonValue> = Vec::new();
    let mut subobject_storage: Vec<HashMap<String, JsonValue>> = Vec::new();
    let mut temp_object: HashMap<String, JsonValue> = HashMap::new();
    let mut current_subobject: HashMap<String, JsonValue> = HashMap::new();

    let mut parsed_map: HashMap<String, JsonValue> = HashMap::new();
    for (i, c) in json.chars().enumerate() {
        if !((c.is_control() || c == ' ') && ignore_whitespace) {
            if object_level == 0 {
                if global_object == false {
                    if c == '{' {
                        object_level = 1;
                        global_object = true;
                        continue
                    }
                    return Err("First non-whitespace character has to be '{'".to_string())
                }
                break
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
                0 => match reading { // TODO: support empty objects and empty arrays
                    Reader::None => match c {
                        '"' => {
                            reading = Reader::Key;
                            ignore_whitespace = false;
                        },
                        _ => return Err(format!("Double quote expected, got '{c}' at char {i}"))
                    },
                    Reader::Key => match c {
                        '"' => {
                            reading = Reader::None;
                            key_level = 1;
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
                        key_level = 2
                    } else {
                        return Err(format!("Colon expected, got '{c}' at char {i}"))
                    }
                },
                2 => { 
                    let mut is_array = false;
                    if let Some(first_hist) = history.last() {
                        if first_hist == &ReadHistory::Array {
                            is_array = true;
                        }
                    }
                    if (c == ',' || c == '}' || (c == ']' && is_array)) && (reading != Reader::None || closing_expect || str_finish) && reading != Reader::ValStr {
                        reading = Reader::None;
                        if !can_continue {
                            return Err(format!("Character {c} placed too early at char {i}"));
                        }
                        let mut already_checked_cb = false;
                        if let Some(hist) = history.pop() {
                            match hist {
                                ReadHistory::Str => {
                                    str_finish = false;
                                },
                                ReadHistory::Num => {
                                    if num_buf.chars().last() == Some('.') {
                                        return Err(format!("Expected digits after decimal, got: {c} at char {i}"));
                                    }
                                    match object_level {
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
                                                    current_subobject.insert(key_buf.clone(), JsonValue::Float(num_buf.clone().parse().unwrap()));
                                                },
                                                false => {
                                                    current_subobject.insert(key_buf.clone(), JsonValue::Int(num_buf.clone().parse().unwrap()));
                                                },
                                            }
                                        }
                                    }
                                    is_float = false;
                                    ignore_whitespace = true;
                                    key_buf.clear();
                                    num_buf.clear();
                                }
                                ReadHistory::Bool => {
                                    if !(bool_buf == "true" || bool_buf == "false") {
                                        return Err(format!("Incomplete bool value with char {c} at char {i}"));
                                    }
                                    match object_level {
                                        1 => {
                                            parsed_map.insert(key_buf.clone(), JsonValue::Bool(bool_buf.clone().parse().unwrap()));
                                        }
                                        _ => {
                                            current_subobject.insert(key_buf.clone(), JsonValue::Bool(bool_buf.clone().parse().unwrap()));
                                        }
                                    }
                                    ignore_whitespace = true;
                                    key_buf.clear();
                                    bool_buf.clear();
                                },
                                ReadHistory::Null => {
                                    if null_buf != "null" {
                                        return Err(format!("Incomplete null value with char {c} at char {i}"));
                                    }
                                    match object_level {
                                        1 => {
                                            parsed_map.insert(key_buf.clone(), JsonValue::Null);
                                        }
                                        _ => {
                                            current_subobject.insert(key_buf.clone(), JsonValue::Null);
                                        }
                                    }
                                    ignore_whitespace = true;
                                    key_buf.clear();
                                    null_buf.clear();
                                },
                                ReadHistory::Array => { // NOTE: incomplete
                                    array_level -= 1;
                                    match object_level {
                                        1 => {
                                            parsed_map.insert(key_buf.clone(), JsonValue::Array(current_array.clone()));
                                        }
                                        _ => {
                                            current_subobject.insert(key_buf.clone(), JsonValue::Array(current_array.clone()));
                                        }
                                    }
                                    current_array.clear();
                                    key_buf.clear();
                                },
                                ReadHistory::Object => {
                                    already_checked_cb = true;
                                    object_level -= 1;
                                    match object_level {
                                        1 => {
                                            parsed_map.insert(key_array.pop().unwrap(), JsonValue::Object(current_subobject.clone()));
                                            current_subobject.clear();
                                        }
                                        0 => break,
                                        _ => {
                                            temp_object = current_subobject.clone();
                                            current_subobject = subobject_storage.pop().unwrap();
                                            current_subobject.insert(key_array.pop().unwrap(), JsonValue::Object(temp_object.clone()));
                                            temp_object.clear();
                                        }
                                    }
                                }
                            }
                        }
                        if c == ',' {
                            key_level = 0;
                            closing_expect = false;
                            can_continue = false;
                        } else if c == '}' && !already_checked_cb {
                            object_level -= 1;
                            match object_level { // i love boilerplate code (using a closure will use a mutable reference for pretty much every value mentioned here)
                                1 => {
                                    assert_eq!(history.pop().unwrap(), ReadHistory::Object);
                                    parsed_map.insert(key_array.pop().unwrap(), JsonValue::Object(current_subobject.clone()));
                                    current_subobject.clear();
                                }
                                0 => break,
                                _ => {
                                    assert_eq!(history.pop().unwrap(), ReadHistory::Object);
                                    temp_object = current_subobject.clone();
                                    current_subobject = subobject_storage.pop().unwrap();
                                    current_subobject.insert(key_array.pop().unwrap(), JsonValue::Object(temp_object.clone()));
                                    temp_object.clear();
                                }
                            }
                            closing_expect = true;
                        }
                        continue;
                    }
                    if closing_expect {
                        return Err(format!("Expected comma or closing bracket at char {i}"))
                    }
                    match reading {
                        Reader::None => match c {
                            '{' => {
                                match object_level {
                                    0 => unreachable!(),
                                    1 => {
                                        key_array.push(key_buf.clone());
                                        key_buf.clear();
                                    },
                                    u16::MAX => return Err(format!("Maximum object nesting reached at char {i}")),
                                    _ => {
                                        subobject_storage.push(current_subobject.clone());
                                        current_subobject.clear();
                                        key_array.push(key_buf.clone());
                                        key_buf.clear();
                                    }
                                }
                                object_level += 1;
                                history.push(ReadHistory::Object);
                                key_level = 0;
                            },
                            '[' => {
                                array_level += 1;
                                history.push(ReadHistory::Array);
                            },
                            '\"' => {
                                reading = Reader::ValStr;
                                ignore_whitespace = false;
                                history.push(ReadHistory::Str);
                                can_continue = true;
                            },
                            '0'..='9' => {
                                reading = Reader::ValNum;
                                num_buf.push(c);
                                history.push(ReadHistory::Num);
                                ignore_whitespace = false;
                                can_continue = true;
                            },
                            '.' => {
                                reading = Reader::ValNum;
                                num_buf.push('0');
                                num_buf.push(c);
                                ignore_whitespace = false;
                                can_continue = true;
                                history.push(ReadHistory::Num);
                            }
                            't' | 'f' => {
                                reading = Reader::ValBool;
                                bool_buf.push(c);
                                ignore_whitespace = false;
                                history.push(ReadHistory::Bool);
                                can_continue = true;
                            },
                            'n' => {
                                reading = Reader::ValNull;
                                null_buf.push(c);
                                ignore_whitespace = false;
                                history.push(ReadHistory::Null);
                                can_continue = true;
                            },
                            e => return Err(format!("Invalid character: {e} at char {i}"))
                        }
                        Reader::Key => unreachable!(),
                        Reader::ValStr => match c {
                            '"' => {
                                reading = Reader::None;
                                ignore_whitespace = true;
                                match object_level {
                                    1 => {
                                        parsed_map.insert(key_buf.clone(), JsonValue::Str(str_buf.clone()));
                                    },
                                    _ => {
                                        current_subobject.insert(key_buf.clone(), JsonValue::Str(str_buf.clone()));
                                    }
                                }
                                key_buf.clear();
                                str_buf.clear();
                                str_finish = true;
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
                            // TODO: add closing bracket
                            o => return Err(format!("Invalid character '{o}' found in number at char {i}"))
                        },
                        Reader::ValBool => match bool_buf.as_str() {
                            "t" => if c == 'r' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                            "tr" => if c == 'u' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                            "tru" => if c == 'e' { ignore_whitespace = true; bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                            "f" => if c == 'a' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                            "fa" => if c == 'l' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                            "fal" => if c == 's' { bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                            "fals" => if c == 'e' { ignore_whitespace = true; bool_buf.push(c) } else { return Err(format!("Invalid character in bool: {c}")) }
                            _ => return Err(format!("Invalid character after bool: {c}"))
                        },
                        Reader::ValNull => match null_buf.as_str() {
                            "n" => if c == 'u' { null_buf.push(c) } else { return Err(format!("Invalid character in nullval: {c}")) }
                            "nu" => if c == 'l' { null_buf.push(c) } else { return Err(format!("Invalid character in nullval: {c}")) }
                            "nul" => if c == 'l' { null_buf.push(c) } else { return Err(format!("Invalid character in nullval: {c}")) }
                            _ => return Err(format!("Invalid character after null: {c}"))
                        }
                    }
                },
                _ => unreachable!()
            }
        }
    };

    Ok(parsed_map)
}

#[allow(unused)]
#[derive(PartialEq, Eq, Debug)]
enum Reader {
    None,
    Key,
    ValStr,
    ValNum,
    ValBool,
    ValNull
}
#[allow(unused)]
#[derive(PartialEq, Eq, Debug)]
enum ReadHistory {
    Str, Num, Bool, Null, Array, Object
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Str(String),
    Int(i128),
    Float(f64),
    Bool(bool),
    Null,
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}
#[allow(unused)]
impl JsonValue {
    pub fn type_of<'a>(&self) -> &'a str {
        match self {
            JsonValue::Str(_) => "string",
            JsonValue::Int(_) => "int",
            JsonValue::Float(_) => "float",
            JsonValue::Bool(_) => "bool",
            JsonValue::Null => "null",
            JsonValue::Array(_) => "array",
            JsonValue::Object(_) => "object",
        }
    }
    pub fn get_str(&self) -> Result<String, bool> {
        if let Self::Str(val) = self {
            return Ok(val.to_string())
        }
        Err(false)
    }
    pub fn get_int(&self) -> Result<i128, bool> {
        if let Self::Int(val) = self {
            return Ok(*val)
        }
        Err(false)
    }
    pub fn get_float(&self) -> Result<f64, bool> {
        if let Self::Float(val) = self {
            return Ok(*val)
        }
        Err(false)
    }
    pub fn get_bool(&self) -> Result<bool, bool> {
        if let Self::Bool(val) = self {
            return Ok(*val)
        }
        Err(false)
    }
    pub fn get_null(&self) -> Result<(), bool> {
        if let Self::Null = self {
            return Ok(())
        }
        Err(false)
    }
    pub fn get_array(&self) -> Result<Vec<JsonValue>, bool> {
        if let Self::Array(val) = self {
            return Ok(val.clone())
        }
        Err(false)
    }
    pub fn get_object(&self) -> Result<HashMap<String, JsonValue>, bool> {
        if let Self::Object(val) = self {
            return Ok(val.clone())
        }
        Err(false)
    }
}
