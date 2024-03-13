#[cfg(test)]
mod serialize_tests {

    //use log::warn;

    use crate::data::{data::deserialize, types::StoredType};

    // integer tests
    #[test]
    fn deserialize_integer() {
        let result = deserialize(":-10\r\n");
        match result.1 {
            StoredType::Integer(x) => assert_eq!(-10, x),
            _ => assert!(false),
        }
    }

    // simple string tests
    #[test]
    fn deserialize_simple_string() {
        let result = deserialize("+hello\r\n");
        match result.1 {
            StoredType::SimpleString(s) => assert_eq!("hello", s),
            _ => assert!(false),
        }
    }

    // simple error tests
    #[test]
    fn deserialize_simple_error() {
        let result = deserialize("-ERR bad\r\n");
        match result.1 {
            StoredType::SimpleError(s) => assert_eq!("ERR bad", s),
            _ => assert!(false),
        }
    }

    // bulk string tests
    #[test]
    fn deserialize_bulk_string() {
        let result = deserialize("$27\r\nHello this is a bulk string\r\n");
        match result.1 {
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
        match result.1 {
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
        match result.1 {
            StoredType::Null => assert!(true),
            _ => assert!(false),
        }
    }

    // null tests
    #[test]
    fn deserialize_null() {
        let result = deserialize("_\r\n");
        match result.1 {
            StoredType::Null => assert!(true),
            _ => assert!(false),
        }
    }

    // boolean tests
    #[test]
    fn deserialize_bool_true() {
        let result = deserialize("#t\r\n");
        match result.1 {
            StoredType::Boolean(x) => assert_eq!(true, x),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_bool_false() {
        let result = deserialize("#f\r\n");
        match result.1 {
            StoredType::Boolean(x) => assert_eq!(false, x),
            _ => assert!(false),
        }
    }

    // array tests
    #[test]
    fn deserialize_array_empty() {
        let result = deserialize("*0\r\n");
        match result.1 {
            StoredType::Array(x, _y) => assert_eq!(0, x),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_array_two_strings() {
        let result = deserialize("*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
        match result.1 {
            StoredType::Array(x, _y) => assert_eq!(2, x),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_null_array() {
        let result = deserialize("*-1\r\n");
        match result.1 {
            StoredType::Array(x, _y) => assert_eq!(-1, x),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_complex_array() {
        let result = deserialize("*3\r\n:4\r\n*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n+OK\r\n");
        match result.1 {
            StoredType::Array(x, _y) => assert_eq!(3, x),
            _ => assert!(false),
        }
    }

    // double tests
    #[test]
    fn deserialize_double_default() {
        let result = deserialize(",1.23\r\n");
        match result.1 {
            StoredType::Double(x, y, z) => {
                if x != 1 || y != 23 || z != 0 {
                    panic!("double was not empty")
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_double_exp() {
        let result = deserialize(",1E+2\r\n");
        match result.1 {
            StoredType::Double(x, y, z) => {
                if x != 1 || y != 0 || z != 2 {
                    panic!("double was not empty")
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_double_exp_neg() {
        let result = deserialize(",1E-2\r\n");
        match result.1 {
            StoredType::Double(x, y, z) => {
                if x != 1 || y != 0 || z != -2 {
                    panic!("double was not empty")
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_double_all() {
        let result = deserialize(",1.23E-2\r\n");
        match result.1 {
            StoredType::Double(x, y, z) => {
                if x != 1 || y != 23 || z != -2 {
                    panic!("double was not empty")
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_double_basic() {
        let result = deserialize(",1\r\n");
        match result.1 {
            StoredType::Double(x, y, z) => {
                if x != 1 || y != 0 || z != 0 {
                    panic!("double was not empty")
                }
            }
            _ => assert!(false),
        }
    }

    // big number tests
    #[test]
    fn deserialize_big_number() {
        let result = deserialize("(3492890328409238509324850943850943825024385\r\n");
        match result.1 {
            StoredType::BigNumber(x) => {
                assert_eq!("3492890328409238509324850943850943825024385", x)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_big_number_neg() {
        let result = deserialize("(-3492890328409238509324850943850943825024385\r\n");
        match result.1 {
            StoredType::BigNumber(x) => {
                assert_eq!("-3492890328409238509324850943850943825024385", x)
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn deserialize_big_number_pos() {
        let result = deserialize("(+3492890328409238509324850943850943825024385\r\n");
        match result.1 {
            StoredType::BigNumber(x) => {
                assert_eq!("3492890328409238509324850943850943825024385", x)
            }
            _ => assert!(false),
        }
    }

    // bulk error tests
    #[test]
    fn deserialize_bulk_error() {
        let result = deserialize("!21\r\nSYNTAX invalid syntax\r\n");
        match result.1 {
            StoredType::BulkError(x, y) => {
                if x != 21 || "SYNTAX invalid syntax" != y {
                    panic!("bulk error was not equal")
                }
            }
            _ => assert!(false),
        }
    }

    // verbatim string tests
    #[test]
    fn deserialize_verbatim_string() {
        let result = deserialize("=15\r\ntxt:Some string\r\n");
        match result.1 {
            StoredType::VerbatimString(len, encoding, data) => {
                if 15 != len || "txt" != encoding || "Some string" != data {
                    panic!("verbatim string was not equal")
                }
            }
            _ => assert!(false),
        }
    }

    // map tests
    #[test]
    fn deserialize_map_basic() {
        let result = deserialize("%2\r\n+first\r\n:1\r\n+second\r\n:2\r\n");
        match result.1 {
            StoredType::Map(size, map) => assert_eq!(2, size),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_map_empty() {
        let result = deserialize("%0\r\n");
        match result.1 {
            StoredType::Map(size, map) => assert_eq!(0, size),
            _ => assert!(false),
        }
    }

    // set tests
    #[test]
    fn deserialize_set_basic() {
        let result = deserialize("~2\r\n+first\r\n:1\r\n");
        match result.1 {
            StoredType::Set(size, set) => assert_eq!(2, size),
            _ => assert!(false),
        }
    }

    // push tests
    #[test]
    fn deserialize_push_basic() {
        let result = deserialize(">2\r\n+first\r\n:1\r\n");
        match result.1 {
            StoredType::Push(size, set) => assert_eq!(2, size),
            _ => assert!(false),
        }
    }
}
