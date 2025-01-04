use std::{fs, path::Path};

use super::*;
use crate::maze::common::wall::WallDirection;

pub fn read_maze(slot: usize) -> MazeStorage {
    let save = format!("saves/save_{}.mz", slot);
    let save_path = Path::new(&save);

    let file = fs::read_to_string(save_path).unwrap();
    let cells = file.split_terminator(";");

    let directions = cells.fold(HashMap::new(), |mut acc, curr| {
        let save_parts = curr.split_terminator(".").collect::<Vec<_>>();

        let row = save_parts[0].parse::<usize>().unwrap();
        let col = save_parts[1].parse::<usize>().unwrap();

        let directions_set = save_parts[2]
            .chars()
            .map(|v| v.to_string().parse::<WallDirection>().unwrap())
            .collect::<HashSet<_>>();

        acc.insert((row, col), directions_set);
        acc
    });

    MazeStorage(directions)
}
