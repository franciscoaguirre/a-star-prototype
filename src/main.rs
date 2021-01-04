use bevy::prelude::*;
use bevy_mod_picking::*;

mod grid;
use grid::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(GridPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
            transform: Transform::from_translation(camera_translation).looking_at(
                Vec3::new(camera_translation.x, 0.0, camera_translation.z),
                Vec3::unit_x(),
            ),
            ..Default::default()
        })
        .with(PickSource::default());
}
