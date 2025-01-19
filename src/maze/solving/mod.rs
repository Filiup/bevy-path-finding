use bevy::app::Plugin;
use bevy::prelude::*;
use queue::reset_mazecell_queue;
use queue::{init_mazecell_queue, CellQueue};

use super::common::cell::MazeCell;
use super::common::states::MazeState;
use super::constants::grid::*;
use super::constants::window::*;
use super::grid::MazeCellGrid;

mod queue;
pub struct MazeSolvingPlugin;

#[derive(Component)]
pub struct StartCell;

#[derive(Component)]
pub struct EndCell;

pub fn init_solving_route(
    mut commands: Commands,
    maze_cell_grid: Res<MazeCellGrid>,
    mut cell_sprite_query: Query<&mut Sprite, With<MazeCell>>,
) {
    let rows = (GRID_WINDOW_HEIGHT / BLOCK_SIZE) as usize;
    let cols = (GRID_WINDOW_WIDTH / BLOCK_SIZE) as usize;

    let start_cell_entity = maze_cell_grid.get(0, 0).unwrap();
    let end_cell_entity = maze_cell_grid.get(rows - 1, cols - 1).unwrap();

    commands.entity(start_cell_entity).insert(StartCell);
    commands.entity(end_cell_entity).insert(EndCell);

    let [mut start_cell_sprite, mut end_cell_sprite] = cell_sprite_query
        .get_many_mut([start_cell_entity, end_cell_entity])
        .unwrap();

    start_cell_sprite.color = START_CELL_COLOR;
    end_cell_sprite.color = END_CELL_COLOR;
}

pub fn reset_solving_route(
    mut commands: Commands,
    mut start_cell_query: Query<(Entity, &mut Sprite), With<StartCell>>,
    mut end_cell_entity_query: Query<(Entity, &mut Sprite), With<EndCell>>,
) {
    let (start_cell_entity, mut start_cell_sprite) = start_cell_query.get_single_mut().unwrap();
    let (end_cell_entity, mut end_cell_sprite) = end_cell_entity_query.get_single_mut().unwrap();

    commands.entity(start_cell_entity).remove::<StartCell>();
    commands.entity(end_cell_entity).remove::<EndCell>();

    start_cell_sprite.color = CELL_COLOR;
    end_cell_sprite.color = CELL_COLOR;
}

impl Plugin for MazeSolvingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CellQueue>()
            .add_systems(
                OnEnter(MazeState::MazeSolving),
                (
                    init_solving_route,
                    init_mazecell_queue.after(init_solving_route),
                ),
            )
            .add_systems(
                OnExit(MazeState::MazeSolving),
                (reset_mazecell_queue, reset_solving_route),
            );
    }
}
