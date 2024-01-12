#[cfg(test)]
mod serialize_tests {
    use crate::data::{data::deserialize, types::StoredType};

    /*
    #[test]
    fn deserialize_integer() {
        assert_eq!("", deserialize(":-10\r\n"));
    }
    */

    #[test]
    fn deserialize_simple_string() {
        let result = deserialize("+hello\r\n");
        match result {
            StoredType::SimpleString(s) => assert_eq!("hello", s),
            _ => assert!(false),
        }
    }
}
