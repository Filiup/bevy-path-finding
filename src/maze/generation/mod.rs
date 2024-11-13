mod cell;
mod color;
mod grid;
mod stack;
mod walls;

use bevy::prelude::*;
use cell::{iterate_cells, CellIterationTimer};

use color::{
    current::{change_current_color, ChangeCurrentColor},
    neighbor::{
        change_neighbor_color, reset_neighbor_color, ChangeNeighborColor, NeighborColorStack,
        ResetNeighborsColor,
    },
};

use grid::{spawn_grid, MazeCellGrid};
use stack::{stack_add_first_mazecell, EntityStack};
use walls::{destroy_walls, DestroyWallsBetween};

pub const BLOCK_SIZE: f32 = 40.0;
pub const WALL_HEIGHT: f32 = 2.0;

pub struct MazeGenerationPlugin;

impl Plugin for MazeGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeCellGrid>()
            .init_resource::<EntityStack>()
            .init_resource::<CellIterationTimer>()
            .init_resource::<NeighborColorStack>()
            .add_event::<DestroyWallsBetween>()
            .add_event::<ResetNeighborsColor>()
            .add_event::<ChangeNeighborColor>()
            .add_event::<ChangeCurrentColor>()
            .add_systems(
                Startup,
                (spawn_grid, stack_add_first_mazecell.after(spawn_grid)),
            )
            .add_systems(
                Update,
                (
                    iterate_cells,
                    destroy_walls,
                    change_neighbor_color,
                    reset_neighbor_color.after(iterate_cells),
                    change_current_color,
                ),
            );
    }
}
