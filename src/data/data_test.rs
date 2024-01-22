#[cfg(test)]
mod serialize_tests {

    use crate::data::{data::deserialize, types::StoredType};

    /*
    // integer tests
    #[test]
    fn deserialize_integer() {
        let result = deserialize(":-10\r\n");
        match result {
            StoredType::Integer(x) => assert_eq!(-10, x),
            _ => assert!(false),
        }
    }

    // simple string tests
    #[test]
    fn deserialize_simple_string() {
        let result = deserialize("+hello\r\n");
        match result {
            StoredType::SimpleString(s) => assert_eq!("hello", s),
            _ => assert!(false),
        }
    }

    // simple error tests
    #[test]
    fn deserialize_simple_error() {
        let result = deserialize("-ERR bad\r\n");
        match result {
            StoredType::SimpleError(s) => assert_eq!("ERR bad", s),
            _ => assert!(false),
        }
    }

    // bulk string tests
    #[test]
    fn deserialize_bulk_string() {
        let result = deserialize("$27\r\nHello this is a bulk string\r\n");
        match result {
            StoredType::BulkString(x, y) => {
                if x != 27 || "Hello this is a bulk string" != y {
                    panic!("bulk string was not equal")
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_bulk_string_empty() {
        let result = deserialize("$0\r\n\r\n");
        match result {
            StoredType::BulkString(x, y) => {
                if x != 0 || "" != y {
                    panic!("bulk string was not empty")
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_bulk_string_null() {
        let result = deserialize("$-1\r\n");
        match result {
            StoredType::Null => assert!(true),
            _ => assert!(false),
        }
    }

    // null tests
    #[test]
    fn deserialize_null() {
        let result = deserialize("_\r\n");
        match result {
            StoredType::Null => assert!(true),
            _ => assert!(false),
        }
    }

    // boolean tests
    #[test]
    fn deserialize_bool_true() {
        let result = deserialize("#t\r\n");
        match result {
            StoredType::Boolean(x) => assert_eq!(true, x),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_bool_false() {
        let result = deserialize("#f\r\n");
        match result {
            StoredType::Boolean(x) => assert_eq!(false, x),
            _ => assert!(false),
        }
    }
    */

    // array tests
    #[test]
    fn deserialize_array_empty() {
        let result = deserialize("*0\r\n");
        match result.1 {
            StoredType::Array(x, y) => assert_eq!(0, x),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_array_two_strings() {
        let result = deserialize("*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
        match result.1 {
            StoredType::Array(x, y) => assert_eq!(2, x),
            _ => assert!(false),
        }
    }
}
