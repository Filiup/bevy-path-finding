use super::cell::MazeCell;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EntityStack(Vec<Entity>);

impl EntityStack {
    pub fn push(&mut self, value: Entity) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<Entity> {
        self.0.pop()
    }
}

pub(crate) fn stack_add_first_mazecell(
    mut entity_stack: ResMut<EntityStack>,
    entity_query: Query<Entity, With<MazeCell>>,
) {
    let entity = entity_query.into_iter().next().unwrap();
    entity_stack.push(entity);
}
