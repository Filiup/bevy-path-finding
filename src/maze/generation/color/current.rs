use crate::maze::generation::cell::MazeCell;
use bevy::prelude::*;

#[derive(Event)]
pub struct ChangeCurrentColor(pub Entity, pub Color);

pub fn change_current_color(
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
    mut change_color_reader: EventReader<ChangeCurrentColor>,
) {
    for ChangeCurrentColor(entity, color) in change_color_reader.read() {
        let mut sprite = sprite_query.get_mut(*entity).unwrap();
        sprite.color = *color;
    }
}
