/// f32 2d vector
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

/// u32 2d vector
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl UVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{UVec2, Vec2};

    #[test]
    fn make_vec2() {
        let vec = Vec2::new(1.0, 4.0);

        assert_eq!(vec, Vec2 { x: 1.0, y: 4.0 })
    }

    #[test]
    fn make_uvec2() {
        let vec = UVec2::new(1, 4);

        assert_eq!(vec, UVec2 { x: 1, y: 4 })
    }
}
