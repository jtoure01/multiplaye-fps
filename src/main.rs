mod maze;
mod player;
mod components;
mod systems;
mod levels;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(components::MazeResource(maze::Maze::new()))
        .add_systems(Startup, systems::setup)
        .add_systems(Update, (
            systems::player_movement,
            systems::update_maze_display,
        ))
        .run();
}