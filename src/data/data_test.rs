#[cfg(test)]
mod serialize_tests {
    use crate::data::data::deserialize;

    #[test]
    fn serialize1() {
        assert_eq!("", deserialize(":1\r\n"));
    }
}
