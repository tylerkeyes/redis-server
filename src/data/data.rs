use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::data::types::StoredType;

// TODO: need to pass 'redis' data, then serialize to string format
#[allow(dead_code)]
pub fn serialize(data: &StoredType) -> String {
    println!("serialize: {:?}", data);
    match data {
        StoredType::SimpleString(..) => serialize_simple_string(data),
        StoredType::SimpleError(..) => serialize_simple_error(data),
        StoredType::Integer(..) => serialize_integer(data),
        StoredType::BulkString(..) => serialize_bulk_string(data),
        StoredType::Array(..) => serialize_array(data),
        StoredType::Null => serialize_null(data),
        StoredType::Boolean(..) => serialize_boolean(data),
        StoredType::Double(..) => serialize_double(data),
        StoredType::BigNumber(..) => serialize_big_number(data),
        StoredType::BulkError(..) => serialize_bulk_error(data),
        StoredType::VerbatimString(..) => serialize_verbatim_string(data),
        StoredType::Map(..) => serialize_map(data),
        StoredType::Set(..) => serialize_set(data),
        StoredType::Push(..) => serialize_push(data),
    }
}

fn serialize_simple_string(data: &StoredType) -> String {
    let mut serialized = "+".to_string();

    let val = match data {
        StoredType::SimpleString(x) => x.to_string(),
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized.push_str("\r\n");

    serialized
}

fn serialize_simple_error(data: &StoredType) -> String {
    let mut serialized = "-".to_string();
    let val = match data {
        StoredType::SimpleError(x) => x.to_string(),
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized.push_str("\r\n");
    serialized
}

fn serialize_integer(data: &StoredType) -> String {
    let mut serialized = ":".to_string();
    let val = match data {
        StoredType::Integer(x) => x.to_string(),
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized.push_str("\r\n");
    serialized
}

fn serialize_bulk_string(data: &StoredType) -> String {
    let mut serialized = "$".to_string();
    let val = match data {
        StoredType::BulkString(size, str) => {
            let mut val_str = (*size).to_string();
            val_str.push_str("\r\n");
            val_str.push_str(&str);
            val_str.push_str("\r\n");
            val_str
        }
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized
}

fn serialize_array(data: &StoredType) -> String {
    let mut serialized = "*".to_string();
    let val = match data {
        StoredType::Array(size, arr) => {
            let mut val_str = (*size).to_string();
            val_str.push_str("\r\n");
            for item in arr {
                let stored_str = serialize(item);
                val_str.push_str(&stored_str); // assume each serialized StoredType should close
                                               // itsself, add ending "\r\n"
            }
            val_str
        }
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized
}

fn serialize_null(data: &StoredType) -> String {
    let mut serialized = "_".to_string();
    let val = match data {
        StoredType::Null => "\r\n".to_string(),
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized
}

fn serialize_boolean(data: &StoredType) -> String {
    let mut serialized = "#".to_string();
    let val = match data {
        StoredType::Boolean(bool) => {
            if *bool {
                "t".to_string()
            } else {
                "f".to_string()
            }
        }
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized.push_str("\r\n");
    serialized
}

fn serialize_double(data: &StoredType) -> String {
    let mut serialized = ",".to_string();
    let val = match data {
        StoredType::Double(num, frac, exp) => {
            let mut double_val = num.to_string();
            if *frac != 0 {
                double_val.push_str(".");
                double_val.push_str(&frac.to_string());
            }
            if *exp != 0 {
                double_val.push_str("e");
                double_val.push_str(&exp.to_string());
            }
            double_val
        }
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized.push_str("\r\n");
    serialized
}

fn serialize_big_number(data: &StoredType) -> String {
    let mut serialized = "(".to_string();
    let val = match data {
        StoredType::BigNumber(num) => num.to_string(),
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized.push_str("\r\n");
    serialized
}

fn serialize_bulk_error(data: &StoredType) -> String {
    let mut serialized = "!".to_string();
    let val = match data {
        StoredType::BulkError(size, str) => {
            let mut err_str = size.to_string();
            err_str.push_str("\r\n");
            err_str.push_str(str);
            err_str.push_str("\r\n");
            err_str
        }
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized
}

fn serialize_verbatim_string(data: &StoredType) -> String {
    let mut serialized = "=".to_string();
    let val = match data {
        StoredType::VerbatimString(size, code, str) => {
            let mut verbatim = size.to_string();
            verbatim.push_str("\r\n");
            verbatim.push_str(code);
            verbatim.push_str(":");
            verbatim.push_str(str);
            verbatim.push_str("\r\n");
            verbatim
        }
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized
}

fn serialize_map(data: &StoredType) -> String {
    let mut serialized = "%".to_string();
    let val = match data {
        StoredType::Map(size, map) => {
            let mut map_str = size.to_string();
            map_str.push_str("\r\n");
            for (key, value) in map {
                let key_str = serialize(key);
                map_str.push_str(&key_str);
                let val_str = serialize(value);
                map_str.push_str(&val_str);
            }
            map_str
        }
        _ => "".to_string(),
    };
    serialized.push_str(&val);
    serialized
}

fn serialize_set(data: &StoredType) -> String {
    "".to_string()
}

fn serialize_push(data: &StoredType) -> String {
    "".to_string()
}

#[allow(dead_code)]
pub fn deserialize(data: &str) -> (usize, StoredType) {
    let characters: Vec<char> = data.chars().collect();
    let char_type = characters.get(0).unwrap();

    //println!("[DEBUG] deserialize: {:?}", data);

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

fn handle_simple_string(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle simple string: {:?}", chars);
    let mut result = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        result.push(*chars.get(i).unwrap());
        i += 1;
    }
    (i + 2, StoredType::SimpleString(result))
}

fn handle_simple_errors(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle simple error: {:?}", chars);
    let mut result = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        result.push(*chars.get(i).unwrap());
        i += 1;
    }
    (i + 2, StoredType::SimpleError(result))
}

fn handle_integer(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle integer: {:?}", chars);
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
    (i + 2, StoredType::Integer(result))
}

fn handle_bulk_string(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle bulk string: {:?}", chars);
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
        i + 2,
        StoredType::BulkString(length.parse::<isize>().unwrap(), data),
    )
}

fn handle_array(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle array: {:?}", chars);
    let mut array: Vec<StoredType> = Vec::new();
    let split_chars: String = chars.into_iter().collect();
    if split_chars == "*-1\r\n" {
        return (5, StoredType::Array(-1, array));
    }

    // start calculating character length of array
    let mut char_len = 0;

    // capture length
    let re_length = Regex::new(r"^*(\d+)\r\n").unwrap();
    let Some(capture) = re_length.captures(&split_chars) else {
        println!("error capturing length");
        return (0, StoredType::Array(0, vec![]));
    };
    let arr_len: usize = (&capture[1]).parse::<usize>().unwrap();
    let tmp_len_str = arr_len.to_string();
    char_len += tmp_len_str.len() + 3;

    // mark start of substring
    let mut start_idx = split_chars.find("\r\n").unwrap() + 2;

    for _ in 0..arr_len {
        let remaining = &split_chars[start_idx..];
        let curr_val = deserialize(remaining);

        // update 'remaining' var
        start_idx += curr_val.0;
        array.push(curr_val.1);
        char_len += curr_val.0;
    }

    (char_len, StoredType::Array(arr_len as isize, array))
}

fn handle_null(_chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle null: {:?}", chars);
    (3, StoredType::Null)
}

fn handle_boolean(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle boolean: {:?}", chars);
    let boolean = *chars.get(1).unwrap(); // #<t|f>\r\n
    match boolean {
        't' => (4, StoredType::Boolean(true)),
        'f' => (4, StoredType::Boolean(false)),
        _ => (4, StoredType::Boolean(false)),
    }
}

fn handle_double(chars: Vec<char>) -> (usize, StoredType) {
    //println!("\n\n[DEBUG] handle double: {:?}", chars);
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

    let mut whole_fraction = Vec::new();
    for part in number.split('.') {
        whole_fraction.push(part);
    }

    let mut whole_num = 0;
    if !whole_fraction.get(0).unwrap().contains('e')
        && !whole_fraction.get(0).unwrap().contains('E')
    {
        whole_num = whole_fraction.get(0).unwrap().parse::<isize>().unwrap();
    }

    let mut frac_num = 0;
    let mut exp_num = 0;
    if whole_fraction.len() > 1 {
        let mut fraction_split = Vec::new();
        let test = whole_fraction.get(1).unwrap().to_lowercase();
        for part in test.split('e') {
            fraction_split.push(part);
        }
        frac_num = fraction_split.get(0).unwrap().parse::<isize>().unwrap();
        if fraction_split.len() > 1 {
            exp_num = fraction_split.get(1).unwrap().parse::<isize>().unwrap();
        }
    } else {
        let mut exp_split = Vec::new();
        let test = whole_fraction.get(0).unwrap().to_lowercase();
        for part in test.split('e') {
            exp_split.push(part);
        }
        whole_num = exp_split.get(0).unwrap().parse::<isize>().unwrap();
        if exp_split.len() > 1 {
            let exp_num_str = *exp_split.get(1).unwrap();
            let exp_chars: Vec<char> = exp_num_str.chars().collect();
            if exp_chars[0] == '+' {
                let mut collected_exp_str = Vec::new();
                for i in 1..exp_chars.len() {
                    collected_exp_str.push(*exp_chars.get(i).unwrap());
                }
                exp_num = collected_exp_str
                    .iter()
                    .collect::<String>()
                    .parse::<isize>()
                    .unwrap();
            } else {
                exp_num = exp_split.get(1).unwrap().parse::<isize>().unwrap();
            }
        }
    }

    (0, StoredType::Double(whole_num * sign, frac_num, exp_num))
}

fn handle_big_number(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle big number: {:?}", chars);
    let mut number = String::from("");
    let mut i = 1;
    let mut skipped = 3; // count skipped chars - leader, ending 2

    // skip '+' character if included
    if *chars.get(1).unwrap() == '+' {
        i += 1;
        skipped += 1;
    }

    while *chars.get(i).unwrap() != '\r' {
        number.push(*chars.get(i).unwrap());
        i += 1;
    }
    let len = number.len() + skipped;

    (len, StoredType::BigNumber(number))
}

fn handle_bulk_error(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle bulk error: {:?}", chars);
    let mut length = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        length.push(*chars.get(i).unwrap());
        i += 1;
    }
    i += 2; // move counter past '\r\n'

    let mut data = String::from("");
    while *chars.get(i).unwrap() != '\r' {
        data.push(*chars.get(i).unwrap());
        i += 1;
    }
    (
        i + 2,
        StoredType::BulkError(length.parse::<isize>().unwrap(), data),
    )
}

fn handle_verbatim_string(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle verbatim string: {:?}", chars);
    let mut length = String::from("");
    let mut i = 1;
    while *chars.get(i).unwrap() != '\r' {
        length.push(*chars.get(i).unwrap());
        i += 1;
    }
    i += 2; // move counter past '\r\n'

    let mut encoding = String::from("");
    while *chars.get(i).unwrap() != ':' {
        encoding.push(*chars.get(i).unwrap());
        i += 1;
    }
    i += 1; // move counter past ':'

    let mut data = String::from("");
    while *chars.get(i).unwrap() != '\r' {
        data.push(*chars.get(i).unwrap());
        i += 1;
    }

    (
        i + 2,
        StoredType::VerbatimString(length.parse::<isize>().unwrap(), encoding, data),
    )
}

fn handle_map(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle map: {:?}", chars);
    let mut value_map = HashMap::new();
    let split_chars: String = (&chars).into_iter().collect();

    // calculate element length of map
    let mut char_len = 1;
    let mut len_str = String::from("");

    while *chars.get(char_len).unwrap() != '\r' {
        len_str.push(*chars.get(char_len).unwrap());
        char_len += 1;
    }
    char_len += 2;
    let map_len = len_str.parse::<isize>().unwrap();

    for _ in 0..map_len {
        let remaining = &split_chars[char_len..];
        let key_result = deserialize(&remaining);
        char_len += key_result.0;

        let remaining = &split_chars[char_len..];
        let value_result = deserialize(&remaining);
        char_len += value_result.0;

        value_map.insert(key_result.1, value_result.1);
    }

    (char_len, StoredType::Map(map_len, value_map))
}

fn handle_set(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle set: {:?}", chars);
    let mut set: HashSet<StoredType> = HashSet::new();
    let split_chars: String = (&chars).into_iter().collect();

    let mut char_len = 1;
    let mut len_str = String::from("");

    while *chars.get(char_len).unwrap() != '\r' {
        len_str.push(*chars.get(char_len).unwrap());
        char_len += 1;
    }
    char_len += 2; // move past separator character
    let set_len = len_str.parse::<isize>().unwrap();

    for _ in 0..set_len {
        let remaining = &split_chars[char_len..];
        let value_res = deserialize(&remaining);
        char_len += value_res.0;
        set.insert(value_res.1);
    }

    (char_len, StoredType::Set(set_len, set))
}

fn handle_push(chars: Vec<char>) -> (usize, StoredType) {
    //println!("[DEBUG] handle push: {:?}", chars);
    let mut push_vec: Vec<StoredType> = Vec::new();
    let mut char_len = 1;
    let mut len_str = String::from("");
    let char_str: String = (&chars).into_iter().collect();

    while *chars.get(char_len).unwrap() != '\r' {
        len_str.push(*chars.get(char_len).unwrap());
        char_len += 1;
    }
    char_len += 2; // move past separator characters
    let push_len = len_str.parse::<isize>().unwrap();

    for _ in 0..push_len {
        let remaining = &char_str[char_len..];
        let result = deserialize(remaining);
        char_len += result.0;
        push_vec.push(result.1);
    }

    (char_len, StoredType::Push(push_len, push_vec))
}
