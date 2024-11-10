use bevy::{
    prelude::{Entity, Resource},
    utils::HashMap,
};

#[derive(Resource, Default)]
pub struct EntityStack(Vec<Entity>);

#[derive(Resource, Default)]
pub struct MazeCellGrid(HashMap<(usize, usize), Entity>);

impl EntityStack {
    pub fn push(&mut self, value: Entity) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<Entity> {
        self.0.pop()
    }
}

impl MazeCellGrid {
    pub fn add(&mut self, row: usize, col: usize, entity: Entity) -> Option<Entity> {
        self.0.insert((row, col), entity)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Entity> {
        self.0.get(&(row, col)).copied()
    }
}
