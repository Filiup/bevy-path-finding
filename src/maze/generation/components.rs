use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MazeCell {
    pub visited: bool,

    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MazeWall {
    pub direction: Direction,
}
