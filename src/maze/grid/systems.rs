use super::{MazeCellGrid, ResetGridEvent};
use crate::maze::{
    common::{
        cell::MazeCell,
        wall::{MazeWall, WallDirection},
    },
    constants::{
        grid::{BLOCK_SIZE, CELL_COLOR, WALL_COLOR, WALL_HEIGHT},
        window::{GRID_WINDOW_HEIGHT, GRID_WINDOW_WIDTH},
    }, generation::ResetMazeCellStackEvent,
};
use bevy::prelude::*;

pub fn reset_grid(
    spawn_grid_event_reader: EventReader<ResetGridEvent>,
    mut reset_stack_event_writer: EventWriter<ResetMazeCellStackEvent>,
    mut commands: Commands,
    mut maze_grid: ResMut<MazeCellGrid>,
    maze_cell_entities: Query<Entity, With<MazeCell>>,
) {
    if spawn_grid_event_reader.is_empty() {
        return;
    }

    for entity in maze_cell_entities.iter() {
        commands.entity(entity).try_despawn_recursive();
    }

    maze_grid.clear();
    spawn_grid(commands, maze_grid);

    reset_stack_event_writer.send(ResetMazeCellStackEvent);
}

pub(crate) fn spawn_grid(mut commands: Commands, mut maze_grid: ResMut<MazeCellGrid>) {
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
                    Sprite {
                        color: CELL_COLOR,
                        custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                        ..default()
                    },
                    Transform::from_xyz(x_position, y_position, 1.0),
                    MazeCell {
                        row,
                        col,
                        visited: false,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            color: WALL_COLOR,
                            custom_size: Some(Vec2::new(BLOCK_SIZE, WALL_HEIGHT)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, half_block_size - half_wall_height, 1.0),
                        MazeWall {
                            direction: WallDirection::Top,
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            color: WALL_COLOR,
                            custom_size: Some(Vec2::new(BLOCK_SIZE, WALL_HEIGHT)),
                            ..default()
                        },
                        Transform::from_xyz(0.0, -half_block_size + half_wall_height, 1.0),
                        MazeWall {
                            direction: WallDirection::Bottom,
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            color: WALL_COLOR,
                            custom_size: Some(Vec2::new(WALL_HEIGHT, BLOCK_SIZE)),
                            ..default()
                        },
                        Transform::from_xyz(half_block_size - half_wall_height, 0.0, 1.0),
                        MazeWall {
                            direction: WallDirection::Right,
                        },
                    ));
                })
                .with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            color: WALL_COLOR,
                            custom_size: Some(Vec2::new(WALL_HEIGHT, BLOCK_SIZE)),
                            ..default()
                        },
                        Transform::from_xyz(-half_block_size + half_wall_height, 0.0, 1.0),
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
