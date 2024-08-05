#[cfg(test)]
mod test {
    #[test]
    fn use_left_dot() {
        let triple = (1, 2, 3);
        // 使用..忽略剩余部分
        let (a, ..) = triple;
        assert_eq!(a, 1);
    }
    #[test]
    fn use_reference() {
        let a = &12;
        // 使用&b模式匹配&12
        let &b = a;
        assert_eq!(b, 12);
    }
    #[test]
    fn use_de_reference() {
        let a = &12;
        let b = *a;
        assert_eq!(b, 12);
    }
    #[test]
    fn test_let_ref() {
        let ref a = 3;
        assert_eq!(a, &3);
        // assert_ne!(a, 3);
    }
    #[test]
    fn test_match_ref() {
        let a = 3;
        match a {
            ref a => {
                assert_eq!(a, &3);
            }
        }
    }
    #[test]
    fn test_match_ref_mut() {
        let mut a = 3;
        match a {
            // 获取可变引用的模式，不能使用&mut
            ref mut a => {
                *a = 4;
            }
        }
        assert_eq!(a, 4);
    }
}
