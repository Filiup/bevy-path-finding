use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default, Debug)]
pub struct PredecessorsMap(HashMap<Entity, Entity>);

impl Deref for PredecessorsMap {
    type Target = HashMap<Entity, Entity>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PredecessorsMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn reset_predecessors_map(mut predecessors_map: ResMut<PredecessorsMap>) {
    predecessors_map.clear();
}
