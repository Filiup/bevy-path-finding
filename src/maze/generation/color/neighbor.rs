use crate::maze::generation::cell::MazeCell;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct NeighborColorStack(Vec<Entity>);

impl NeighborColorStack {
    pub fn push(&mut self, entity: Entity) {
        self.0.push(entity);
    }

    pub fn pop(&mut self) -> Option<Entity> {
        self.0.pop()
    }
}

#[derive(Event)]
pub struct ChangeNeighborColor(pub Entity);

#[derive(Event)]
pub struct ResetNeighborsColor;

pub fn change_neighbor_color(
    mut change_color_reader: EventReader<ChangeNeighborColor>,
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
    mut changed_color_stack: ResMut<NeighborColorStack>,
) {
    for ChangeNeighborColor(entity) in change_color_reader.read() {
        let mut sprite = sprite_query.get_mut(*entity).unwrap();
        sprite.color = Color::srgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0);

        changed_color_stack.push(*entity);
    }
}

pub fn reset_neighbor_color(
    mut changed_color_stack: ResMut<NeighborColorStack>,
    mut reset_cell_color_reader: EventReader<ResetNeighborsColor>,
    mut sprite_query: Query<&mut Sprite, With<MazeCell>>,
) {
    for _ in reset_cell_color_reader.read() {
        while let Some(entity) = changed_color_stack.pop() {
            let mut sprite = sprite_query.get_mut(entity).unwrap();

            sprite.color = Color::WHITE;
        }
    }
}
