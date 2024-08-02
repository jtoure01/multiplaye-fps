mod maze;
mod player;
mod components;
mod systems;
mod levels;
mod player_spawn;
// use bevy::gltf::GltfPlugin;
use bevy::prelude::*;
use bevy::asset::ChangeWatcher;
use std::time::Duration;
mod weapon;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(2)),
            ..default()
        }))
        .insert_resource(components::MazeResource(maze::Maze::new()))
        .add_systems(Startup, (
            systems::setup,
            systems::spawn_player_with_weapon,
        ))
        .add_systems(Update, (
            systems::player_movement,
            systems::update_maze_display,
        ))
        .run();
}