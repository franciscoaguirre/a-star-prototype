use bevy::prelude::*;
use priority_queue::PriorityQueue;

use crate::{SelectedSquare, Square, Player};

use std::collections::HashMap;
use std::cmp::Reverse;

const EDGE_COST: i32 = 1;

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Frontier>()
            .init_resource::<CameFrom>()
            .init_resource::<CurrentCosts>()
            .add_startup_system(a_start_setup.system())
            .add_system(a_star_finder.system())
            // .add_system(print_came_from.system())
            .add_system(a_start_initializer.system());
    }
}

#[derive(Default)]
struct Frontier(PriorityQueue<(i32, i32), Reverse<i32>>);

#[derive(Default, Debug)]
struct CameFrom(HashMap<(i32, i32), Option<(i32, i32)>>);

#[derive(Default)]
struct CurrentCosts(HashMap<(i32, i32), i32>);

fn a_start_initializer(
    mut frontier: ResMut<Frontier>,
    mut came_from: ResMut<CameFrom>,
    mut current_costs: ResMut<CurrentCosts>,
    _selected_square: ChangedRes<SelectedSquare>
) {
    frontier.0.clear();
    came_from.0.clear();
    current_costs.0.clear();

    frontier.0.push((0, 0), Reverse(0));
    came_from.0.insert((0, 0), None);
    current_costs.0.insert((0, 0), 0);
}

fn a_star_finder(
    mut frontier: ResMut<Frontier>,
    mut came_from: ResMut<CameFrom>,
    mut current_costs: ResMut<CurrentCosts>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    selected_square: Res<SelectedSquare>,
) {
    if frontier.0.is_empty() {
        return;
    }

    let current = frontier.0.pop().unwrap().0;

    for mut player_transform in player_transform_query.iter_mut() {
        let (goal_x, goal_y) = (selected_square.x, selected_square.y);
        if current == (goal_x, goal_y) {
            paint_path(&mut came_from, current);
            frontier.0.clear();
            break;
        }


        // println!("Picked x: {:?}, y: {:?}, goal is: {:?}, {:?}", current.0, current.1, goal_x, goal_y);
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

fn paint_path(came_from: &mut ResMut<CameFrom>, goal: (i32, i32)) {
    let mut current_square = goal;
    let mut path = vec![];
    while current_square != (0, 0) {
        path.push(current_square);

        if came_from.0[&current_square] == None {
            println!("Did not found path");
            break;
        }

        current_square = came_from.0[&current_square].unwrap();
    }

    println!("{:?}", path);
}

fn print_came_from(came_from: ChangedRes<CameFrom>) {
    println!("{:?}", came_from);
}

fn adjacents(square: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (square.0, square.1 + 1),
        (square.0, square.1 - 1),
        (square.0 + 1, square.1),
        (square.0 - 1, square.1)
    ]
}

fn heuristic(goal: (i32, i32), next_step: (i32, i32)) -> i32 {
    (((goal.0 - next_step.0).abs() + (goal.1 - next_step.1).abs()) as f32).sqrt() as i32
}

fn a_start_setup(
    mut frontier: ResMut<Frontier>,
    mut came_from: ResMut<CameFrom>,
    mut current_costs: ResMut<CurrentCosts>
) {
    frontier.0.push((0, 0), Reverse(0));
    came_from.0.insert((0, 0), None);
    current_costs.0.insert((0, 0), 0);
}
