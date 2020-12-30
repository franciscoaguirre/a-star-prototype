use bevy::prelude::*;

const GRID_HEIGHT: u32 = 10;
const GRID_WIDTH: u32 = 10;
const TILE_SIZE: f32 = 1.0;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_grid(commands, &mut meshes, &mut materials);

    let camera_translation = Vec3::new(
        GRID_WIDTH as f32 * TILE_SIZE * 0.5,
        15.0,
        GRID_HEIGHT as f32 * TILE_SIZE * 0.5,
    );

    // add entities to the world
    commands
        // cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(camera_translation)
                .looking_at(
                    Vec3::new(camera_translation.x, 0.0, camera_translation.z),
                    Vec3::unit_x()
                ),
            ..Default::default()
        });
}

fn create_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    for row in 1..=GRID_HEIGHT {
        for column in 1..=GRID_WIDTH {
            let color = if (row + column) % 2 == 0 { Color::WHITE } else { Color::BLACK };

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: TILE_SIZE })),
                    material: materials.add(color.into()),
                    transform: Transform::from_translation(Vec3::new(
                        (column - 1) as f32,
                        0.0,
                        (row - 1) as f32
                    )),
                    ..Default::default()
                });
        }
    }
}
