use crate::maze::{
    common::cell::{MazeCell, MazeGenerationTimer},
    grid::MazeCellGrid,
};

use super::{
    color::{ChangeStackColor, ResetStackColor},
    stack::EntityStack,
    walls::DestroyWallsBetween,
};
use bevy::prelude::*;
use rand::prelude::*;

fn find_neighbors(maze_cell: &MazeCell, cell_grid: &MazeCellGrid) -> impl Iterator<Item = Entity> {
    let find_neighbor = |row, col| cell_grid.get(row?, col?);

    let top_neighbor = find_neighbor(maze_cell.row.checked_add(1), Some(maze_cell.col));
    let bottom_neighbor = find_neighbor(maze_cell.row.checked_sub(1), Some(maze_cell.col));

    let left_neighbor = find_neighbor(Some(maze_cell.row), maze_cell.col.checked_sub(1));
    let right_neighbor = find_neighbor(Some(maze_cell.row), maze_cell.col.checked_add(1));

    [top_neighbor, bottom_neighbor, left_neighbor, right_neighbor]
        .into_iter()
        .flatten()
}

#[allow(clippy::too_many_arguments)]
pub fn iterate_cells(
    cell_grid: Res<MazeCellGrid>,
    time: Res<Time>,
    mut cell_stack: ResMut<EntityStack>,
    mut cell_iteration_timer: ResMut<MazeGenerationTimer>,

    mut destroy_walls_writer: EventWriter<DestroyWallsBetween>,
    mut change_color_writer: EventWriter<ChangeStackColor>,
    mut reset_color_writer: EventWriter<ResetStackColor>,

    mut maze_cells_query: Query<&mut MazeCell>,
) {
    cell_iteration_timer.timer.tick(time.delta());
    if !cell_iteration_timer.timer.finished() {
        return;
    }

    if let Some(current_entity) = cell_stack.pop() {
        reset_color_writer.send(ResetStackColor {
            entity: current_entity,
        });
        let mut current_cell = maze_cells_query.get_mut(current_entity).unwrap();

        current_cell.visited = true;

        let neighbors = find_neighbors(&current_cell, &cell_grid)
            .filter(|&entity| {
                let neighbor_cell = maze_cells_query.get(entity).unwrap();
                !neighbor_cell.visited
            })
            .collect::<Vec<_>>();

        let choosen_neighbor = neighbors.choose(&mut rand::thread_rng());
        if let Some(&neighbor_entity) = choosen_neighbor {
            cell_stack.push(current_entity);
            change_color_writer.send(ChangeStackColor {
                cell: current_entity,
                neighbor_cells: neighbors.clone(),
            });

            {
                let mut neighbor_cell = maze_cells_query.get_mut(neighbor_entity).unwrap();
                neighbor_cell.visited = true;
            }

            destroy_walls_writer.send(DestroyWallsBetween {
                current: current_entity,
                neighbor: neighbor_entity,
            });
            cell_stack.push(neighbor_entity);
        }
    }
}
