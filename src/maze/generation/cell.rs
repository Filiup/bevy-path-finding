use super::{grid::MazeCellGrid, stack::EntityStack, walls::DestroyWallsBetween};
use bevy::prelude::*;
use rand::prelude::*;
use std::convert::identity;

#[derive(Component, Debug)]
pub struct MazeCell {
    pub row: usize,
    pub col: usize,
    pub visited: bool,
}

fn find_neighbors(maze_cell: &MazeCell, cell_grid: &MazeCellGrid) -> impl Iterator<Item = Entity> {
    let find_neighbor = |row, col| cell_grid.get(row?, col?);

    let top_neighbor = find_neighbor(maze_cell.row.checked_add(1), Some(maze_cell.col));
    let bottom_neighbor = find_neighbor(maze_cell.row.checked_sub(1), Some(maze_cell.col));

    let left_neighbor = find_neighbor(Some(maze_cell.row), maze_cell.col.checked_sub(1));
    let right_neighbor = find_neighbor(Some(maze_cell.row), maze_cell.col.checked_add(1));

    [top_neighbor, bottom_neighbor, left_neighbor, right_neighbor]
        .into_iter()
        .filter_map(identity)
}

pub fn iterate_cells(
    mut cell_stack: ResMut<EntityStack>,
    cell_grid: Res<MazeCellGrid>,
    mut destroy_walls_writer: EventWriter<DestroyWallsBetween>,
    mut maze_cells_query: Query<(&mut MazeCell, &mut Sprite)>,
) {
    if let Some(current_entity) = cell_stack.pop() {
        let (mut current_cell, mut current_sprite) =
            maze_cells_query.get_mut(current_entity).unwrap();

        current_sprite.color = Color::BLACK;
        current_cell.visited = true;

        let neighbors = find_neighbors(&current_cell, &cell_grid)
            .filter(|&entity| {
                let (neighbor_cell, _) = maze_cells_query.get(entity).unwrap();
                !neighbor_cell.visited
            })
            .collect::<Vec<_>>();

        let choosen_neighbor = neighbors.choose(&mut rand::thread_rng());

        if let Some(&neighbor_entity) = choosen_neighbor {
            cell_stack.push(current_entity);
            {
                let (mut neighbor_cell, _) = maze_cells_query.get_mut(neighbor_entity).unwrap();
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
