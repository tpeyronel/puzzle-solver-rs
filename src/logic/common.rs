use std::ops::{Add, AddAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: u32,
    pub y: u32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Self::splat(0);
    pub const MAX: Vec2 = Self::splat(u32::MAX);
    pub const MIN: Vec2 = Self::splat(u32::MIN);

    #[inline]
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(v: u32) -> Self {
        Self { x: v, y: v }
    }

    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: u32::min(self.x, rhs.x),
            y: u32::min(self.y, rhs.y),
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl AddAssign<Vec2> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl SubAssign<Vec2> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl From<(u32, u32)> for Vec2 {
    fn from(p: (u32, u32)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl From<Vec2i> for Vec2 {
    fn from(v: Vec2i) -> Self {
        Self {
            x: v.x as u32,
            y: v.y as u32,
        }
    }
}

pub type Vec2i = glam::i32::IVec2;
