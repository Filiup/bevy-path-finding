mod systems;

use bevy::{prelude::*, utils::HashMap};
pub use systems::spawn_grid;

pub const BLOCK_SIZE: f32 = 40.0;
pub const WALL_HEIGHT: f32 = 2.0;

pub const CELL_COLOR: Color = Color::WHITE;
pub const WALL_COLOR: Color = Color::BLACK;

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
            .add_systems(Startup, spawn_grid);
    }
}
