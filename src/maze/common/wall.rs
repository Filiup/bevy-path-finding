use bevy::prelude::Component;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WallDirection {
    Top,
    Bottom,
    Left,
    Right,
}

impl fmt::Display for WallDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Top => write!(f, "T"),
            Self::Bottom => write!(f, "B"),
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MazeWall {
    pub direction: WallDirection,
}
