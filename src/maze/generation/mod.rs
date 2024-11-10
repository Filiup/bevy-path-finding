use bevy::prelude::*;
use events::DestroyWallsBetween;
use resources::{EntityStack, MazeCellGrid};
use systems::{destroy_walls, generate_maze, spawn_maze_cells, stack_add_first_mazecell};

pub const BLOCK_SIZE: f32 = 40.0;
pub const WALL_HEIGHT: f32 = 2.0;

pub(crate) mod components;
pub(crate) mod events;
pub(crate) mod resources;
pub(crate) mod systems;

pub struct MazeGenerationPlugin;

impl Plugin for MazeGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeCellGrid>()
            .init_resource::<EntityStack>()
            .add_event::<DestroyWallsBetween>()
            .add_systems(
                Startup,
                (
                    spawn_maze_cells,
                    stack_add_first_mazecell.after(spawn_maze_cells),
                ),
            )
            .add_systems(Update, (generate_maze, destroy_walls));
    }
}
