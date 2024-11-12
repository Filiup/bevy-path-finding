mod cell;
mod grid;
mod stack;
mod walls;

use bevy::prelude::*;
use cell::{
    change_cell_color, iterate_cells, reset_cell_color, CellIterationTimer, ChangeCellColor,
    ChangedColorStack, ResetCellColor,
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
            .init_resource::<ChangedColorStack>()
            .add_event::<DestroyWallsBetween>()
            .add_event::<ResetCellColor>()
            .add_event::<ChangeCellColor>()
            .add_systems(
                Startup,
                (spawn_grid, stack_add_first_mazecell.after(spawn_grid)),
            )
            .add_systems(
                Update,
                (
                    iterate_cells,
                    destroy_walls,
                    change_cell_color,
                    reset_cell_color.after(iterate_cells),
                ),
            );
    }
}
