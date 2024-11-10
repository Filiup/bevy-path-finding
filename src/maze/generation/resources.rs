use bevy::{
    prelude::{Entity, Resource},
    time::{Timer, TimerMode}, utils::HashMap,
};

#[derive(Resource, Default)]
pub struct MazeCellEntityStack(Vec<Entity>);

#[derive(Resource, Default)]
pub struct CellGrid(HashMap<(usize, usize), Entity>);

impl MazeCellEntityStack {
    pub fn push(&mut self, value: Entity) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<Entity> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<Entity> {
        self.0.last().copied()
    }
}

impl CellGrid {
    pub fn add(&mut self, row: usize, col: usize, entity: Entity) -> Option<Entity> {
        self.0.insert((row, col), entity)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Entity> {
        self.0.get(&(row, col)).copied()
    }

}





// Grid -> Resoruce Hashmap({x, y}, Entity)
// na rychle hladanie susedov