mod systems;

use bevy::{
    prelude::*,
    utils::{hashbrown::hash_map::Iter, HashMap},
};
use systems::reset_grid;
use systems::spawn_grid;

#[derive(Event)]
pub struct ResetGridEvent;

#[derive(Resource, Default, Debug)]
pub struct MazeCellGrid(HashMap<(usize, usize), Entity>);

impl MazeCellGrid {
    pub(crate) fn add(&mut self, row: usize, col: usize, entity: Entity) -> Option<Entity> {
        self.0.insert((row, col), entity)
    }

    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Entity> {
        self.0.get(&(row, col)).copied()
    }

    pub fn iter(&self) -> Iter<(usize, usize), Entity> {
        self.0.iter()
    }
}

pub struct MazeGridPlugin;

impl Plugin for MazeGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeCellGrid>()
            .add_event::<ResetGridEvent>()
            .add_systems(PreStartup, spawn_grid)
            .add_systems(PreUpdate, reset_grid);
    }
}
