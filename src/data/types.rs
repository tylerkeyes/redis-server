// generic trait to share functionality between all stored data types

use std::collections::HashMap;

// TODO: use to select what type of data is being stored
#[derive(Debug)]
pub enum StoredType {
    SimpleString(String),
    SimpleError(String),
    Integer(isize),
    BulkString(isize, String),         // length of string, value
    Array(isize, Vec<StoredType>), // # of elements, list of objects
    Null,
    Boolean(bool),
    Double(i64, u64), // whole number, decimal
    BigNumber(String),
    BulkError(isize, String),              // length of error, value
    VerbatimString(isize, String, String), // size, encoding, value
    Map(isize, HashMap<StoredType, StoredType>), // # of elements, map of object mappings
    Set(isize, Vec<StoredType>), // # of elements, set of objects TODO: need to make sure this
                                     // functions as a set
    Push(isize, Vec<StoredType>),
}

