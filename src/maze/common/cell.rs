use crate::maze::grid::MazeCellGrid;
use bevy::prelude::{Component, Entity};

#[derive(Component, Debug)]
pub struct MazeCell {
    pub row: usize,
    pub col: usize,
    pub visited: bool,
}

#[derive(Component, Clone, Copy)]
pub struct StartCell;

#[derive(Component, Clone, Copy)]
pub struct EndCell;

pub trait CellPathCompoment: Component + Copy {}
impl CellPathCompoment for StartCell {}
impl CellPathCompoment for EndCell {}

pub fn find_cell_neighbors(
    maze_cell: &MazeCell,
    cell_grid: &MazeCellGrid,
) -> impl Iterator<Item = Entity> {
    let find_neighbor = |row, col| cell_grid.get(row?, col?);

    let top_neighbor = find_neighbor(maze_cell.row.checked_add(1), Some(maze_cell.col));
    let bottom_neighbor = find_neighbor(maze_cell.row.checked_sub(1), Some(maze_cell.col));

    let left_neighbor = find_neighbor(Some(maze_cell.row), maze_cell.col.checked_sub(1));
    let right_neighbor = find_neighbor(Some(maze_cell.row), maze_cell.col.checked_add(1));

    [top_neighbor, bottom_neighbor, left_neighbor, right_neighbor]
        .into_iter()
        .flatten()
}
