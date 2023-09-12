use std::ops::{Add, Sub, Div, Mul, MulAssign, SubAssign, AddAssign, Neg};

use super::ffi as ffi;

type Pos2 = Vec2;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2{
    pub x: f32,
    pub y: f32,
}
impl Neg for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn neg(self) -> Vec2 {
        vec2(-self.x, -self.y)
    }
}

impl AddAssign for Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec2) {
        *self = Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl SubAssign for Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Element-wise multiplication
impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * vec.x,
            y: self.y * vec.y,
        }
    }
}

/// Element-wise division
impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: self * vec.x,
            y: self * vec.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x / factor,
            y: self.y / factor,
        }
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.1} {:.1}]", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32)->Self{
        Self{x, y}
    }
    pub const fn splat(v: f32)->Self{
        Self { x: v, y: v }
    }
    pub const X: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const Y: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
    pub const UP: Vec2 = Vec2 { x: 0.0, y: -1.0 };
    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    pub fn smaller_comp(&self)->f32{
        self.x.min(self.y)
    }

}
impl Into<ffi::Vector2> for Vec2 {
    fn into(self) -> ffi::Vector2 {
        ffi::Vector2 { x: self.x, y: self.y }
    }
}
impl Into<Vec2> for ffi::Vector2 {
    fn into(self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
}
pub fn vec2(x: f32, y: f32)->Vec2{
    Vec2 { x, y }
}
pub fn pos2(x: f32, y: f32)->Vec2{
    Vec2 { x, y }
}
#[derive(Clone, Copy, PartialEq)]
pub struct Rect{
    pub min: Vec2,
    pub max: Vec2,
}

impl Into<ffi::Rectangle> for Rect {
    fn into(self) -> ffi::Rectangle {
        ffi::Rectangle { x: self.min.x, y: self.min.y, width: self.width(), height: self.height() }
    }
}
impl Into<Rect> for ffi::Rectangle {
    fn into(self) -> Rect {
        Rect::from_min_size(Vec2{x: self.x, y: self.y}, Vec2{x: self.width, y: self.height})
    }
}

impl Rect{
    pub fn center(&self) -> Pos2 {
        Pos2 {
            x: (self.min.x + self.max.x) / 2.0,
            y: (self.min.y + self.max.y) / 2.0,
        }
    }

    pub fn new(x: f32, y: f32, width: f32, height: f32)->Self{
        Self { min: vec2(x, y), max: vec2(x+width, y+height) }
    }
    pub fn from_min_max(min: Vec2, max: Vec2) -> Self{
        Self { min, max }
    }
    #[must_use]
    pub fn shrink(self, amnt: f32) -> Self {
        self.shrink2(Vec2::splat(amnt))
    }

    #[must_use]
    pub fn shrink2(self, amnt: Vec2) -> Self {
        Rect::from_min_max(self.min + amnt, self.max - amnt)
    }
    #[inline(always)]
    pub fn from_min_size(min: Vec2, size: Vec2) -> Self {
        Rect {
            min,
            max: min + size,
        }
    }
    #[inline(always)]
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }



}
impl Rect {
    /// `min.x`
    #[inline(always)]
    pub fn left(&self) -> f32 {
        self.min.x
    }

    /// `min.x`
    #[inline(always)]
    pub fn left_mut(&mut self) -> &mut f32 {
        &mut self.min.x
    }

    /// `min.x`
    #[inline(always)]
    pub fn set_left(&mut self, x: f32) {
        self.min.x = x;
    }

    /// `max.x`
    #[inline(always)]
    pub fn right(&self) -> f32 {
        self.max.x
    }

    /// `max.x`
    #[inline(always)]
    pub fn right_mut(&mut self) -> &mut f32 {
        &mut self.max.x
    }

    /// `max.x`
    #[inline(always)]
    pub fn set_right(&mut self, x: f32) {
        self.max.x = x;
    }

    /// `min.y`
    #[inline(always)]
    pub fn top(&self) -> f32 {
        self.min.y
    }

    /// `min.y`
    #[inline(always)]
    pub fn top_mut(&mut self) -> &mut f32 {
        &mut self.min.y
    }

    /// `min.y`
    #[inline(always)]
    pub fn set_top(&mut self, y: f32) {
        self.min.y = y;
    }

    /// `max.y`
    #[inline(always)]
    pub fn bottom(&self) -> f32 {
        self.max.y
    }

    /// `max.y`
    #[inline(always)]
    pub fn bottom_mut(&mut self) -> &mut f32 {
        &mut self.max.y
    }

    /// `max.y`
    #[inline(always)]
    pub fn set_bottom(&mut self, y: f32) {
        self.max.y = y;
    }

    #[inline(always)]
    pub fn left_top(&self) -> Pos2 {
        pos2(self.left(), self.top())
    }

    #[inline(always)]
    pub fn center_top(&self) -> Pos2 {
        pos2(self.center().x, self.top())
    }

    #[inline(always)]
    pub fn right_top(&self) -> Pos2 {
        pos2(self.right(), self.top())
    }

    #[inline(always)]
    pub fn left_center(&self) -> Pos2 {
        pos2(self.left(), self.center().y)
    }

    #[inline(always)]
    pub fn right_center(&self) -> Pos2 {
        pos2(self.right(), self.center().y)
    }

    #[inline(always)]
    pub fn left_bottom(&self) -> Pos2 {
        pos2(self.left(), self.bottom())
    }

    #[inline(always)]
    pub fn center_bottom(&self) -> Pos2 {
        pos2(self.center().x, self.bottom())
    }

    #[inline(always)]
    pub fn right_bottom(&self) -> Pos2 {
        pos2(self.right(), self.bottom())
    }

    ///// Split rectangle in left and right halves. `t` is expected to be in the (0,1) range.
    //pub fn split_left_right_at_fraction(&self, t: f32) -> (Rect, Rect) {
    //    self.split_left_right_at_x(lerp(self.min.x..=self.max.x, t))
    //}

    /// Split rectangle in left and right halves at the given `x` coordinate.
    pub fn split_left_right_at_x(&self, split_x: f32) -> (Rect, Rect) {
        let left = Rect::from_min_max(self.min, Pos2::new(split_x, self.max.y));
        let right = Rect::from_min_max(Pos2::new(split_x, self.min.y), self.max);
        (left, right)
    }

    ///// Split rectangle in top and bottom halves. `t` is expected to be in the (0,1) range.
    //pub fn split_top_bottom_at_fraction(&self, t: f32) -> (Rect, Rect) {
    //    self.split_top_bottom_at_y(lerp(self.min.y..=self.max.y, t))
    //}

    /// Split rectangle in top and bottom halves at the given `y` coordinate.
    pub fn split_top_bottom_at_y(&self, split_y: f32) -> (Rect, Rect) {
        let top = Rect::from_min_max(self.min, Pos2::new(self.max.x, split_y));
        let bottom = Rect::from_min_max(Pos2::new(self.min.x, split_y), self.max);
        (top, bottom)
    }
}
