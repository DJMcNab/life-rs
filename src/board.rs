use bevy::math::Vec2;
use std::ops::Add;

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub border: Vec2,
}

#[derive(Copy, Clone)]
pub struct CellPosition {
    x: i32,
    y: i32,
}

impl From<CellPosition> for Vec2 {
    fn from(pos: CellPosition) -> Self {
        Vec2::new(pos.x as f32, pos.y as f32)
    }
}

impl Add<CellPosition> for CellPosition {
    type Output = CellPosition;

    fn add(self, rhs: CellPosition) -> Self::Output {
        CellPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(i32, i32)> for CellPosition {
    fn from((x, y): (i32, i32)) -> Self {
        CellPosition { x, y }
    }
}

impl Board {
    pub fn new(width: i32, height: i32, border: f32) -> Self {
        Board {
            width,
            height,
            border: Vec2::new(border, border),
        }
    }

    pub fn length(&self) -> i32 {
        self.width * self.height
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }

    pub fn idx2vec(&self, index: i32) -> CellPosition {
        let x = index % self.width;
        let y = index / self.width;

        CellPosition { x, y }
    }

    pub fn vec2idx(&self, vec: CellPosition) -> i32 {
        vec.y * self.width + vec.x
    }
}
