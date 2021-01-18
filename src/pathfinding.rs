use bevy::prelude::*;
use priority_queue::PriorityQueue;

use crate::{player::Player, SelectedPath, SelectedSquare};

use std::cmp::Reverse;
use std::collections::HashMap;

const EDGE_COST: i32 = 1;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Frontier>()
            .init_resource::<CameFrom>()
            .init_resource::<CurrentCosts>()
            .add_startup_system(a_star_setup.system())
            .add_system(a_star_initializer.system());
    }
}

#[derive(Default)]
struct Frontier(PriorityQueue<(i32, i32), Reverse<i32>>);

#[derive(Default, Debug)]
struct CameFrom(HashMap<(i32, i32), Option<(i32, i32)>>);

#[derive(Default)]
struct CurrentCosts(HashMap<(i32, i32), i32>);

fn a_star_initializer(
    mut frontier: ResMut<Frontier>,
    mut came_from: ResMut<CameFrom>,
    mut current_costs: ResMut<CurrentCosts>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    mut selected_path: ResMut<SelectedPath>,
    selected_square: ChangedRes<SelectedSquare>,
) {
    for transform in player_transform_query.iter_mut() {
        frontier.0.clear();
        came_from.0.clear();
        current_costs.0.clear();

        let player_position = (
            transform.translation.x.round() as i32,
            transform.translation.z.round() as i32,
        );
        frontier.0.push(player_position, Reverse(0));
        came_from.0.insert(player_position, None);
        current_costs.0.insert(player_position, 0);

        create_path(
            &mut frontier,
            &mut came_from,
            &mut current_costs,
            &mut selected_path,
            player_position,
            &selected_square,
        )
    }
}

fn create_path(
    frontier: &mut ResMut<Frontier>,
    came_from: &mut ResMut<CameFrom>,
    current_costs: &mut ResMut<CurrentCosts>,
    selected_path: &mut ResMut<SelectedPath>,
    source: (i32, i32),
    selected_square: &ChangedRes<SelectedSquare>,
) {
    while !frontier.0.is_empty() {
        let current = frontier.0.pop().unwrap().0;

        let (goal_x, goal_y) = (selected_square.x, selected_square.y);
        if current == (goal_x, goal_y) {
            select_path(came_from, source, selected_path, current);
            frontier.0.clear();
            break;
        }

        for (x, y) in adjacents(current) {
            let new_cost = current_costs.0[&current] + EDGE_COST;

            if !current_costs.0.contains_key(&(x, y)) || new_cost < current_costs.0[&(x, y)] {
                current_costs.0.insert((x, y), new_cost);
                let priority = new_cost + heuristic((goal_x, goal_y), (x, y));
                frontier.0.push((x, y), Reverse(priority));
                came_from.0.insert((x, y), Some(current));
            }
        }
    }
}

fn select_path(
    came_from: &mut ResMut<CameFrom>,
    source: (i32, i32),
    selected_path: &mut ResMut<SelectedPath>,
    goal: (i32, i32),
) {
    let mut current_square = goal;
    selected_path.squares.clear();

    while current_square != source {
        selected_path.squares.push(current_square);

        if came_from.0[&current_square] == None {
            println!("Did not found path");
            break;
        }

        current_square = came_from.0[&current_square].unwrap();
    }
}

fn adjacents(square: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (square.0, square.1 + 1),
        (square.0, square.1 - 1),
        (square.0 + 1, square.1),
        (square.0 - 1, square.1),
    ]
}

fn heuristic(goal: (i32, i32), next_step: (i32, i32)) -> i32 {
    // (((goal.0 - next_step.0).abs() + (goal.1 - next_step.1).abs()) as f32).sqrt() as i32
    // Change heurisitc specific for grid
    (goal.0 - next_step.0).abs() + (goal.1 - next_step.1).abs()
}

fn a_star_setup(
    mut frontier: ResMut<Frontier>,
    mut came_from: ResMut<CameFrom>,
    mut current_costs: ResMut<CurrentCosts>,
) {
    frontier.0.push((0, 0), Reverse(0));
    came_from.0.insert((0, 0), None);
    current_costs.0.insert((0, 0), 0);
}
