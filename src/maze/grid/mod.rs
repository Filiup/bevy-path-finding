mod systems;

use bevy::{prelude::*, utils::HashMap};
use systems::spawn_grid;

#[derive(Resource, Default, Debug)]
pub struct MazeCellGrid(HashMap<(usize, usize), Entity>);

impl MazeCellGrid {
    pub(crate) fn add(&mut self, row: usize, col: usize, entity: Entity) -> Option<Entity> {
        self.0.insert((row, col), entity)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Entity> {
        self.0.get(&(row, col)).copied()
    }
}

pub struct MazeGridPlugin;

impl Plugin for MazeGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeCellGrid>()
            .add_systems(PreStartup, spawn_grid);
    }
}
