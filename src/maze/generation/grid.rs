use super::{
    cell::MazeCell,
    walls::{Direction, MazeWall},
    BLOCK_SIZE, WALL_HEIGHT,
};
use crate::maze::window::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct MazeCellGrid(HashMap<(usize, usize), Entity>);

impl MazeCellGrid {
    fn add(&mut self, row: usize, col: usize, entity: Entity) -> Option<Entity> {
        self.0.insert((row, col), entity)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Entity> {
        self.0.get(&(row, col)).copied()
    }
}

pub fn spawn_grid(mut commands: Commands, mut maze_grid: ResMut<MazeCellGrid>) {
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
