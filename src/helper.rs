#[derive(Debug, Clone, Copy)]
pub enum Direct2D {
    Up,
    Down,
    Left,
    Right,
}
use std::fmt::{self, Display};

use crate::helper::Direct2D::*;

impl Direct2D {
    fn to_vector(&self) -> (i8, i8) {
        match self {
            Up => (0, 1),
            Down => (0, -1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
    fn from_vector(x: i8, y: i8) -> Self {
        match (x, y) {
            (0, 1) => Up,
            (0, -1) => Down,
            (-1, 0) => Left,
            _ => Right,
        }
    }
    /// to 90deg* 0 , 1 , 2, 4, conterclockwise
    fn to_number(&self) -> u8 {
        match self {
            Up => 1,
            Down => 3,
            Left => 2,
            Right => 0,
        }
    }
    fn from_number(n: u8) -> Self {
        match n % 4 {
            1 => Up,
            3 => Down,
            2 => Left,
            _ => Right,
        }
    }
    fn opposite(&self) -> Self {
        let v = self.to_vector();
        Self::from_vector(-v.0, -v.1)
    }
    ///rotate , conterclockwise
    fn rotation(&self, rot: Self) -> Self {
        Self::from_number(self.to_number() + rot.to_number())
    }
    pub fn from_str(str: &str) -> Self {
        match str {
            "^" => Up,
            "v" => Down,
            ">" => Right,
            _ => Left,
        }
    }
}
impl Default for Direct2D {
    fn default() -> Self {
        Right
    }
}
impl Display for Direct2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Up => write!(f, "^"),
            Down => write!(f, "v"),
            Left => write!(f, "<"),
            Right => write!(f, ">"),
        }
    }
}
