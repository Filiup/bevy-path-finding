mod read;
mod save;
use std::{collections::HashSet, fmt};

use bevy::{app::{Plugin, Update}, utils::{hashbrown::hash_map, HashMap}};

pub use read::*;
pub use save::*;

use super::common::wall::WallDirection;

pub struct MazeStoragePlugin;

impl Plugin for MazeStoragePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SaveMazeEvent>()
            .add_systems(Update, save_maze);
    }
}

pub(crate) struct MazeStorage(HashMap<(usize, usize), HashSet<WallDirection>>);

impl IntoIterator for MazeStorage {
    type Item = ((usize, usize), HashSet<WallDirection>);
    type IntoIter = hash_map::IntoIter<(usize, usize), HashSet<WallDirection>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for MazeStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ((row, col), directions) in &self.0 {
            let directions_str = directions.iter().map(|v| v.to_string()).collect::<String>();

            write!(f, "{}.{}.{};", row, col, directions_str)?;
        }
        Ok(())
    }
}
