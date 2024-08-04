pub struct Number(i32);

impl std::convert::Into<String> for Number {
    fn into(self) -> String {
        String::from(format!("{}", self.0))
    }
}

pub struct Int(i32);

impl std::convert::From<i8> for Int {
    fn from(value: i8) -> Self {
        Self(value as i32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn into_string() {
        let n = Number(10);
        let s: String = n.into();
        assert_eq!(s, "10");
    }

    #[test]
    fn i8_to_i32() {
        let i = Int::from(8i8);
        assert_eq!(i.0, 8);
    }

    #[test]
    fn auto_into() {
        let i = 10i8;
        let n: Int = i.into();
        assert_eq!(n.0, 10);
    }
}
