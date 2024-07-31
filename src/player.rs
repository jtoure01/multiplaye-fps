use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub position: Vec3,
    pub weapon_entity: Option<Entity>,
}

impl Player {
    pub fn new(position: Vec3) -> Self {
        Player { 
            position,
            weapon_entity: None,
        }
    }
}