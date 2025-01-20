use std::time::Duration;

use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

use crate::maze::constants::iterration::DEFAULT_MAZE_GENERATION_TIMER_VALUE;

#[derive(Resource)]
pub struct MazeGenerationTimer {
    pub timer: Timer,
}

impl Default for MazeGenerationTimer {
    fn default() -> Self {
        MazeGenerationTimer {
            timer: Timer::new(
                Duration::from_millis(DEFAULT_MAZE_GENERATION_TIMER_VALUE),
                TimerMode::Repeating,
            ),
        }
    }
}
