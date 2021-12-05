use crate::ErrorWrapper;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};
use std::str::FromStr;

#[derive(Clone, Debug, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl <T: FromStr> FromStr for Vec2<T> {
    type Err = ErrorWrapper;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut parts = str.split(',');
        let x = parts.next()
            .ok_or_else(|| ErrorWrapper::Simple("Missing component".to_string()))?
            .parse()
            .map_err(|err| ErrorWrapper::ParsingError("Invalid component".to_string()))?;
        let y = parts.next()
            .ok_or_else(|| ErrorWrapper::Simple("Missing component".to_string()))?
            .parse()
            .map_err(|err| ErrorWrapper::ParsingError("Invalid component".to_string()))?;
        if parts.next().is_some() {
            Err(ErrorWrapper::ParsingError("Too many parts".to_string()))
        } else {
            Ok(Self { x, y })
        }
    }
}

impl<T: Hash> Hash for Vec2<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: PartialEq> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Eq> Eq for Vec2<T> {}

impl<T: Default> Default for Vec2<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

use std::fmt::Display;
impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Rem<Output = T>> Rem for Vec2<T> {
    type Output = Self;
    fn rem(self, o: Self) -> Self::Output {
        Self {
            x: self.x % o.x,
            y: self.y % o.y,
        }
    }
}

impl<T: Rem<Output = T> + Copy> Rem for &Vec2<T> {
    type Output = Vec2<T>;
    fn rem(self, o: Self) -> Self::Output {
        Self::Output {
            x: self.x % o.x,
            y: self.y % o.y,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, o: Self) -> Self::Output {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, o: Self) -> Self::Output {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, o: Self) -> Self::Output {
        Self {
            x: self.x * o.x,
            y: self.y * o.y,
        }
    }
}

impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Self;
    fn div(self, o: Self) -> Self::Output {
        Self {
            x: self.x / o.x,
            y: self.y / o.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vec2<T> {
    fn add_assign(&mut self, o: Self) {
        *self = Self {
            x: self.x + o.x,
            y: self.y + o.y,
        };
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, o: Self) {
        *self = Self {
            x: self.x - o.x,
            y: self.y - o.y,
        };
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign for Vec2<T> {
    fn mul_assign(&mut self, o: Self) {
        *self = Self {
            x: self.x * o.x,
            y: self.y * o.y,
        };
    }
}

impl<T: Div<Output = T> + Copy> DivAssign for Vec2<T> {
    fn div_assign(&mut self, o: Self) {
        *self = Self {
            x: self.x / o.x,
            y: self.y / o.y,
        };
    }
}
