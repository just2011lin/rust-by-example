#[cfg(test)]
mod test {
    #[test]
    fn test_06x() {
        let red = 0xff0000;
        // 06x表示至少六位的十六进制数字符串
        let str = format!("{red:06x}");
        assert_eq!(str, "ff0000");
    }
}
