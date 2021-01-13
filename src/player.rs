use bevy::prelude::*;
use crate::{SelectedPath};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(move_player.system());
    }
}

pub struct Player {
    pub x: i8,
    pub y: i8,
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ..Default::default()
    }).with(Player{x: 0, y: 0});
}

fn move_player(time: Res<Time>, mut selected_path: ResMut<SelectedPath>, mut query: Query<&mut Transform, With<Player>>) {
    let mut should_pop = false;

    if let Some(next_square) = selected_path.squares.last() {
        for mut transform in query.iter_mut() {
            let direction = Vec3::new(next_square.0 as f32, 0.5, next_square.1 as f32) - transform.translation;

            if direction.length() > 0.05 {
                transform.translation += direction.normalize() * time.delta_seconds();
            } else {
                should_pop = true;
            }
        }
    }

    if should_pop {
        selected_path.squares.pop();
    }
}
