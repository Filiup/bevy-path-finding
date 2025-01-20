use std::time::Duration;

use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct MazeSolvingTimer {
    pub timer: Timer,
}

impl Default for MazeSolvingTimer {
    fn default() -> Self {
        MazeSolvingTimer {
            timer: Timer::new(Duration::from_millis(300), TimerMode::Repeating),
        }
    }
}
