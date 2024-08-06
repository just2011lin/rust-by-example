#![allow(dead_code, unused_variables)]

#[test]
fn partial_move() {
    let a = (1, 2);
    let (b, ref c) = a;
    // 因为a.1的所有权未被移走，所以仍可以使用
    assert_eq!(a.1, 2);
}

#[test]
fn move_child() {
    let a = &(1, 2);
    // 使用模式匹配将获取到引用
    let (b, ..) = a;
    assert_eq!(*b, 1);
    // 这样写会获取引用
    let b = &a.0;
    // 这样写会获取子项的Copy值
    let b = (&a).0;
}

#[test]
fn move_child_2() {
    struct Student {
        name: String,
    }
    let a = Student {
        name: String::from("Lily"),
    };
    // 模式匹配获取到a.name的引用
    let Student { name } = &a;
    assert_eq!(name, "Lily");
    // 不能从引用中获取子项的所有权
    // 只有当子项实现了Copy时才可以获取Copy的子项值
    let b = &a;
    let x = &b.name;
    assert_eq!(x, "Lily");
}

// 15.3.1 可变性
#[test]
fn copy_when_assignment() {
    // 赋值时会优先拷贝值
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let a = Point { x: 1, y: 2 };
    // 未发生所有权转移，而是进行了Copy
    let b = a;

    assert_eq!(a.x, 1);
}

#[test]
fn use_ref() {
    struct Student {
        name: String,
    }
    let a = Student {
        name: String::from("Lily"),
    };
    let Student { ref name } = a;
    // 不能转移子项所有权
    // let b = *name;
    // 只能通过所有权本体转移子项所有权
    let name = a.name;
}
