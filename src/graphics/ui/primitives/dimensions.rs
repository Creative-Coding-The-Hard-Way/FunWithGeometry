use crate::graphics::{ui::primitives::Rect, vec2, Vec2};

/// The Dimensions of something on screen.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    /// Create a new dimensions object.
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Take the minimum values of the two dimensions.
    #[inline]
    pub fn min(&self, other: &Self) -> Self {
        Self::new(self.width.min(other.width), self.height.min(other.height))
    }

    /// Take the maximum values of the two dimensions.
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self::new(self.width.max(other.width), self.height.max(other.height))
    }

    /// Create a rect with top-left at (0, 0) and dimensions matching these
    /// dimensions.
    pub fn as_rect(&self) -> Rect {
        Rect::new(0.0, 0.0, self.height, self.width)
    }
}

impl From<Vec2> for Dimensions {
    fn from(vec: Vec2) -> Self {
        Self::new(vec.x, vec.y)
    }
}

impl From<Dimensions> for Vec2 {
    fn from(dimensions: Dimensions) -> Self {
        vec2(dimensions.width, dimensions.height)
    }
}

impl From<(i32, i32)> for Dimensions {
    fn from((w, h): (i32, i32)) -> Self {
        Self::new(w as f32, h as f32)
    }
}

impl From<(f32, f32)> for Dimensions {
    fn from((w, h): (f32, f32)) -> Self {
        Self::new(w, h)
    }
}
