use bevy::prelude::*;
use crate::player::Player;
use crate::components::{MazeResource, Wall, MazeDisplay};
use crate::maze::{WIDTH, HEIGHT};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze: Res<MazeResource>,
) {
    // Camera
 // Camera
commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::new(1.0, 0.5, 1.0), Vec3::Y),
    ..default()
});
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(1.0, 0.5, 1.0),
            ..default()
        },
        Player::new(),
    ));

    // Maze walls
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

    // Floor
  // Floor
commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(20.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    transform: Transform::from_xyz(10.0, 0.0, 10.0),
    ..default()
});

    // 2D Maze display
   // 2D Maze display
commands.spawn((
    TextBundle::from_section(
        "",
        TextStyle {
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        top: Val::Px(10.0),
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
    let (mut transform, mut player) = query.single_mut();
    let mut camera_transform = camera_query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction -= Vec3::new(0.0, 0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction += Vec3::new(0.0, 0.0, 1.0);
    }

    if direction != Vec3::ZERO {
        let speed = 5.0; // Ajustez cette valeur pour modifier la vitesse de déplacement
        let new_position = player.position + direction.normalize() * speed * time.delta_seconds();
        let new_x = new_position.x.round() as usize;
        let new_z = new_position.z.round() as usize;

        if !maze.0.is_wall(new_x, new_z) {
            player.position = new_position;
            transform.translation = player.position;
            
            // Mise à jour de la position de la caméra
            camera_transform.translation = player.position + Vec3::new(-10.0, 10.0, 10.0);
            camera_transform.look_at(player.position, Vec3::Y);
        }
    }
}

pub fn update_maze_display(
    maze: Res<MazeResource>,
    player: Query<&Transform, With<Player>>,
    mut text_query: Query<&mut Text, With<MazeDisplay>>,
) {
    let player_transform = player.single();
    let player_x = player_transform.translation.x.round() as usize;
    let player_z = player_transform.translation.z.round() as usize;

    let mut maze_text = String::new();
    for z in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == player_x && z == player_z {
                maze_text.push('P');
            } else if maze.0.is_wall(x, z) {
                maze_text.push('█');
            } else {
                maze_text.push('·');
            }
        }
        maze_text.push('\n');
    }

    let mut text = text_query.single_mut();
    text.sections[0].value = maze_text;
}