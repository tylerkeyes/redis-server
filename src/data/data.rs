use crate::data::types::{Shared, StoredType, StoredTypeKind};

use super::types::StoredSimpleString;

// TODO: need to pass 'redis' data, then serialize to string format
pub fn serialize(data: &str) -> &str {
    return "";
}

pub fn deserialize(data: &str) -> &str {
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

    return "";
}

fn handle_simple_string(chars: Vec<char>) -> StoredTypeKind {
    println!("handle simple string: {:?}", chars);
    StoredTypeKind::SimpleString(chars.iter().collect())
}

fn handle_simple_errors(chars: Vec<char>) -> StoredTypeKind {
    println!("handle simple error: {:?}", chars);
    StoredTypeKind::SimpleError(chars.iter().collect())
}

fn handle_integer(chars: Vec<char>) -> StoredTypeKind {
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
    StoredTypeKind::Integer(result)
}

fn handle_bulk_string(chars: Vec<char>) -> StoredTypeKind {
    println!("handle bulk string: {:?}", chars);
    let mut length = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        length.push(*chars.get(i).unwrap());
        i += 1;
    }

    StoredTypeKind::BulkString(0, "".to_string())
}

fn handle_array(chars: Vec<char>) -> StoredTypeKind {}

fn handle_null(chars: Vec<char>) -> StoredTypeKind {}

fn handle_boolean(chars: Vec<char>) -> StoredTypeKind {}

fn handle_double(chars: Vec<char>) -> StoredTypeKind {}

fn handle_big_number(chars: Vec<char>) -> StoredTypeKind {}

fn handle_bulk_error(chars: Vec<char>) -> StoredTypeKind {}

fn handle_verbatim_string(chars: Vec<char>) -> StoredTypeKind {}

fn handle_map(chars: Vec<char>) -> StoredTypeKind {}

fn handle_set(chars: Vec<char>) -> StoredTypeKind {}

fn handle_push(chars: Vec<char>) -> StoredTypeKind {}
