// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

#[cfg(test)]
mod test {
    #[test]
    fn test_double_into_u8() {
        // 将浮点数转为u8数字
        let u = 16.18 as u8;
        assert_eq!(u, 16);
    }
    #[test]
    fn test_u8_into_char() {
        let d = 98u8;
        // 只有u8才能转为char类型
        let c = d as char;
        assert_eq!(c, 'b');
    }
    #[test]
    fn test_mod_as() {
        // 对于正整数来说
        // 强转更小单位的u类型
        // 相当于取模操作
        let u: i32 = 1000 % (std::u8::MAX as i32 + 1);
        assert_eq!(u as u8, 1000 as u8);
    }
    #[test]
    fn test_u2i() {
        let a = 128i8;
        let b = a as u8;
        assert_eq!(b, 128);
    }
}
