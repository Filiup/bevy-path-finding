use super::{MazeCellGrid, BLOCK_SIZE, CELL_COLOR, WALL_COLOR, WALL_HEIGHT};
use crate::maze::{
    common::{
        cell::MazeCell,
        wall::{MazeWall, WallDirection},
    },
    window::{GRID_WINDOW_HEIGHT, GRID_WINDOW_WIDTH},
};
use bevy::prelude::*;

pub fn spawn_grid(mut commands: Commands, mut maze_grid: ResMut<MazeCellGrid>) {
    let half_block_size = BLOCK_SIZE / 2.0;
    let half_wall_height = WALL_HEIGHT / 2.0;

    let rows = (GRID_WINDOW_HEIGHT / BLOCK_SIZE) as usize;
    let cols = (GRID_WINDOW_WIDTH / BLOCK_SIZE) as usize;

    for row in 0..rows {
        for col in 0..cols {
            let x_position = (col as f32) * BLOCK_SIZE + half_block_size;
            let y_position = (row as f32) * BLOCK_SIZE + half_block_size;

            let entity = commands
                .spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x_position, y_position, 0.0),

                        sprite: Sprite {
                            color: CELL_COLOR,
                            custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
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
                                color: WALL_COLOR,
                                custom_size: Some(Vec2::new(BLOCK_SIZE, WALL_HEIGHT)),
                                ..default()
                            },
                            ..default()
                        },
                        MazeWall {
                            direction: WallDirection::Top,
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
                                color: WALL_COLOR,
                                custom_size: Some(Vec2::new(BLOCK_SIZE, WALL_HEIGHT)),
                                ..default()
                            },
                            ..default()
                        },
                        MazeWall {
                            direction: WallDirection::Bottom,
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
                                color: WALL_COLOR,
                                custom_size: Some(Vec2::new(WALL_HEIGHT, BLOCK_SIZE)),
                                ..default()
                            },
                            ..default()
                        },
                        MazeWall {
                            direction: WallDirection::Right,
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
                                color: WALL_COLOR,
                                custom_size: Some(Vec2::new(WALL_HEIGHT, BLOCK_SIZE)),
                                ..default()
                            },
                            ..default()
                        },
                        MazeWall {
                            direction: WallDirection::Left,
                        },
                    ));
                })
                .id();

            maze_grid.add(row, col, entity);
        }
    }
}
