use std::ops::{Add, Sub};

// Vec2

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub col: u16,
    pub row: u16,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col.saturating_sub(rhs.col),
            row: self.row.saturating_sub(rhs.row),
        }
    }
}

impl From<(u16, u16)> for Vec2 {
    fn from(value: (u16, u16)) -> Self {
        Self {
            col: value.0,
            row: value.1,
        }
    }
}

#[macro_export]
macro_rules! vec2 {
    ($arg1:expr, $arg2:expr) => {{
        let col_arg = u16::try_from($arg1);
        let row_arg = u16::try_from($arg2);

        match (col_arg, row_arg) {
            (Ok(f1), Ok(f2)) => Ok(Vec2 { col: f1, row: f2 }),
            _ => Err("cannot convert args to u16"),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let vec_1 = vec2!(12, 7).unwrap();
        let vec_2: Vec2 = (2, 20).into();
        let vec_3 = Vec2 { row: 1, col: 3 };
        let vec_result = vec_1 + vec_2 - vec_3;
        assert_eq!(vec_result.col, 11);
        assert_eq!(vec_result.row, 26);
    }

    #[test]
    fn error_macro() {
        let vec_1_result = vec2!(-3, 4);
        assert!(vec_1_result.is_err());
    }

    #[test]
    fn subtract_underflow() {
        let vec_1 = vec2!(3, 3).unwrap();
        let vec_2 = vec2!(4, 4).unwrap();
        let vec_result = vec_1 - vec_2;
        assert_eq!(vec_result, vec2!(0, 0).unwrap())
    }
}
