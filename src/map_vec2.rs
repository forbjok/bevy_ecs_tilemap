use bevy::prelude::*;

#[derive(Debug, Default, Hash, Eq, PartialEq, Clone, Copy)]
pub struct MapVec2 {
    pub x: i32,
    pub y: i32,
}

impl MapVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn new_f(x: f32, y: f32) -> Self {
        Self::new(x as i32, y as i32)
    }
}

impl Into<Vec2> for MapVec2 {
    fn into(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl From<Vec2> for MapVec2 {
    fn from(a: Vec2) -> MapVec2 {
        MapVec2 {
            x: a.x as i32,
            y: a.y as i32,
        }
    }
}