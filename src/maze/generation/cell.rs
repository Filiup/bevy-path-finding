use crate::maze::{
    common::{
        cell::{find_cell_neighbors, MazeCell},
        generation::MazeGenerationTimer,
    },
    grid::MazeCellGrid,
};

use super::{
    color::{ChangeStackColor, ResetStackColor},
    stack::EntityStack,
    walls::DestroyWallsBetween,
};
use bevy::prelude::*;
use rand::prelude::*;

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

        let neighbors = find_cell_neighbors(&current_cell, &cell_grid)
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
