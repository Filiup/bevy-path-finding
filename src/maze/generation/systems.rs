use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::prelude::*;
use std::convert::identity;

use crate::maze::window::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{
    components::{Direction, MazeCell, MazeWall},
    events::DestroyWallsBetween,
    resources::{EntityStack, MazeCellGrid},
    BLOCK_SIZE, WALL_HEIGHT,
};

pub(crate) fn spawn_maze_cells(mut commands: Commands, mut maze_grid: ResMut<MazeCellGrid>) {
    let half_block_size = BLOCK_SIZE / 2.0;
    let half_wall_height = WALL_HEIGHT / 2.0;

    let rows = (WINDOW_HEIGHT / BLOCK_SIZE) as usize;
    let cols = (WINDOW_WIDTH / BLOCK_SIZE) as usize;

    for row in 0..rows {
        for col in 0..cols {
            let x_position = (col as f32) * BLOCK_SIZE + half_block_size;
            let y_position = (row as f32) * BLOCK_SIZE + half_block_size;

            let entity = commands
                .spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x_position, y_position, 0.0),

                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(BLOCK_SIZE as f32, BLOCK_SIZE as f32)),
                            ..default()
                        },
                        ..default()
                    },
                    MazeCell {
                        row,
                        col,
                        visited: false,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                0.0,
                                half_block_size - half_wall_height,
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::srgb(255.0, 0.0, 0.0),
                                custom_size: Some(Vec2::new(BLOCK_SIZE, WALL_HEIGHT)),
                                ..default()
                            },
                            ..default()
                        },
                        MazeWall {
                            direction: Direction::Top,
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                0.0,
                                -half_block_size + half_wall_height,
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::srgb(255.0, 0.0, 0.0),
                                custom_size: Some(Vec2::new(BLOCK_SIZE, WALL_HEIGHT)),
                                ..default()
                            },
                            ..default()
                        },
                        MazeWall {
                            direction: Direction::Bottom,
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                half_block_size - half_wall_height,
                                0.0,
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::srgb(255.0, 0.0, 0.0),
                                custom_size: Some(Vec2::new(WALL_HEIGHT, BLOCK_SIZE)),
                                ..default()
                            },
                            ..default()
                        },
                        MazeWall {
                            direction: Direction::Right,
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                -half_block_size + half_wall_height,
                                0.0,
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::srgb(255.0, 0.0, 0.0),
                                custom_size: Some(Vec2::new(WALL_HEIGHT, BLOCK_SIZE)),
                                ..default()
                            },
                            ..default()
                        },
                        MazeWall {
                            direction: Direction::Left,
                        },
                    ));
                })
                .id();

            maze_grid.add(row, col, entity);
        }
    }
}

pub(crate) fn stack_add_first_mazecell(
    mut entity_stack: ResMut<EntityStack>,
    entity_query: Query<Entity, With<MazeCell>>,
) {
    let entity = entity_query.into_iter().next().unwrap();
    entity_stack.push(entity);
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

fn get_walls_to_destroy(
    d_col: i32,
    d_row: i32,
    current_walls: HashMap<Direction, Entity>,
    neighbor_walls: HashMap<Direction, Entity>,
) -> [Option<Entity>; 2] {
    let mut entities_destroy = [None; 2];

    if d_col == 1 {
        entities_destroy[0] = current_walls.get(&Direction::Left).copied();
        entities_destroy[1] = neighbor_walls.get(&Direction::Right).copied();
    }

    if d_col == -1 {
        entities_destroy[0] = current_walls.get(&Direction::Right).copied();
        entities_destroy[1] = neighbor_walls.get(&Direction::Left).copied();
    }

    if d_row == -1 {
        entities_destroy[0] = current_walls.get(&Direction::Top).copied();
        entities_destroy[1] = neighbor_walls.get(&Direction::Bottom).copied();
    }

    if d_row == 1 {
        entities_destroy[0] = current_walls.get(&Direction::Bottom).copied();
        entities_destroy[1] = neighbor_walls.get(&Direction::Top).copied();
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

        assert!(d_col >= -1 && d_col <= 1);
        assert!(d_row >= -1 && d_row <= 1);

        let walls_to_destroy = get_walls_to_destroy(d_col, d_row, current_walls, neighbor_walls);

        walls_to_destroy
            .into_iter()
            .filter_map(identity)
            .for_each(|entity| commands.entity(entity).despawn());
    }
}

pub fn generate_maze(
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
