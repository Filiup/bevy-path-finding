use super::*;
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

// FIFO queue resource for solving using DFS
#[derive(Resource, Default)]
pub struct CellQueue(VecDeque<Entity>);

impl Deref for CellQueue {
    type Target = VecDeque<Entity>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CellQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CellQueue {
    pub fn enqueue(&mut self, entity: Entity) {
        self.push_back(entity)
    }

    pub fn dequeue(&mut self) -> Option<Entity> {
        self.pop_front()
    }
}

pub fn init_mazecell_queue(
    mut queue: ResMut<CellQueue>,
    start_cell_entity_query: Query<Entity, With<StartCell>>,
) {
    let start_cell = start_cell_entity_query.get_single().unwrap();
    queue.enqueue(start_cell)
}

pub fn reset_mazecell_queue(mut queue: ResMut<CellQueue>) {
    queue.clear()
}
