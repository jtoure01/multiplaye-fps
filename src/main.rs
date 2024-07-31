mod maze;
mod player;
mod components;
mod systems;
mod levels;
mod player_spawn;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(components::MazeResource(maze::Maze::new()))
        .add_systems(Startup, (
            systems::setup,
            //player_spawn::spawn_player_with_weapon, // Ajoutez cette ligne
        ))
        .add_systems(Update, (
            systems::player_movement,
            systems::update_maze_display,
        ))
        .run();
}