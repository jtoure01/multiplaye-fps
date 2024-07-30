use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub position: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vec3::new(1.0, 0.5, 1.0), // Commence à (1, 1) au lieu de (0, 0)
        }
    }
}