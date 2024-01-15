#[cfg(test)]
mod serialize_tests {
    use crate::data::{data::deserialize, types::StoredType};

    /*
    #[test]
    fn deserialize_integer() {
        assert_eq!("", deserialize(":-10\r\n"));
    }

    #[test]
    fn deserialize_simple_string() {
        let result = deserialize("+hello\r\n");
        match result {
            StoredType::SimpleString(s) => assert_eq!("hello", s),
            _ => assert!(false),
        }
    }

    #[test]
    fn deserialize_simple_error() {
        let result = deserialize("-ERR bad\r\n");
        match result {
            StoredType::SimpleError(s) => assert_eq!("ERR bad", s),
            _ => assert!(false),
        }
    }
    */

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
}
