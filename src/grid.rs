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
            .init_resource::<SelectedPath>()
            .add_startup_stage("game_setup", SystemStage::serial())
            // .add_startup_system(create_grid.system())
            .add_startup_system_to_stage("game_setup", create_grid.system())
            .add_startup_system_to_stage("game_setup", calculate_startup_adjacents.system())
            .add_event::<UpdateAdjacentsEvent>()
            .add_system(color_squares.system())
            .add_system(select_square.system())
            .add_system(place_wall.system());
    }
}

#[derive(Default)]
pub struct SelectedPath {
    pub squares: Vec<(i32, i32)>,
}

#[derive(Default)]
pub struct SelectedSquare {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default)]
pub struct Square {
    pub x: i32,
    pub y: i32,
    pub obstructed: bool,
    pub unobstructed_adjacents: Vec<Entity>,
    pub adjacents: Vec<Entity>,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y) % 2 == 0
    }

    // fn adjacents(&self) -> Vec<&Square> {
    //     self.adjacents
    //         .iter()
    //         .filter(|square| !square.obstructed)
    //         .map(|square| *square)
    //         .collect()
    // }

    fn adjacents_numbers(&self) -> Vec<(i32, i32)> {
        vec![
            (self.x, self.y + 1),
            (self.x, self.y - 1),
            (self.x + 1, self.y),
            (self.x - 1, self.y),
        ]
    }
}

struct UpdateAdjacentsEvent {
    square: Entity,
}

struct SquareMaterials {
    highlight_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
    path_color_dark: Handle<StandardMaterial>,
    path_color_light: Handle<StandardMaterial>,
}
impl FromResources for SquareMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        SquareMaterials {
            highlight_color: materials.add(Color::RED.into()),
            black_color: materials.add(Color::BLACK.into()),
            white_color: materials.add(Color::WHITE.into()),
            path_color_dark: materials.add(Color::rgb(0., 0., 1.).into()),
            path_color_light: materials.add(Color::rgb(0., 0.33, 1.).into()),
        }
    }
}

fn calculate_startup_adjacents(
    mut query: Query<&mut Square>,
    // adjacent_squares: Query<(Entity, &Square)>,
) {
    println!("Calculating grid adjacency...");
    // println!("{:?}", query.iter_mut().collect::<Vec<_>>());
    for mut square in query.iter_mut() {
        // for (entity, adjacent) in adjacent_squares.iter() {
        //     if square
        //         .adjacents_numbers()
        //         .contains(&(adjacent.x, adjacent.y))
        //     {
        //         square.adjacents.push(entity);
        //     }
        // }

        println!(
            "x: {:?} y: {:?} adj: {:?}",
            square.x,
            square.y,
            0 //, square.adjacents
        );
    }
    println!("Finished adjacency");
}

fn adjust_adjacents(
    events: Res<Events<UpdateAdjacentsEvent>>,
    mut event_reader: Local<EventReader<UpdateAdjacentsEvent>>,
    query: Query<&Square>,
) {
    for event in event_reader.iter(&events) {
        let square = query.get(event.square).unwrap();

        // for adjacent in square.adjacents {}
    }
}

fn create_grid(
    commands: &mut Commands,
    square_materials: Res<SquareMaterials>,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut query: Query<&mut Square>,
    // adjacent_squares: Query<(Entity, &Square)>,
) {
    println!("Creating grid...");
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
                        row as f32,
                        0.,
                        column as f32,
                    )),
                    ..Default::default()
                })
                .with(Square {
                    x: row,
                    y: column,
                    ..Default::default()
                })
                .with(PickableMesh::default());
        }
    }
    println!("Finished grid");
    // calculate_startup_adjacents(query);//, adjacent_squares);
}

fn color_squares(
    pick_state: Res<PickState>,
    materials: Res<SquareMaterials>,
    selected_path: Res<SelectedPath>,
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
        let contained = selected_path.squares.contains(&(square.x, square.y));

        *material = if Some(entity) == top_entity {
            materials.highlight_color.clone()
        } else if contained && square.is_white() {
            materials.path_color_light.clone()
        } else if contained {
            materials.path_color_dark.clone()
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

fn place_wall(
    mouse_button_inputs: Res<Input<MouseButton>>,
    pick_state: Res<PickState>,
    commands: &mut Commands,
    mut squares_query: Query<&mut Square>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Right) {
        return;
    }

    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        if let Ok(mut square) = squares_query.get_mut(*square_entity) {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::GRAY.into()),
                transform: Transform::from_translation(Vec3::new(
                    square.x as f32,
                    0.5,
                    square.y as f32,
                )),
                ..Default::default()
            });

            square.obstructed = true;
        }
    }
}
