use bevy::prelude::Component;
use std::{fmt, str::FromStr};

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

impl FromStr for WallDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T" => Ok(Self::Top),
            "B" => Ok(Self::Bottom),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("'{}' is not a valid enum variant", s)),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MazeWall {
    pub direction: WallDirection,
}
