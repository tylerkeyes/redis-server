// generic enum to share functionality between all stored data types

use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

#[derive(Debug, Eq)]
pub enum StoredType {
    SimpleString(String),
    SimpleError(String),
    Integer(isize),
    BulkString(isize, String),     // length of string, value
    Array(isize, Vec<StoredType>), // # of elements, list of objects
    Null,
    Boolean(bool),
    Double(isize, isize, isize), // whole number, decimal, exponent
    BigNumber(String),
    BulkError(isize, String),                    // length of error, value
    VerbatimString(isize, String, String),       // size, encoding, value
    Map(isize, HashMap<StoredType, StoredType>), // # of elements, map of object mappings
    Set(isize, HashSet<StoredType>), // # of elements, set of objects TODO: need to make sure this functions as a set
    Push(isize, Vec<StoredType>),
}

// derive the traits Eq, PartialEq, & Hash to allow the HashMap<StoredType, StoredType>
// to exist and not create an error.

impl PartialEq for StoredType {
    fn eq(&self, other: &StoredType) -> bool {
        match &self {
            StoredType::SimpleString(x) => match other {
                StoredType::SimpleString(y) => x == y,
                _ => false,
            },
            StoredType::SimpleError(x) => match other {
                StoredType::SimpleError(y) => x == y,
                _ => false,
            },
            StoredType::Integer(x) => match other {
                StoredType::Integer(y) => x == y,
                _ => false,
            },
            StoredType::BulkString(x, y) => match other {
                StoredType::BulkString(x1, y1) => x == x1 && y == y1,
                _ => false,
            },
            StoredType::Array(x, y) => match other {
                StoredType::Array(x1, y1) => {
                    x == x1 && {
                        for i in 0..y.len() {
                            if y.get(i) != y1.get(i) {
                                return false;
                            }
                        }
                        true
                    }
                }
                _ => false,
            },
            StoredType::Null => match other {
                StoredType::Null => true,
                _ => false,
            },
            StoredType::Boolean(x) => match other {
                StoredType::Boolean(y) => x == y,
                _ => false,
            },
            StoredType::Double(x, y, z) => match other {
                StoredType::Double(x1, y1, z1) => x == x1 && y == y1 && z == z1,
                _ => false,
            },
            StoredType::BigNumber(x) => match other {
                StoredType::BigNumber(y) => x == y,
                _ => false,
            },
            StoredType::BulkError(x, y) => match other {
                StoredType::BulkError(x1, y1) => x == x1 && y == y1,
                _ => false,
            },
            StoredType::VerbatimString(x, y, z) => match other {
                StoredType::VerbatimString(x1, y1, z1) => x == x1 && y == y1 && z == z1,
                _ => false,
            },
            StoredType::Map(x, y) => match other {
                StoredType::Map(x1, y1) => {
                    x == x1 && {
                        for i in y.keys() {
                            // just checks keys for now
                            if !y1.contains_key(i) {
                                return false;
                            }
                        }
                        true
                    }
                }
                _ => false,
            },
            StoredType::Set(x, y) => match other {
                StoredType::Set(x1, y1) => {
                    x == x1 && {
                        for item in y {
                            if !y1.contains(item) {
                                return false;
                            }
                        }
                        true
                    }
                }
                _ => false,
            },
            StoredType::Push(x, y) => match other {
                StoredType::Push(x1, y1) => {
                    x == x1 && {
                        for i in 0..y.len() {
                            if y.get(i) != y1.get(i) {
                                return false;
                            }
                        }
                        true
                    }
                }
                _ => false,
            },
        }
    }
}

impl Hash for StoredType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            StoredType::SimpleString(x) => x.hash(state),
            StoredType::SimpleError(x) => x.hash(state),
            StoredType::Integer(x) => x.hash(state),
            StoredType::BulkString(_x, y) => y.hash(state),
            StoredType::Array(x, _y) => x.hash(state),
            StoredType::Null => self.hash(state),
            StoredType::Boolean(x) => x.hash(state),
            StoredType::Double(x, y, z) => (x + y + z).hash(state),
            StoredType::BigNumber(x) => x.hash(state),
            StoredType::BulkError(_x, y) => y.hash(state),
            StoredType::VerbatimString(_x, _y, z) => z.hash(state),
            StoredType::Map(x, _y) => x.hash(state),
            StoredType::Set(x, _y) => x.hash(state),
            StoredType::Push(x, _z) => x.hash(state),
        };
    }
}
