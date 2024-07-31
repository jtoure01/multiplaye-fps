// Dans systems/player_spawn.rs
use bevy::prelude::*;
use crate::player::Player;

pub fn spawn_player_with_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Charger le modèle de l'arme
    let weapon_handle: Handle<Scene> = asset_server.load("models/weapon.gltf#Scene0");

    // Spawner le joueur
    let player_entity = commands.spawn((
        Player::new(Vec3::new(0.0, 1.0, 0.0)),
        TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
    )).id();

    // Spawner l'arme comme enfant du joueur
    let weapon_entity = commands.spawn(SceneBundle {
        scene: weapon_handle,
        transform: Transform::from_xyz(0.5, -0.25, -0.5)
            .with_rotation(Quat::from_rotation_y(-0.2))
            .with_scale(Vec3::splat(0.2)),
        ..default()
    }).id();

    // Attacher l'arme au joueur
    commands.entity(player_entity).push_children(&[weapon_entity]);

    // Mettre à jour la référence de l'arme dans le composant Player
    commands.entity(player_entity).insert(Player {
        position: Vec3::new(0.0, 1.0, 0.0),
        weapon_entity: Some(weapon_entity),
    });
}