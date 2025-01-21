use super::{
    color::ChangeQueueColor, predecessors::PredecessorsMap, queue::CellQueue,
    visited_set::VisitedCellSet, EndCell,
};
use crate::maze::{
    common::{
        cell::{find_cell_neighbors, MazeCell},
        solving::MazeSolvingTimer,
        wall::{MazeWall, WallDirection},
    },
    grid::MazeCellGrid,
};
use bevy::prelude::*;
use std::collections::HashSet;

fn is_accessible_neigbor(
    current_cell: &MazeCell,
    neighbor_cell: &MazeCell,
    current_wall_directions: &HashSet<WallDirection>,
    neighbor_wall_directions: HashSet<WallDirection>,
) -> bool {
    let d_col = current_cell.col as i32 - neighbor_cell.col as i32;
    let d_row = current_cell.row as i32 - neighbor_cell.row as i32;

    assert!((-1..=1).contains(&d_col));
    assert!((-1..=1).contains(&d_row));

    match (d_col, d_row) {
        (1, _) => {
            !current_wall_directions.contains(&WallDirection::Left)
                && !neighbor_wall_directions.contains(&WallDirection::Right)
        }

        (-1, _) => {
            !current_wall_directions.contains(&WallDirection::Right)
                && !neighbor_wall_directions.contains(&WallDirection::Left)
        }

        (_, -1) => {
            !current_wall_directions.contains(&WallDirection::Top)
                && !neighbor_wall_directions.contains(&WallDirection::Bottom)
        }

        (_, 1) => {
            !current_wall_directions.contains(&WallDirection::Bottom)
                && !neighbor_wall_directions.contains(&WallDirection::Top)
        }

        _ => true,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn iterate_cells(
    cell_grid: Res<MazeCellGrid>,
    mut cell_queue: ResMut<CellQueue>,
    mut visited_cell_set: ResMut<VisitedCellSet>,
    mut solving_timer: ResMut<MazeSolvingTimer>,
    mut predecessors_map: ResMut<PredecessorsMap>,
    time: Res<Time>,

    mut change_color_writer: EventWriter<ChangeQueueColor>,

    maze_cells_query: Query<(&MazeCell, &Children)>,
    end_cell_entity_query: Query<Entity, With<EndCell>>,
    walls_query: Query<&MazeWall>,
) {
    solving_timer.timer.tick(time.delta());
    if !solving_timer.timer.finished() {
        return;
    }

    let end_cell_entity = end_cell_entity_query.get_single().unwrap();

    if let Some(current_entity) = cell_queue.dequeue() {
        visited_cell_set.insert(current_entity);

        if current_entity == end_cell_entity {
            cell_queue.clear();
            return;
        }

        let (current_cell, current_children) = maze_cells_query.get(current_entity).unwrap();
        let current_wall_directions = current_children
            .iter()
            .filter_map(|&we| walls_query.get(we).ok())
            .map(|wc| wc.direction)
            .collect::<HashSet<_>>();

        let cell_neighbors = find_cell_neighbors(current_cell, &cell_grid)
            .filter(|ne| !visited_cell_set.contains(ne))
            .filter(|&ne| {
                let (neighbor_cell, neighbor_children) = maze_cells_query.get(ne).unwrap();

                let neighbor_wall_directions = neighbor_children
                    .iter()
                    .filter_map(|&we| walls_query.get(we).ok())
                    .map(|wc| wc.direction)
                    .collect::<HashSet<_>>();

                is_accessible_neigbor(
                    current_cell,
                    neighbor_cell,
                    &current_wall_directions,
                    neighbor_wall_directions,
                )
            })
            .collect::<Vec<_>>();

        change_color_writer.send(ChangeQueueColor {
            cell: current_entity,
            neighbor_cells: cell_neighbors.clone(),
        });

        for ne in cell_neighbors {
            predecessors_map.insert(ne, current_entity);
            cell_queue.enqueue(ne);
        }
    }
}
