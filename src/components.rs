use specs::{Component, VecStorage};
use raylib::math::{Rectangle, Vector2};
use serde::{Serialize, Deserialize};
use raylib::consts::rIconDescription::*;

use std::ops::{Mul, Add, AddAssign, Deref,  Sub, SubAssign, MulAssign, Div, DivAssign, Neg};

#[derive(Component, Debug, PartialEq, Default, Copy, Clone, Serialize, Deserialize)]
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
    pub fn zero() -> Self {
        Position::new(0., 0.)
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

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct PlatformController {
    pub can_jump: bool,
    pub coyote_time: f32
}

impl PlatformController {
    pub fn new() -> PlatformController {
        PlatformController {
            can_jump: false,
            coyote_time: 0.
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct Moveable {
    pub velocity: Position,
    pub width: f32,
    pub height: f32
}

impl Moveable {
    pub fn new() -> Self {
        Moveable {
            velocity: Position::new(0., 0.),
            width: 32.,
            height: 32.
        }
    }
    pub fn to_hitbox(&self, position: Position) -> Hitbox {
        Hitbox {
            position,
            width: self.width,
            height: self.height
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct FallingBlock {
    pub count: u32,
    pub should_fall: bool
}

impl FallingBlock {
    pub fn default() -> Self {
        FallingBlock {
            count: 0,
            should_fall: false,
        }
    }
}


#[derive(Component, Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[storage(VecStorage)]
pub struct Hitbox {
    pub width: f32,
    pub height: f32,
    pub position: Position
}

impl Hitbox {
    pub fn new(x: f32, y: f32) -> Self {
        Hitbox {
            position: Position::new(x, y),
            width: 32.,
            height: 32.
        }
    }
}

impl Into<Rectangle> for Hitbox {
    fn into(self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.width,
            height: self.height
        }
    }
}

impl CollisionsPoint<Position> for Hitbox {
    fn point_inside_rec(&self, point: Position) -> bool {
        let Hitbox {
            width, height, position: Position {
                x, y
            }, ..
        } = self;

        point.x > *x && x + width > point.x && point.y > *y && height + y > point.y
    }
}

impl CollisionsRec for Hitbox {
    fn collision_rec(self, other: Self) -> bool {
        let Position {
            x, y
        } = self.position;

        x < other.position.x + other.width && x + self.width > other.position.x && y < other.position.y + other.height && y + self.height > other.position.y
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct Triggerbox {
    pub width: f32,
    pub height: f32,
    pub position: Position
}

impl Into<Rectangle> for Triggerbox {
    fn into(self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.width,
            height: self.height
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub position: Position
}

impl Into<Rectangle> for Rect {
    fn into(self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.width,
            height: self.height
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct DragBox {
    pub dragging: bool,
    pub drag_offset: Position,
}

impl DragBox {
    pub fn default() -> Self {
        DragBox {
            dragging: false,
            drag_offset: Position::zero()
        }
    }
}

#[derive(Component, Debug, Default, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Sprite {
    pub name: String
}


pub trait CollisionsPoint<T> {
    fn point_inside_rec(&self, point: T) -> bool;
}

pub trait CollisionsRec {
    fn collision_rec(self, other: Self) -> bool;
}

use strum_macros::EnumIter; // etc.

#[derive(AsStaticStr, EnumIter,  Debug, Clone, Copy, PartialEq)]
pub enum ToolPalette {
    Block,
    FallingBlock,
    SpikeBlock
}

impl Default for ToolPalette {
    fn default() -> Self {
        ToolPalette::Block
    }
}


#[derive(Component, Debug, Default, Clone)]
#[storage(VecStorage)]
pub struct EditBtn {
   pub bounds: Rect,
   pub text: String,
   pub ty: ToolPalette
}

impl CollisionsPoint<Position> for EditBtn {
    fn point_inside_rec(&self, point: Position) -> bool {
        let Rect {
            width, height, position: Position {
                x, y
            }
        } = self.bounds;
        
        point.x > x && x + width > point.x && point.y > y && height + y > point.y
    }
}

#[derive(Debug, Clone)]
pub struct _Icon (pub raylib::consts::rIconDescription);

impl Default for _Icon {
    fn default() -> Self {
        _Icon (RICON_BIN)
    }
}

#[derive(Component, Debug, Default, Clone)]
#[storage(VecStorage)]
pub struct Icon {
    pub icon: _Icon,
    pub position: Position,
}

impl Icon {
    pub fn new(icon: raylib::consts::rIconDescription, position: Position) -> Self {
        Icon {
            icon: _Icon(icon),
            position
        }
    }
}

pub struct EditState {
    pub should_save: bool,
    pub editting: bool
}

impl EditState {
    pub fn new() -> EditState {
        EditState {
            should_save: false,
            editting: true,
        }
    }
}