mod save;
use bevy::app::{Plugin, Update};
pub use save::*;

pub struct MazeStoragePlugin;

impl Plugin for MazeStoragePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SaveMazeEvent>()
            .add_systems(Update, save_maze);
    }
}
