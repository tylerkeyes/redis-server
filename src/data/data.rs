use log::{error, info};
use regex::Regex;
use std::collections::HashMap;

use crate::data::types::StoredType;

// TODO: need to pass 'redis' data, then serialize to string format
pub fn serialize(data: &str) -> &str {
    return "";
}

pub fn deserialize(data: &str) -> (isize, StoredType) {
    let characters: Vec<char> = data.chars().collect();
    let char_type = characters.get(0).unwrap();

    println!("deserialize: {:?}", data);

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

    //println!("result: {:?}", result);
    (result.0, result.1)
}

fn handle_simple_string(chars: Vec<char>) -> (isize, StoredType) {
    //println!("handle simple string: {:?}", chars);
    let mut result = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        result.push(*chars.get(i).unwrap());
        i += 1;
    }
    (i as isize, StoredType::SimpleString(result))
}

fn handle_simple_errors(chars: Vec<char>) -> (isize, StoredType) {
    //println!("handle simple error: {:?}", chars);
    let mut result = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        result.push(*chars.get(i).unwrap());
        i += 1;
    }
    (i as isize, StoredType::SimpleError(result))
}

fn handle_integer(chars: Vec<char>) -> (isize, StoredType) {
    //println!("handle integer: {:?}", chars);
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
    let numeric = number.parse::<isize>().unwrap_or_default();
    let result = numeric * sign;
    (i as isize, StoredType::Integer(result))
}

fn handle_bulk_string(chars: Vec<char>) -> (isize, StoredType) {
    //println!("handle bulk string: {:?}", chars);
    let mut length = String::from("");
    let mut i = 1;
    if *chars.get(i).unwrap() == '-' {
        return (5, StoredType::Null); // $-1\r\n
    }
    while *chars.get(i).unwrap() != '\r' {
        length.push(*chars.get(i).unwrap());
        i += 1;
    }
    i += 2; // move counter
    let mut data = String::from("");
    while *chars.get(i).unwrap() != '\r' {
        data.push(*chars.get(i).unwrap());
        i += 1;
    }
    (
        i as isize,
        StoredType::BulkString(length.parse::<isize>().unwrap(), data),
    )
}

fn split_array(array_str: String) -> Vec<String> {
    /* Might be better to change this to parse out the first nested type,
     *  then return two values: (parsed first type, remainder of input String).
     *
     *  This would be a more iterative way of parsing the input String, instead
     *  of going through the entire String at one time and returning a vector of
     *  the parsed types. */
    let mut result = Vec::new();
    let simple_dict = "+-:$_#,(!=";
    let complex_dict = "*%~>";
    println!("in split array with value: {}", array_str);

    for c in array_str.chars() {
        if simple_dict.contains(c) {
            println!("type match: {}", c);
        } else if complex_dict.contains(c) {
            println!("complex type match: {}", c);
        }
    }

    result
}

fn handle_array(chars: Vec<char>) -> (isize, StoredType) {
    // TODO: finish function
    println!("handle array: {:?}", chars);
    let mut array: Vec<StoredType> = Vec::new();
    let split_chars: String = chars.into_iter().collect();

    // capture length
    let re_length = Regex::new(r"^*(\d+)\r\n").unwrap();
    let Some(capture) = re_length.captures(&split_chars) else {
        println!("error capturing length");
        return (0, StoredType::Array(0, vec![]));
    };
    let arr_len: isize = (&capture[1]).parse::<isize>().unwrap();

    // capture rest of string
    let start_idx = split_chars.find("\r\n").unwrap() + 2;
    let remaining = &split_chars[start_idx..];
    println!("DEBUG remaining: {}", remaining);

    let values = split_array(split_chars); // TODO: values in returning an empty vec, causing
                                           // unwrap to fail
    let length = values.get(0).unwrap()[1..].parse::<isize>().unwrap();
    println!("DEBUG length: {}", length);

    /*
    for i in 1..values.len() {
        if values.get(i).unwrap() != "" {
            println!("got value: {:?}", values.get(i));
            let pass_val = format!("{}{}", values.get(i).unwrap().as_str(), "\r\n");
            println!("temp result: {:?}", pass_val);
            let deserialized_val = deserialize(&pass_val);
            array.push(deserialized_val.1);
        }
    }
    */
    println!("array length: {}, array values: {:?}", length, array);

    (length, StoredType::Array(arr_len, array))
}

fn handle_null(chars: Vec<char>) -> (isize, StoredType) {
    (3, StoredType::Null)
}

fn handle_boolean(chars: Vec<char>) -> (isize, StoredType) {
    let boolean = *chars.get(1).unwrap(); // #<t|f>\r\n
    match boolean {
        't' => (4, StoredType::Boolean(true)),
        'f' => (4, StoredType::Boolean(false)),
        _ => (4, StoredType::Boolean(false)),
    }
}

fn handle_double(chars: Vec<char>) -> (isize, StoredType) {
    // TODO: finish function
    (0, StoredType::Double(0, 0))
}

fn handle_big_number(chars: Vec<char>) -> (isize, StoredType) {
    // TODO: finish function
    (0, StoredType::BigNumber(String::from("0")))
}

fn handle_bulk_error(chars: Vec<char>) -> (isize, StoredType) {
    // TODO: finish function
    (0, StoredType::BulkError(0, String::from("")))
}

fn handle_verbatim_string(chars: Vec<char>) -> (isize, StoredType) {
    // TODO: finish function
    (
        0,
        StoredType::VerbatimString(0, String::from("txt"), String::from("")),
    )
}

fn handle_map(chars: Vec<char>) -> (isize, StoredType) {
    // TODO: finish function
    (0, StoredType::Map(0, HashMap::new()))
}

fn handle_set(chars: Vec<char>) -> (isize, StoredType) {
    // TODO: finish function
    (0, StoredType::Set(0, vec![]))
}

fn handle_push(chars: Vec<char>) -> (isize, StoredType) {
    // TODO: finish function
    (0, StoredType::Push(0, vec![]))
}
