use bevy::prelude::*;
use crate::player::Player;
use crate::components::{MazeResource, Wall, MazeDisplay};
use crate::maze::{Maze, WIDTH, HEIGHT};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze: Res<MazeResource>,
) {
    // Caméra
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Lumière
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Joueur
    let spawn_pos = find_spawn_position(&maze.0);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(spawn_pos),
            ..default()
        },
        Player::new(spawn_pos),
    ));

    // Murs du labyrinthe
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if maze.0.is_wall(x, y) {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
                        transform: Transform::from_xyz(x as f32, 0.5, y as f32),
                        ..default()
                    },
                    Wall,
                ));
            }
        }
    }
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(WIDTH as f32).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(WIDTH as f32 / 2.0, 0.0, HEIGHT as f32 / 2.0),
        ..default()
    });

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(5.0),
            top: Val::Px(5.0),
            ..default()
        }),
        MazeDisplay,
    ));
}
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    maze: Res<MazeResource>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut camera_transform = camera_query.single_mut();
        let rotation_speed = 2.5;
        let movement_speed = 5.0;

        if keyboard_input.pressed(KeyCode::Right) {
            let rotation = -rotation_speed * time.delta_seconds();
            camera_transform.rotate_y(rotation);
            transform.rotate_y(rotation);
        } else if keyboard_input.pressed(KeyCode::Left) {
            let rotation = rotation_speed * time.delta_seconds();
            camera_transform.rotate_y(rotation);
            transform.rotate_y(rotation);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= transform.local_z();
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += transform.local_z();
        }

        if direction != Vec3::ZERO {
            let move_delta = direction.normalize() * movement_speed * time.delta_seconds();
            let new_position = player.position + move_delta;

            let current_x = player.position.x.round() as usize;
            let current_z = player.position.z.round() as usize;
            let new_x = new_position.x.round() as usize;
            let new_z = new_position.z.round() as usize;

            if (new_x != current_x && !maze.0.is_wall(new_x, current_z)) || new_x == current_x {
                player.position.x = new_position.x;
            }
            if (new_z != current_z && !maze.0.is_wall(current_x, new_z)) || new_z == current_z {
                player.position.z = new_position.z;
            }

            transform.translation = player.position;
        }

        camera_transform.translation = player.position;
        camera_transform.look_at(player.position + transform.local_z(), Vec3::Y);
    }
}

pub fn update_maze_display(
    maze: Res<MazeResource>,
    player: Query<&Transform, With<Player>>,
    mut text_query: Query<&mut Text, With<MazeDisplay>>,
) {
    let player_pos = player.get_single().map(|t| t.translation).unwrap_or(Vec3::ZERO);
    let player_x = player_pos.x.round() as usize;
    let player_z = player_pos.z.round() as usize;

    let mut maze_text = String::new();
    
    maze_text.push_str(&"+".repeat(WIDTH + 2));
    maze_text.push('\n');

    for z in 0..HEIGHT {
        maze_text.push('|'); 
        for x in 0..WIDTH {
            let char = if x == player_x && z == player_z {
                'p' 
            } else if maze.0.is_wall(x, z) {
                '#' 
            }  else {
                ' ' 
            };
            maze_text.push_str(&char.to_string());
        }
        maze_text.push_str("|\n"); 
    }

    maze_text.push_str(&"+".repeat(WIDTH + 2));

    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = maze_text;
    }
}

fn find_spawn_position(maze: &Maze) -> Vec3 {
    for z in 0..HEIGHT {
        for x in 0..WIDTH {
            if maze.is_spawn_area(x, z) {
                return Vec3::new(x as f32, 0.5, z as f32);
            }
        }
    }
    // Position par défaut si aucune zone de spawn n'est trouvée
    Vec3::new(1.0, 0.5, 1.0)
}