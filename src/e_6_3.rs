use std::str::FromStr;

pub struct Point(i32, i32);

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut is = s.split(",");
        let (Some(a), Some(b)) = (is.next(), is.next()) else {
            return Err(String::from("解析错误"));
        };
        let a: i32 = a.parse().map_err(|_| {
            return String::from("解析错误");
        })?;
        let b: i32 = b.parse().map_err(|_| {
            return String::from("解析错误");
        })?;
        Ok(Self(a, b))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_to_point() {
        let s = "10,2";
        let p: Point = s.parse().unwrap();
        assert_eq!(p.0, 10);
        assert_eq!(p.1, 2);
    }

    #[test]
    fn parse_err() {
        let s = "100";
        let p = s.parse::<Point>().err();
        assert_eq!(p, Some("解析错误".to_string()));
    }
}
