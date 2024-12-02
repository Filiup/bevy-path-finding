use crate::maze::{
    common::cell::MazeCell,
    constants::{
        generation::{CELL_STACK_COLOR, NEIGHBOR_CELL_COLOR},
        grid::CELL_COLOR,
    },
};
use bevy::prelude::*;

#[derive(Event)]
pub struct ChangeStackColor {
    pub cell: Entity,
    pub neighbor_cells: Vec<Entity>,
}

#[derive(Event)]
pub struct ResetStackColor {
    pub entity: Entity,
}

pub fn change_stack_color(
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
    mut change_color_reader: EventReader<ChangeStackColor>,
) {
    for ChangeStackColor {
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

pub fn reset_stack_color(
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
    mut reset_color_reader: EventReader<ResetStackColor>,
) {
    for ResetStackColor { entity } in reset_color_reader.read() {
        let mut current_sprite = sprite_query.get_mut(*entity).unwrap();
        current_sprite.color = CELL_COLOR;
    }
}
