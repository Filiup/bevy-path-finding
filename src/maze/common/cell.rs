use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct MazeCell {
    pub row: usize,
    pub col: usize,
    pub visited: bool,
}
