use std::{collections::HashSet, fmt, fs::File, io::Write};

use crate::maze::{
    common::{
        cell::MazeCell,
        wall::{MazeWall, WallDirection},
    },
    grid::MazeCellGrid,
};
use bevy::{prelude::*, utils::HashMap};

#[derive(Event)]
pub struct SaveMazeEvent {
    pub slot: usize,
}

struct MazeWriter(HashMap<(usize, usize), HashSet<WallDirection>>);

impl fmt::Display for MazeWriter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ((row, col), directions) in &self.0 {
            let directions_str = directions.iter().map(|v| v.to_string()).collect::<String>();

            write!(f, "{}{}{};", row, col, directions_str)?;
        }
        Ok(())
    }
}

fn write_maze(writer: MazeWriter, slot: usize) {
    let mut file = File::create(format!("saves/save_{}.mz", slot)).unwrap();

    let _ = file.write_all(writer.to_string().as_bytes());
    println!("{}", writer);
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

        let writer = MazeWriter(missing_walls);
        write_maze(writer, event.slot);
    }
}
