use bevy::prelude::*;

use crate::maze::{
    common::cell::MazeCell,
    constants::generation::{CELL_STACK_COLOR, NEIGHBOR_CELL_COLOR},
};

#[derive(Event)]
pub struct ChangeQueueColor {
    pub cell: Entity,
    pub neighbor_cells: Vec<Entity>,
}

pub fn change_queue_color(
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
    mut change_color_reader: EventReader<ChangeQueueColor>,
) {
    for ChangeQueueColor {
        cell: current,
        neighbor_cells: neighbors,
    } in change_color_reader.read()
    {
        let mut current_sprite = sprite_query.get_mut(*current).unwrap();
        current_sprite.color = CELL_STACK_COLOR;

        for &neighbor_entity in neighbors {
            let mut neighbor_sprite = sprite_query.get_mut(neighbor_entity).unwrap();
            neighbor_sprite.color = NEIGHBOR_CELL_COLOR;
        }
    }
}
