#[cfg(test)]
mod test {
    #[test]
    fn char_of_u8() {
        let i = 8u8;
        // std::mem::size_of_val
        // 可以获取变量所占字节数
        assert_eq!(std::mem::size_of_val(&i), 1);
    }
    #[test]
    fn char_of_char() {
        let c = '♥';
        assert_eq!(std::mem::size_of_val(&c), 4);
    }
}
