use bevy::prelude::*;
use bevy_mod_picking::*;

pub const GRID_HEIGHT: i32 = 10;
pub const GRID_WIDTH: i32 = 10;
pub const TILE_SIZE: f32 = 1.0;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SquareMaterials>()
            .init_resource::<SelectedSquare>()
            .add_startup_system(create_grid.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}

#[derive(Default)]
pub struct SelectedSquare {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Square {
    pub x: i32,
    pub y: i32,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y) % 2 == 0
    }
}

struct SquareMaterials {
    highlight_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
}
impl FromResources for SquareMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        SquareMaterials {
            highlight_color: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
            black_color: materials.add(Color::BLACK.into()),
            white_color: materials.add(Color::WHITE.into()),
        }
    }
}

fn create_grid(
    commands: &mut Commands,
    square_materials: Res<SquareMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for row in 0..GRID_HEIGHT {
        for column in 0..GRID_WIDTH {
            let material = if (row + column) % 2 == 0 {
                square_materials.white_color.clone()
            } else {
                square_materials.black_color.clone()
            };

            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: TILE_SIZE })),
                    material,
                    transform: Transform::from_translation(Vec3::new(
                        column as f32,
                        0.,
                        row as f32,
                    )),
                    ..Default::default()
                })
                .with(Square { x: row, y: column })
                .with(PickableMesh::default());
        }
    }
}

fn color_squares(
    pick_state: Res<PickState>,
    materials: Res<SquareMaterials>,
    mut query: Query<(Entity, &Square, &mut Handle<StandardMaterial>)>,
) {
    // Get entity under the cursor, if there is one
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, mut material) in query.iter_mut() {
        // Change the material
        *material = if Some(entity) == top_entity {
            materials.highlight_color.clone()
        } else if square.is_white() {
            materials.white_color.clone()
        } else {
            materials.black_color.clone()
        };
    }
}

fn select_square(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    squares_query: Query<&Square>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }
    // Get the square under the cursor and set it as the selected
    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        // Get the actual square. This ensures it exists and is a square. Not really needed
        if let Ok(square) = squares_query.get(*square_entity) {
            // Mark it as selected
            selected_square.x = square.x;
            selected_square.y = square.y;
        }
    } else {
        // Player clicked outside the board, deselect everything
        selected_square.x = 0;
        selected_square.y = 0;
    }
}