pub struct Matrix(f32, f32, f32, f32);

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Matrix(a, b, c, d) = self;
        write!(f, "( {} {} )\r\n( {} {} )", a, b, c, d)
    }
}

impl Matrix {
    pub fn transpose(self) -> Matrix {
        let Matrix(a, b, c, d) = self;
        Matrix(a, c, b, d)
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn matrix_display() {
        let m = Matrix(1.1, 2.2, 3.3, 4.4);
        let str = m.to_string();
        assert_eq!(str, "( 1.1 2.2 )\r\n( 3.3 4.4 )");
    }

    #[test]
    fn matrix_transpose() {
        let m = Matrix(1.1, 2.2, 3.3, 4.4);
        let tm = m.transpose();
        assert_eq!("( 1.1 3.3 )\r\n( 2.2 4.4 )", tm.to_string())
    }
}
