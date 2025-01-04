use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::maze::{
    common::{
        cell::MazeCell,
        wall::{MazeWall, WallDirection},
    },
    grid::MazeCellGrid,
};
use bevy::{prelude::*, utils::HashMap};

use super::*;

#[derive(Event)]
pub struct SaveMazeEvent {
    pub slot: usize,
}

fn write_maze(writer: MazeStorage, slot: usize) {
    let save = format!("saves/save_{}.mz", slot);
    let save_path = Path::new(&save);
    let save_dir_path = save_path.parent().unwrap();

    if !save_dir_path.exists() {
        fs::create_dir_all(save_dir_path).unwrap();
    }

    let mut file = File::create(save_path).unwrap();

    let _ = file.write_all(writer.to_string().as_bytes());
}

pub fn save_maze(
    mut event_reader: EventReader<SaveMazeEvent>,
    maze_cell_children_query: Query<&Children, With<MazeCell>>,
    maze_wall_query: Query<&MazeWall>,
    maze_cell_grid: Res<MazeCellGrid>,
) {
    let all_directions = HashSet::from([
        WallDirection::Top,
        WallDirection::Left,
        WallDirection::Right,
        WallDirection::Bottom,
    ]);

    for event in event_reader.read() {
        let missing_walls = maze_cell_grid
            .iter()
            .map(|(position, &entity)| (*position, maze_cell_children_query.get(entity).unwrap()))
            .map(|(position, maze_cell_children)| {
                let maze_walls = maze_cell_children
                    .iter()
                    .filter_map(|&e| maze_wall_query.get(e).ok())
                    .map(|w| w.direction)
                    .collect::<HashSet<_>>();

                (position, maze_walls)
            })
            .map(|(position, directions)| {
                let destroyed_directions = all_directions
                    .difference(&directions)
                    .copied()
                    .collect::<HashSet<_>>();

                (position, destroyed_directions)
            })
            .collect::<HashMap<_, _>>();

        let writer = MazeStorage(missing_walls);
        write_maze(writer, event.slot);
    }
}
