use specs::{Component, VecStorage};
use raylib::math::Vector2;
use std::ops::{Mul, Add, AddAssign, Sub, SubAssign, MulAssign, Div, DivAssign, Neg};

#[derive(Component, Debug, PartialEq, Default, Copy, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            x,
            y
        }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f32> for Position {
    type Output = Position;
    fn add(self, value: f32) -> Self {
        Position {
            x: self.x + value,
            y: self.y + value,
        }
    }
}


impl AddAssign for Position {
    fn add_assign(&mut self, v: Position) {
        *self = *self + v;
    }
}

impl AddAssign<f32> for Position {
    fn add_assign(&mut self, value: f32) {
        *self = *self + value;
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, v: Position) -> Self {
        Position {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Sub<f32> for Position {
    type Output = Position;
    fn sub(self, value: f32) -> Self {
        Position {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, v: Position) {
        *self = *self - v;
    }
}

impl SubAssign<f32> for Position {
    fn sub_assign(&mut self, value: f32) {
        *self = *self - value;
    }
}

impl Mul for Position {
    type Output = Position;
    fn mul(self, v: Position) -> Self {
        Position {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

impl Mul<f32> for Position {
    type Output = Position;
    fn mul(self, value: f32) -> Self {
        Position {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl MulAssign for Position {
    fn mul_assign(&mut self, v: Position) {
        *self = *self * v;
    }
}

impl MulAssign<f32> for Position {
    fn mul_assign(&mut self, value: f32) {
        *self = *self * value;
    }
}

impl Div for Position {
    type Output = Position;
    fn div(self, v: Position) -> Self {
        Position {
            x: self.x / v.x,
            y: self.y / v.y,
        }
    }
}

impl Div<f32> for Position {
    type Output = Position;
    fn div(self, value: f32) -> Self {
        Position {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl DivAssign for Position {
    fn div_assign(&mut self, v: Position) {
        *self = *self / v;
    }
}

impl DivAssign<f32> for Position {
    fn div_assign(&mut self, value: f32) {
        *self = *self / value;
    }
}

impl Neg for Position {
    type Output = Position;
    fn neg(self) -> Self {
        Position {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<Vector2> for Position {
    fn from(v: Vector2) -> Position {
       Position {
            x:v.x,
            y: v.y
       }
    }
}

impl Into<Vector2> for Position {
    fn into(self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct Transform {
    pub width: f32,
    pub height: f32,
    pub position: Position
}
