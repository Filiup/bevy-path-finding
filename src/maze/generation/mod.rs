pub mod cell;
mod color;
mod stack;
mod walls;

use bevy::prelude::*;
use cell::iterate_cells;

use color::{change_stack_color, reset_stack_color, ChangeStackColor, ResetStackColor};
use stack::{stack_add_first_mazecell, EntityStack};
use walls::{destroy_walls, DestroyWallsBetween};

use super::states::{MazeState, MenuState};

pub struct MazeGenerationPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GenerationState {
    #[default]
    Running,
    Finished,
}

fn change_generation_state(
    cell_stack: Res<EntityStack>,
    mut generation_state: ResMut<NextState<GenerationState>>,
) {
    if cell_stack.is_empty() {
        generation_state.set(GenerationState::Finished);
    }
}

fn change_maze_state(mut maze_state: ResMut<NextState<MazeState>>) {
    maze_state.set(MazeState::MainMenu(MenuState::WithMaze));
}

impl Plugin for MazeGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityStack>()
            .init_state::<GenerationState>()
            .add_event::<DestroyWallsBetween>()
            .add_event::<ChangeStackColor>()
            .add_event::<ResetStackColor>()
            .add_systems(Startup, stack_add_first_mazecell)
            .add_systems(OnEnter(GenerationState::Finished), change_maze_state)
            .add_systems(
                Update,
                (
                    change_generation_state,
                    iterate_cells,
                    destroy_walls,
                    change_stack_color,
                    reset_stack_color.after(iterate_cells),
                )
                    .run_if(in_state(MazeState::MazeGeneration)),
            );
    }
}
