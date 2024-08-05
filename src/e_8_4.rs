#[cfg(test)]
mod test {
    #[test]
    fn test_into_iter() {
        let names = vec!["Bob", "Frank", "Ferris"];
        let mut s = String::new();
        // 使用into_iter会消耗掉names
        for name in names.into_iter() {
            s.push_str(name);
        }
        assert_eq!(s, "BobFrankFerris");
    }
}
