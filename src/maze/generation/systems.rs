use bevy::prelude::*;

use crate::maze::window::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{
    components::{Direction, MazeCell, MazeWall, VisitedCell},
    resources::{CellGrid, MazeCellEntityStack},
    BLOCK_SIZE, WALL_HEIGHT,
};

pub fn spawn_maze_cells(mut commands: Commands, mut maze_grid: ResMut<CellGrid>) {
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
                    MazeCell { row, col },
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

pub fn stack_add_first_mazecell(
    mut commands: Commands,
    mut entity_stack: ResMut<MazeCellEntityStack>,
    entity_query: Query<Entity, With<MazeCell>>,
) {
    let entity = entity_query.into_iter().next().unwrap();
    entity_stack.push(entity);

    commands.entity(entity).insert(VisitedCell);
}
