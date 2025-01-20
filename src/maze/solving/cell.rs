// TODO: Solving alghorithm
/* Požiadavky:
1. Inicializacia Cell od ktoreho sa zacina a cell kde sa bude koncit - DONE
1. Vytvorenie FIFO Queue - DONE
2. Pridanie miesta začiatku ako prvý element do Queue - Done


Algoritmuz:

1. Pokiaľ queue nie je prázdna
    1. zober z nej jeden cell a označ ho ako "visited" -> DONE
    2. nájdi všetkým sesedov, cez ktorých je možný prechod a neboli už navštívení ( o jedno vpred )
    3. pridaj všetkých nájdených susedov do queue */

use super::{queue::CellQueue, visited_set::VisitedCellSet};
use crate::maze::{
    common::{
        cell::{find_cell_neighbors, MazeCell},
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

pub fn iterate_cells(
    mut cell_queue: ResMut<CellQueue>,
    mut visited_cell_set: ResMut<VisitedCellSet>,

    cell_grid: Res<MazeCellGrid>,
    maze_cells_query: Query<(&MazeCell, &Children)>,
    walls_query: Query<&MazeWall>,
) {
    if let Some(current_entity) = cell_queue.dequeue() {
        visited_cell_set.insert(current_entity);

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
            });

        for ne in cell_neighbors {
            cell_queue.enqueue(ne);
        }
    }
}
