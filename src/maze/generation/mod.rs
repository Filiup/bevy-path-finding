use bevy::prelude::*;
use cell::iterate_cells;
use grid::{spawn_grid, MazeCellGrid};
use stack::{stack_add_first_mazecell, EntityStack};
use walls::{destroy_walls, DestroyWallsBetween};

pub const BLOCK_SIZE: f32 = 40.0;
pub const WALL_HEIGHT: f32 = 2.0;

pub(crate) mod cell;
pub(crate) mod grid;
pub(crate) mod stack;
pub(crate) mod walls;

pub struct MazeGenerationPlugin;

impl Plugin for MazeGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeCellGrid>()
            .init_resource::<EntityStack>()
            .add_event::<DestroyWallsBetween>()
            .add_systems(
                Startup,
                (spawn_grid, stack_add_first_mazecell.after(spawn_grid)),
            )
            .add_systems(Update, (iterate_cells, destroy_walls));
    }
}
