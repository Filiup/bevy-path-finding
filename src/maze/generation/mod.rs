use bevy::prelude::*;
use resources::{CellGrid, MazeCellEntityStack};
use systems::{spawn_maze_cells, stack_add_first_mazecell};

pub const BLOCK_SIZE: f32 = 40.0;
pub const WALL_HEIGHT: f32 = 2.0;

pub(crate) mod components;
pub(crate) mod resources;
pub(crate) mod systems;

pub struct MazeGenerationPlugin;

impl Plugin for MazeGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CellGrid>()
            .init_resource::<MazeCellEntityStack>()
            .add_systems(
                Startup,
                (
                    spawn_maze_cells,
                    stack_add_first_mazecell.after(spawn_maze_cells),
                ),
            );
    }
}
