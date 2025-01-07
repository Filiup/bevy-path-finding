use crate::maze::common::cell::MazeCell;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EntityStack(Vec<Entity>);

#[derive(Event)]
pub struct ResetMazeCellStackEvent;

impl EntityStack {
    pub fn push(&mut self, value: Entity) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<Entity> {
        self.0.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub(crate) fn init_mazecell_stack(
    mut entity_stack: ResMut<EntityStack>,
    entity_query: Query<Entity, With<MazeCell>>,
) {
    let entity = entity_query.into_iter().next().unwrap();
    entity_stack.push(entity);
}

pub fn reset_mazecell_stack(
    reset_stack_event_reader: EventReader<ResetMazeCellStackEvent>,
    entity_stack: ResMut<EntityStack>,
    entity_query: Query<Entity, With<MazeCell>>,
) {
    if reset_stack_event_reader.is_empty() {
        return;
    }

    init_mazecell_stack(entity_stack, entity_query);
}
