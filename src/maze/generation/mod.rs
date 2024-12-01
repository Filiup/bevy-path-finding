pub mod cell;
mod color;
mod stack;
mod walls;

use bevy::prelude::*;
use cell::{iterate_cells, CellIterationTimer};

use color::{change_stack_color, reset_stack_color, ChangeStackColor, ResetStackColor};
use stack::{stack_add_first_mazecell, EntityStack};
use walls::{destroy_walls, DestroyWallsBetween};

use super::states::MazeState;

pub const CELL_STACK_COLOR: Color = Color::srgb(173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0);
pub const NEIGHBOR_COLOR: Color = Color::srgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0);
pub const DEFAULT_CELL_ITERATION_TIMER_VALUE: u64 = 1000;

pub struct MazeGenerationPlugin;

impl Plugin for MazeGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityStack>()
            .init_resource::<CellIterationTimer>()
            .add_event::<DestroyWallsBetween>()
            .add_event::<ChangeStackColor>()
            .add_event::<ResetStackColor>()
            .add_systems(Startup, stack_add_first_mazecell)
            .add_systems(
                Update,
                (
                    iterate_cells,
                    destroy_walls,
                    change_stack_color,
                    reset_stack_color.after(iterate_cells),
                )
                    .run_if(in_state(MazeState::Generation)),
            );
    }
}
