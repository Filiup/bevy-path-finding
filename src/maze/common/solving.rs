use std::time::Duration;

use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

use crate::maze::constants::iterration::DEFAULT_MAZE_SOLVING_TIMER_VALUE;

#[derive(Resource)]
pub struct MazeSolvingTimer {
    pub timer: Timer,
}

impl Default for MazeSolvingTimer {
    fn default() -> Self {
        MazeSolvingTimer {
            timer: Timer::new(
                Duration::from_millis(DEFAULT_MAZE_SOLVING_TIMER_VALUE),
                TimerMode::Repeating,
            ),
        }
    }
}
