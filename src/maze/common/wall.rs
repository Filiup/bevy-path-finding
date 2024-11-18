use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WallDirection {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MazeWall {
    pub direction: WallDirection,
}
