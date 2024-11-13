use super::cell::MazeCell;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ChangedColorStack(Vec<Entity>);

impl ChangedColorStack {
    pub fn push(&mut self, entity: Entity) {
        self.0.push(entity);
    }

    pub fn pop(&mut self) -> Option<Entity> {
        self.0.pop()
    }
}

#[derive(Event)]
pub struct ChangeCellColor {
    pub entity: Entity,
    pub color: Color,
}

#[derive(Event)]
pub struct ResetCellColor;

pub fn change_cell_color(
    mut change_color_reader: EventReader<ChangeCellColor>,
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
    mut changed_color_stack: ResMut<ChangedColorStack>,
) {
    for ChangeCellColor { color, entity } in change_color_reader.read() {
        let mut sprite = sprite_query.get_mut(*entity).unwrap();
        sprite.color = *color;

        changed_color_stack.push(*entity);
    }
}

pub fn reset_cell_color(
    mut changed_color_stack: ResMut<ChangedColorStack>,
    mut reset_cell_color_reader: EventReader<ResetCellColor>,
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
) {
    for _ in reset_cell_color_reader.read() {
        while let Some(entity) = changed_color_stack.pop() {
            let mut sprite = sprite_query.get_mut(entity).unwrap();

            sprite.color = Color::WHITE;
        }
    }
}
