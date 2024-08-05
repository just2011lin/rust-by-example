#[cfg(test)]
mod test {
    #[test]
    fn use_returned_fn() {
        fn return_fn() -> impl Fn() -> [i32; 1] {
            let a = [10];
            move || a
        }

        let return_a = return_fn();
        let a = return_a();
        assert_eq!(a[0], 10);
        let b = return_a();
        assert_eq!(b[0], 10);
    }
    #[test]
    fn use_returned_fn_2() {
        fn return_fn() -> impl Fn() -> [i32; 1] {
            let a = [10];
            move || a
        }

        let return_a = return_fn();
        let a = return_a();
        let b = return_a();
        assert_eq!(a[0], 10);
        assert_eq!(b[0], 10);
    }
    #[test]
    fn use_returned_fn_3() {
        // 如果返回值未实现CopyTrait，则也许只能使用FnOnce类型
        fn return_fn() -> impl FnOnce() -> String {
            let a = String::from("hello");
            move || a
        }

        let return_a = return_fn();
        let a = return_a();
        // 不可以再次调用
        // let b = return_a();
        assert_eq!(a, "hello");
    }
    #[test]
    #[should_panic]
    fn foo() {
        // 发散函数，使用!标记
        fn foo() -> ! {
            panic!("报错了啦");
        }
        foo();
    }
}
