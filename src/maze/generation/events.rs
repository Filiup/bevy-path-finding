use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct DestroyWallsBetween {
    pub current: Entity,
    pub neighbor: Entity,
}
