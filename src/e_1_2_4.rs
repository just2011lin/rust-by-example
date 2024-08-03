pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:X}{:X}{:X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_color() {
        let c = Color {
            red: 128,
            green: 255,
            blue: 90,
        };
        let str = c.to_string();
        assert_eq!(str, "RGB (128, 255, 90) 0x80FF5A");
    }
}
