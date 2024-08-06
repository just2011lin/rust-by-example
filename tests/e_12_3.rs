#[test]
fn main() {
    println!("Hello, World!");
}

#[test]
fn use_cfg_macro() {
    // 会获取一个布尔值
    let b: bool = cfg!(test);
    assert!(b);
}

struct Any;

// 定义带泛型的trait
trait Test<T: PartialEq> {
    fn assert_eq(a: T, b: T) -> bool;
}

// 实现泛型为i32的Test trait
impl Test<i32> for Any {
    fn assert_eq(a: i32, b: i32) -> bool {
        a == b
    }
}

#[test]
fn use_test_trait() {
    let r = Any::assert_eq(1, 1);
    assert!(r);
}

#[test]
fn static_str() {
    let a: &'static str = "red";
    assert_eq!(a, "red");
}

#[test]
fn format_some() {
    let a = Some(1);
    let b = format!("{:?}", a);
    assert_eq!("Some(1)", b);
}

#[test]
fn where_some() {
    fn print_some<T>(v: T) -> String
    // where约束不仅限于对T的直接约束
    where
        Option<T>: std::fmt::Debug,
    {
        format!("{:?}", Some(v))
    }

    let a = print_some(1);
    assert_eq!("Some(1)", a);
}
