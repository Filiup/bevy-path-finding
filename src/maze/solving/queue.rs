use super::*;
use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

// TODO: Solving alghorithm
/* Požiadavky:
1. Inicializacia Cell od ktoreho sa zacina a cell kde sa bude koncit - DONE
1. Vytvorenie FIFO Queue - DONE
2. Pridanie miesta začiatku ako prvý element do Queue - Done


Algoritmuz:

1. Pokiaľ queue nie je prázdna
    1. zober z nej jeden cell a označ ho ako "visited"
    2. nájdi všetkým sesedov, cez ktorých je možný prechod a neboli už navštívení ( o jedno vpred )
    3. pridaj všetkých nájdených susedov do queue */




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
