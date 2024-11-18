use bevy::prelude::{Entity, Event};
use bevy::{prelude::*, utils::HashMap};

use super::cell::MazeCell;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WallDirection {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MazeWall {
    pub direction: WallDirection,
}

#[derive(Event)]
pub(crate) struct DestroyWallsBetween {
    pub current: Entity,
    pub neighbor: Entity,
}

fn get_walls_to_destroy(
    d_col: i32,
    d_row: i32,
    current_walls: HashMap<WallDirection, Entity>,
    neighbor_walls: HashMap<WallDirection, Entity>,
) -> [Option<Entity>; 2] {
    let mut entities_destroy = [None; 2];

    if d_col == 1 {
        entities_destroy[0] = current_walls.get(&WallDirection::Left).copied();
        entities_destroy[1] = neighbor_walls.get(&WallDirection::Right).copied();
    }

    if d_col == -1 {
        entities_destroy[0] = current_walls.get(&WallDirection::Right).copied();
        entities_destroy[1] = neighbor_walls.get(&WallDirection::Left).copied();
    }

    if d_row == -1 {
        entities_destroy[0] = current_walls.get(&WallDirection::Top).copied();
        entities_destroy[1] = neighbor_walls.get(&WallDirection::Bottom).copied();
    }

    if d_row == 1 {
        entities_destroy[0] = current_walls.get(&WallDirection::Bottom).copied();
        entities_destroy[1] = neighbor_walls.get(&WallDirection::Top).copied();
    }

    entities_destroy
}

pub fn destroy_walls(
    mut commands: Commands,
    mut destroy_walls_reader: EventReader<DestroyWallsBetween>,
    maze_cells_query: Query<(&MazeCell, &Children)>,
    walls_query: Query<&MazeWall>,
) {
    for DestroyWallsBetween { current, neighbor } in destroy_walls_reader.read() {
        let [(current_cell, current_childrens), (neighbor_cell, neighbor_childrens)] =
            maze_cells_query.many([*current, *neighbor]);

        let current_walls =
            current_childrens
                .into_iter()
                .fold(HashMap::new(), |mut acc, &entity| {
                    let wall = walls_query.get(entity);
                    if let Ok(wall) = wall {
                        acc.insert(wall.direction, entity);
                    }

                    acc
                });
        let neighbor_walls =
            neighbor_childrens
                .into_iter()
                .fold(HashMap::new(), |mut acc, &entity| {
                    let wall = walls_query.get(entity);
                    if let Ok(wall) = wall {
                        acc.insert(wall.direction, entity);
                    }

                    acc
                });

        let d_col = current_cell.col as i32 - neighbor_cell.col as i32;
        let d_row = current_cell.row as i32 - neighbor_cell.row as i32;

        assert!((-1..=1).contains(&d_col));
        assert!((-1..=1).contains(&d_row));

        let walls_to_destroy = get_walls_to_destroy(d_col, d_row, current_walls, neighbor_walls);

        walls_to_destroy
            .into_iter()
            .flatten()
            .for_each(|entity| commands.entity(entity).despawn());
    }
}
