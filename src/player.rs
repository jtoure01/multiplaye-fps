use bevy::prelude::*;
#[derive(Component)]
pub struct Player {
    pub position: Vec3,
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        Player { position }
    }
}