use bevy::prelude::*;
use crate::maze::Maze;

#[derive(Resource)]
pub struct MazeResource(pub Maze);

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct MazeDisplay;

