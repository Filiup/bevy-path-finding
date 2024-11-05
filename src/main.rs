use std::{cell::RefCell, convert::identity};

use bevy::{
    prelude::*,
    utils::hashbrown::HashMap,
    window::{PrimaryWindow, WindowResolution},
};

use rand::prelude::*;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub const BLOCK_SIZE: f32 = 50.0;
pub const WALL_HEIGHT: f32 = 2.0;

#[derive(Component, Debug)]
pub struct MazeCell {
    pub visited: bool,

    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MazeWall {
    direction: Direction,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_maze_cells,
                generate_maze.after(spawn_maze_cells),
            ),
        )
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let primary_window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            primary_window.width() / 2.0,
            primary_window.height() / 2.0,
            0.0,
        ),
        ..default()
    });
}

pub fn spawn_maze_cells(mut commands: Commands) {
    let half_block_size = BLOCK_SIZE / 2.0;
    let half_wall_height = WALL_HEIGHT / 2.0;

    let rows = (WINDOW_HEIGHT / BLOCK_SIZE) as usize;
    let cols = (WINDOW_WIDTH / BLOCK_SIZE) as usize;

    for row in 0..rows {
        for col in 0..cols {
            let x_position = (col as f32) * BLOCK_SIZE + half_block_size;
            let y_position = (row as f32) * BLOCK_SIZE + half_block_size;

            commands
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
                        visited: false,
                        row,
                        col,
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
                });
        }
    }
}

fn find_neighbor_indexes(maze_cell: &Mut<MazeCell>, offset: usize) -> [Option<usize>; 4] {
    let cols = (WINDOW_WIDTH / BLOCK_SIZE) as usize;
    let rows = (WINDOW_HEIGHT / BLOCK_SIZE) as usize;

    let find_index = |col, row| match (col, row) {
        (Some(c), Some(r)) if c < cols && r < rows => Some(c + r * cols),
        _ => None,
    };

    let neighbor_top_index = find_index(maze_cell.col.checked_add(offset), Some(maze_cell.row));
    let neighbor_bottom_index = find_index(maze_cell.col.checked_sub(offset), Some(maze_cell.row));

    let neighbor_left_index = find_index(Some(maze_cell.col), maze_cell.row.checked_sub(offset));
    let neigbor_right_index = find_index(Some(maze_cell.col), maze_cell.row.checked_add(offset));

    [
        neighbor_top_index,
        neighbor_bottom_index,
        neighbor_left_index,
        neigbor_right_index,
    ]
}

fn find_neighbors<'a>(
    cells: &'a [RefCell<(Mut<'a, MazeCell>, &'a Children)>],
    mazecell: &Mut<MazeCell>,
) -> Vec<&'a RefCell<(Mut<'a, MazeCell>, &'a Children)>> {
    let indexes = find_neighbor_indexes(mazecell, 1);
    let neighbors = indexes
        .into_iter()
        .filter_map(identity)
        .filter_map(|index| cells.get(index))
        .filter(|&cell| !cell.borrow().0.visited)
        .collect::<Vec<_>>();

    neighbors
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

pub fn generate_maze(
    mut commands: Commands,
    mut maze_cell_query: Query<(&mut MazeCell, &Children)>,
    walls_query: Query<&MazeWall>,
) {
    // pop, push, last, empty
    let mut stack = Vec::new();
    let maze_cells = maze_cell_query
        .iter_mut()
        .map(|e| RefCell::new(e))
        .collect::<Vec<_>>();

    stack.push(&maze_cells[0]);

    while let Some(current) = stack.pop() {
        let (current_cell, current_childrens) = &mut *current.borrow_mut();
        current_cell.visited = true;

        let neighbors = find_neighbors(&maze_cells, &current_cell);

        let choosen_neighbor = neighbors.choose(&mut rand::thread_rng());

        if let Some(&neighbor) = choosen_neighbor {
            stack.push(current);

            let (neighbor_cell, neighbor_childrens) = &mut *neighbor.borrow_mut();

            let current_walls =
                current_childrens
                    .into_iter()
                    .fold(HashMap::new(), |mut acc, &entity| {
                        let wall = walls_query.get(entity).unwrap().clone();
                        acc.insert(wall.direction, entity);
                        acc
                    });

            let neighbor_walls =
                neighbor_childrens
                    .into_iter()
                    .fold(HashMap::new(), |mut acc, &entity| {
                        let wall = walls_query.get(entity).unwrap().clone();
                        acc.insert(wall.direction, entity);
                        acc
                    });

            let d_col = current_cell.col as i32 - neighbor_cell.col as i32;
            let d_row = current_cell.row as i32 - neighbor_cell.row as i32;

            assert!(d_col >= -1 && d_col <= 1);
            assert!(d_row >= -1 && d_row <= 1);

            let walls_to_destroy =
                get_walls_to_destroy(d_col, d_row, current_walls, neighbor_walls);

            walls_to_destroy
                .into_iter()
                .filter_map(identity)
                .for_each(|entity| commands.entity(entity).despawn());

            neighbor_cell.visited = true;
            stack.push(&neighbor);
        }
    }
}
