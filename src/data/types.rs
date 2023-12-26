// generic trait to share functionality between all stored data types

use std::collections::HashMap;

// Option 1
pub trait StoredTypeTrait {
    fn add(&self);
}

pub struct StoredInteger {
    pub value: i64,
}

impl StoredTypeTrait for StoredInteger {
    fn add(&self) {}
}

pub struct StoredSimpleString {
    pub value: String,
}

impl StoredTypeTrait for StoredSimpleString {
    fn add(&self) {}
}

// Option 2
pub struct Shared<T, U> {
    pub value: T,
    pub next: Option<U>,
}

// TODO: use to select what type of data is being stored
enum StoredTypeKind {
    SimpleString(String),
    SimpleError(String),
    Integer(isize),
    BulkString(isize, String),         // length of string, value
    Array(isize, Vec<StoredTypeKind>), // # of elements, list of objects
    Null,
    Boolean(bool),
    Double(i64, u64), // whole number, decimal
    BigNumber(String),
    BulkError(isize, String),              // length of error, value
    VerbatimString(isize, String, String), // size, encoding, value
    Map(isize, HashMap<StoredTypeKind, StoredTypeKind>), // # of elements, map of object mappings
    Set(isize, Vec<StoredTypeKind>), // # of elements, set of objects TODO: need to make sure this
                                     // functions as a set
    Push(isize, Vec<StoredTypeKind>),
}

// Option 3
// Idea is to use this type as the generic type for all stored values.
// This might end up not working.
pub struct StoredType {
    pub simple_string: Option<String>, // simple string val
    pub next: Option<Box<StoredType>>, // value for map entry
    pub num_of_elements: Option<u64>,
    pub integer: Option<i64>,
    pub bulk_str_len: Option<u64>,
    pub is_null: Option<bool>,
    pub is_bool: Option<bool>,
    pub decimal_digits: Option<u64>, // digits after decimal point, 0 for none
    pub big_int: Option<String>,
    pub verbatim_encoding: Option<String>,
}

impl StoredType {
    pub fn new() -> StoredType {
        StoredType {
            simple_string: None, // simple string val
            next: None,          // value for map entry
            num_of_elements: None,
            integer: None,
            bulk_str_len: None,
            is_null: None,
            is_bool: None,
            decimal_digits: None, // digits after decimal point, 0 for none
            big_int: None,
            verbatim_encoding: None,
        }
    }

    // TODO: Create functions to set each property
}
