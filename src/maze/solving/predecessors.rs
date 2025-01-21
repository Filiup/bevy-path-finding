use std::ops::{Deref, DerefMut};

use bevy::{prelude::{Entity, Resource}, utils::HashMap};

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
