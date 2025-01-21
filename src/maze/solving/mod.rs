use bevy::app::Plugin;
use bevy::prelude::*;
use cell::iterate_cells;
use color::{change_queue_color, ChangeQueueColor};
use path::{draw_shortest_path, DrawShortestPath};
use predecessors::{reset_predecessors_map, PredecessorsMap};
use queue::reset_mazecell_queue;
use queue::{init_mazecell_queue, CellQueue};
use visited_set::{reset_visited_cell_set, VisitedCellSet};

use super::common::cell::{EndCell, StartCell};
use super::common::states::{MazeState, SolveState};

mod cell;
mod color;
mod path;
mod predecessors;
mod queue;
mod visited_set;

pub struct MazeSolvingPlugin;

pub fn reset_solving_route(
    mut commands: Commands,
    mut start_cell_query: Query<Entity, With<StartCell>>,
    mut end_cell_entity_query: Query<Entity, With<EndCell>>,
) {
    let start_cell_entity = start_cell_query.get_single_mut().unwrap();
    let end_cell_entity = end_cell_entity_query.get_single_mut().unwrap();

    commands.entity(start_cell_entity).remove::<StartCell>();
    commands.entity(end_cell_entity).remove::<EndCell>();
}

impl Plugin for MazeSolvingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CellQueue>()
            .init_resource::<VisitedCellSet>()
            .init_resource::<PredecessorsMap>()
            .add_event::<ChangeQueueColor>()
            .add_event::<DrawShortestPath>()
            .add_systems(
                OnEnter(MazeState::MazeSolve(SolveState::Solving)),
                init_mazecell_queue,
            )
            .add_systems(
                OnExit(MazeState::MazeSolve(SolveState::Solving)),
                (
                    reset_mazecell_queue,
                    reset_solving_route,
                    reset_predecessors_map,
                    reset_visited_cell_set,
                ),
            )
            .add_systems(
                Update,
                (iterate_cells, change_queue_color, draw_shortest_path)
                    .run_if(in_state(MazeState::MazeSolve(SolveState::Solving))),
            );
    }
}
