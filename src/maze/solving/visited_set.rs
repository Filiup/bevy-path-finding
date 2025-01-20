use std::ops::{Deref, DerefMut};

use bevy::{
    prelude::{Entity, Resource},
    utils::HashSet,
};

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
