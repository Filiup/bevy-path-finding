use std::time::Duration;

use bevy::{
    prelude::{Component, Resource},
    time::{Timer, TimerMode},
};

use crate::maze::constants::generation::DEFAULT_CELL_ITERATION_TIMER_VALUE;

#[derive(Component, Debug)]
pub struct MazeCell {
    pub row: usize,
    pub col: usize,
    pub visited: bool,
}

#[derive(Resource)]
pub struct MazeGenerationTimer {
    pub timer: Timer,
}

impl Default for MazeGenerationTimer {
    fn default() -> Self {
        MazeGenerationTimer {
            timer: Timer::new(
                Duration::from_millis(DEFAULT_CELL_ITERATION_TIMER_VALUE),
                TimerMode::Repeating,
            ),
        }
    }
}
