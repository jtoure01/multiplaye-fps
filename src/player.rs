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
    pub fn add_weapon(&mut self, weapon_entity: Entity) {
        self.weapon_entity = Some(weapon_entity);
    }
}