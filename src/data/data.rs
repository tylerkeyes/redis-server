use std::collections::HashMap;

use crate::data::types::{StoredType};


// TODO: need to pass 'redis' data, then serialize to string format
pub fn serialize(data: &str) -> &str {
    return "";
}

pub fn deserialize(data: &str) -> StoredType {
    let characters: Vec<char> = data.chars().collect();
    let char_type = characters.get(0).unwrap();

    let result = match char_type {
        '+' => handle_simple_string(characters),
        '-' => handle_simple_errors(characters),
        ':' => handle_integer(characters),
        '$' => handle_bulk_string(characters),
        '*' => handle_array(characters),
        '_' => handle_null(characters),
        '#' => handle_boolean(characters),
        ',' => handle_double(characters),
        '(' => handle_big_number(characters),
        '!' => handle_bulk_error(characters),
        '=' => handle_verbatim_string(characters),
        '%' => handle_map(characters),
        '~' => handle_set(characters),
        '>' => handle_push(characters),
        _ => handle_simple_errors(characters),
    };

    println!("result: {:?}", result);
    return result;
}

fn handle_simple_string(chars: Vec<char>) -> StoredType {
    println!("handle simple string: {:?}", chars);
    let mut result = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        result.push(*chars.get(i).unwrap());
        i += 1;
    }
    StoredType::SimpleString(result)
}

fn handle_simple_errors(chars: Vec<char>) -> StoredType {
    println!("handle simple error: {:?}", chars);
    StoredType::SimpleError(chars.iter().collect())
}

fn handle_integer(chars: Vec<char>) -> StoredType {
    println!("handle integer: {:?}", chars);

    let mut number = String::from("");
    let mut i = 1;
    // handle optional sign of number
    let mut sign = 1;
    if *chars.get(i).unwrap() == '-' {
        sign = -1;
        i += 1;
    } else if *chars.get(i).unwrap() == '+' {
        i += 1;
    }

    while *chars.get(i).unwrap() != '\r' {
        number.push(*chars.get(i).unwrap());
        i += 1;
    }

    println!("number: {}", number);
    let numeric = number.parse::<isize>().unwrap_or_default();
    println!("numeric: {}", numeric);
    let result = numeric * sign;
    println!("handle_integer: {}", result);
    StoredType::Integer(result)
}

fn handle_bulk_string(chars: Vec<char>) -> StoredType {
    // TODO: finish
    println!("handle bulk string: {:?}", chars);
    let mut length = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        length.push(*chars.get(i).unwrap());
        i += 1;
    }
    println!("bulk string length: {}", length);
    
    i += 2; // move counter
    let mut data = String::from("");
    while *chars.get(i).unwrap() != '\r' {
        data.push(*chars.get(i).unwrap());
        i += 1;
    }
    println!("bulk string data: {}", data);

    StoredType::BulkString(0, "".to_string())
}

fn handle_array(chars: Vec<char>) -> StoredType {
    StoredType::Array(0, vec![])
}

fn handle_null(chars: Vec<char>) -> StoredType {
    StoredType::Null
}

fn handle_boolean(chars: Vec<char>) -> StoredType {
    StoredType::Boolean(false)
}

fn handle_double(chars: Vec<char>) -> StoredType {
    StoredType::Double(0, 0)
}

fn handle_big_number(chars: Vec<char>) -> StoredType {
    StoredType::BigNumber(String::from("0"))
}

fn handle_bulk_error(chars: Vec<char>) -> StoredType {
    StoredType::BulkError(0, String::from(""))
}

fn handle_verbatim_string(chars: Vec<char>) -> StoredType {
    StoredType::VerbatimString(0, String::from("txt"), String::from(""))
}

fn handle_map(chars: Vec<char>) -> StoredType {
    StoredType::Map(0, HashMap::new())
}

fn handle_set(chars: Vec<char>) -> StoredType {
    StoredType::Set(0, vec![])
}

fn handle_push(chars: Vec<char>) -> StoredType {
    StoredType::Push(0, vec![])
}
