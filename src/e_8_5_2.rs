#[cfg(test)]
mod test {
    #[test]
    fn tet_if() {
        let a = (10, 20);
        let b = match a {
            // 匹配x>y的情况
            (x, y) if x > y => true,
            _ => false,
        };
        assert!(!b);
    }
    #[test]
    fn test_match_bind() {
        let a = 12;
        let n = match a {
            // 匹配并绑定到变量n
            n @ 1..=17 => n,
            n => n,
        };
        assert_eq!(n, 12);
    }
}
