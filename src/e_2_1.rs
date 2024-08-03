#[cfg(test)]
mod test {
    #[test]
    fn define_eight() {
        let a = 0o10;
        assert_eq!(a, 8);
    }
    #[test]
    fn define_sixteen() {
        let a = 0x10;
        assert_eq!(a, 16);
    }
}
