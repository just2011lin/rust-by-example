// 地址：https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_display/testcase_list.html

pub struct List(Vec<i32>);

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let v = &self.0;
        for (i, v) in v.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{i}: {v}")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn display_list() {
        let list = List(vec![1, 2, 3]);
        let str = list.to_string();
        assert_eq!(str, "[0: 1, 1: 2, 2: 3]".to_string())
    }
}
