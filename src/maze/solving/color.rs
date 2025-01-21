use bevy::prelude::*;

use crate::maze::{
    common::cell::MazeCell,
    constants::iterration::{CELL_STACK_COLOR, NEIGHBOR_CELL_COLOR},
};

use super::{EndCell, StartCell};

#[derive(Event)]
pub struct ChangeQueueColor {
    pub cell: Entity,
    pub neighbor_cells: Vec<Entity>,
}

pub fn change_queue_color(
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
    start_cell_entity_query: Query<Entity, With<StartCell>>,
    end_cell_entity_query: Query<Entity, With<EndCell>>,
    mut change_color_reader: EventReader<ChangeQueueColor>,
) {
    let start_cell_entity = start_cell_entity_query.get_single().unwrap();
    let end_cell_entity = end_cell_entity_query.get_single().unwrap();
    let can_change_color = |e| e != start_cell_entity && e != end_cell_entity;

    for ChangeQueueColor {
        cell: current,
        neighbor_cells: neighbors,
    } in change_color_reader.read()
    {
        if can_change_color(*current) {
            let mut current_sprite = sprite_query.get_mut(*current).unwrap();
            current_sprite.color = CELL_STACK_COLOR;
        }

        for &neighbor_entity in neighbors {
            if can_change_color(neighbor_entity) {
                let mut neighbor_sprite = sprite_query.get_mut(neighbor_entity).unwrap();
                neighbor_sprite.color = NEIGHBOR_CELL_COLOR;
            }
        }
    }
}
