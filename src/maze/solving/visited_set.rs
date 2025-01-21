use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, utils::HashSet};

#[derive(Resource, Default)]
pub struct VisitedCellSet(HashSet<Entity>);

impl Deref for VisitedCellSet {
    type Target = HashSet<Entity>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VisitedCellSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn reset_visited_cell_set(mut visited_cell_set: ResMut<VisitedCellSet>) {
    visited_cell_set.clear();
}
